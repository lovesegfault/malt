pub mod area;
pub mod mbid;
pub mod release;

use std::{error::Error, sync::Arc};

use lucene_query_builder::QueryString;
use reqwest::{Method, Request, Response};
use serde::Deserialize;
use tower::{Service, ServiceExt};

pub use crate::{area::Area, mbid::Mbid, release::Release};

#[derive(Debug, thiserror::Error)]
pub enum MusicBrainzError {
    #[error("Failed to parse lookup url")]
    ParseLookupUrl(#[source] url::ParseError),
    #[error("Failed waiting for lookup service to be ready")]
    LookupServiceReady(#[source] Arc<dyn Error + Send + Sync>),
    #[error("Lookup request failed")]
    LookupRequest(#[source] Arc<dyn Error + Send + Sync>),
    #[error("Parsing lookup response failed")]
    LookupParse(#[source] reqwest::Error),
    #[error("Failed to create MusicBrainz client")]
    CreateClient(#[source] reqwest::Error),
}

#[async_trait::async_trait]
pub trait Entity<S>
where
    for<'de> Self: Deserialize<'de>,
    Self: Sized,
    S: Service<Request, Response = Response, Error = Arc<dyn Error + Send + Sync>> + Send,
    S::Future: Send,
{
    const NAME: &'static str;

    #[tracing::instrument(skip(svc))]
    async fn lookup(svc: &mut S, mbid: &Mbid) -> Result<Self, MusicBrainzError> {
        let lookup_url = url::Url::parse(&format!(
            "https://musicbrainz.org/ws/2/{}/{}",
            Self::NAME,
            mbid
        ))
        .map_err(MusicBrainzError::ParseLookupUrl)?;
        tracing::debug!(%lookup_url);

        svc.ready()
            .await
            .map_err(MusicBrainzError::LookupServiceReady)?
            .call(Request::new(Method::GET, lookup_url))
            .await
            .map_err(MusicBrainzError::LookupRequest)?
            .json()
            .await
            .map_err(MusicBrainzError::LookupParse)
    }

    #[tracing::instrument(skip(svc))]
    #[allow(unused)]
    async fn search(
        svc: &mut S,
        query: QueryString,
        limit: usize,
        offset: usize,
    ) -> Result<Self, MusicBrainzError> {
        todo!()
    }

    #[tracing::instrument(skip(svc))]
    #[allow(unused)]
    async fn browse(
        svc: &mut S,
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

// FIXME: I would like to have a `Client` type instead of this, but I don't know how to wrap a
// Tower service in another struct.
pub fn musicbrainz_service() -> Result<
    impl Service<Request, Response = Response, Error = Arc<dyn Error + Send + Sync>, Future = impl Send>,
    MusicBrainzError,
> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::ClientBuilder::new()
        .user_agent("musicbrainz-rs/0.0.0 (https://github.com/lovesegfault/musicbrainz-rs)")
        .default_headers(headers)
        .build()
        .map_err(MusicBrainzError::CreateClient)?;

    Ok(tower::ServiceBuilder::new()
        .buffer(100)
        .rate_limit(1, std::time::Duration::from_secs(1))
        .retry(MusicBrainzRetry(5))
        .timeout(std::time::Duration::from_secs(10))
        .service(client)
        .map_err(Arc::<dyn Error + Send + Sync>::from))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
