pub mod area;
pub mod mbid;
pub mod release;

use std::{error::Error, sync::Arc};

use lucene_query_builder::QueryString;
use reqwest::{Method, Request, Response};
use serde::Deserialize;
use tower::{util::BoxService, Service, ServiceExt};

pub use crate::{area::Area, mbid::Mbid, release::Release};

#[derive(Debug, thiserror::Error)]
pub enum MusicBrainzError {
    #[error("Failed to create MusicBrainz client")]
    ClientCreate(#[source] reqwest::Error),
    #[error("Failed waiting for lookup service to be ready")]
    ClientReady(#[source] Arc<dyn Error + Send + Sync>),
    #[error("Failed to GET from MusicBrainz")]
    ClientGet(#[source] Arc<dyn Error + Send + Sync>),
    #[error("Failed to parse lookup url")]
    LookupParseUrl(#[source] url::ParseError),
    #[error("Failed to parse lookup response as JSON")]
    LookupParseResponse(#[source] reqwest::Error),
}

#[async_trait::async_trait]
pub trait Entity
where
    for<'de> Self: Deserialize<'de>,
    Self: Send ,
{
    const NAME: &'static str;

    #[tracing::instrument(skip(client))]
    async fn lookup(client: &mut Client, mbid: &Mbid) -> Result<Self, MusicBrainzError> {
        let lookup_url = reqwest::Url::parse(&format!(
            "https://musicbrainz.org/ws/2/{}/{}",
            Self::NAME,
            mbid
        ))
        .map_err(MusicBrainzError::LookupParseUrl)?;
        tracing::debug!(%lookup_url);

        client
            .get(lookup_url)
            .await?
            .json()
            .await
            .map_err(MusicBrainzError::LookupParseResponse)
    }

    #[tracing::instrument(skip(client))]
    #[allow(unused)]
    async fn search(
        client: &mut Client,
        query: QueryString,
        limit: usize,
        offset: usize,
    ) -> Result<Self, MusicBrainzError> {
        todo!()
    }

    #[tracing::instrument(skip(client))]
    #[allow(unused)]
    async fn browse(
        client: &mut Client,
        related: String,
        limit: usize,
        offset: usize,
    ) -> Result<Self, MusicBrainzError> {
        todo!()
    }
}

#[derive(Clone)]
struct MusicBrainzRetry(usize);

impl MusicBrainzRetry {
    fn subtract(&self) -> std::future::Ready<Self> {
        std::future::ready(Self(self.0 - 1))
    }
}

impl<E: std::fmt::Debug> tower::retry::Policy<Request, Response, E> for MusicBrainzRetry {
    type Future = std::future::Ready<Self>;

    fn retry(&self, _: &Request, result: Result<&Response, &E>) -> Option<Self::Future> {
        match result {
            Ok(response) => {
                match response.status().as_u16() {
                    // retry on timeout or throttle
                    408 | 429 => Some(self.subtract()),
                    // don't retry otherwise
                    _ => None,
                }
            }
            Err(err) => {
                tracing::warn!(?err);
                if self.0 > 0 {
                    tracing::warn!("Retrying, {} attempts remain.", self.0);
                    Some(std::future::ready(MusicBrainzRetry(self.0 - 1)))
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

pub struct Client {
    svc: BoxService<Request, Response, Arc<dyn Error + Send + Sync>>,
}

impl Client {
    pub fn new() -> Result<Self, MusicBrainzError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let client = reqwest::ClientBuilder::new()
            .user_agent("musicbrainz-rs/0.0.0 (https://github.com/lovesegfault/musicbrainz-rs)")
            .default_headers(headers)
            .build()
            .map_err(MusicBrainzError::ClientCreate)?;
        Ok(Self::from(client))
    }

    async fn get(&mut self, url: reqwest::Url) -> Result<Response, MusicBrainzError> {
        self.svc
            .ready()
            .await
            .map_err(MusicBrainzError::ClientReady)?
            .call(Request::new(Method::GET, url))
            .await
            .map_err(MusicBrainzError::ClientGet)
    }

    pub async fn lookup<E: Entity>(&mut self, mbid: &Mbid) -> Result<E, MusicBrainzError> {
        E::lookup(self, mbid).await
    }
}

impl From<reqwest::Client> for Client {
    fn from(client: reqwest::Client) -> Self {
        let svc = tower::ServiceBuilder::new()
            .buffer(100)
            .rate_limit(1, std::time::Duration::from_secs(1))
            .retry(MusicBrainzRetry(5))
            .timeout(std::time::Duration::from_secs(10))
            .service(client)
            .map_err(Arc::<dyn Error + Send + Sync>::from)
            .boxed();
        Self { svc }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
