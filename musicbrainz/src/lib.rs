pub mod area;
pub mod countries;
pub mod languages;
pub mod mbid;
pub mod scripts;

use std::{error::Error, sync::Arc};

use derive_builder::Builder;
use heck::ToKebabCase;
use lucene_query_builder::QueryString;
use reqwest::{Method, Request, Response};
use serde::Deserialize;
use tower::{Service, ServiceExt};

pub use crate::{area::Area, countries::Country, languages::Language, mbid::Mbid, scripts::Script};

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
}

#[derive(Debug, strum::Display, PartialEq, Eq)]
pub enum EntityType {
    Area,
    Artist,
    Event,
    Genre,
    Instrument,
    Label,
    Place,
    Recording,
    Release,
    #[strum(serialize = "Release Group")]
    ReleaseGroup,
    Series,
    Url,
    Work,
}

#[async_trait::async_trait]
pub trait Entity<S>
where
    for<'de> Self: Deserialize<'de>,
    Self: Sized,
    S: Service<Request, Response = Response, Error = Arc<dyn Error + Send + Sync>> + Send,
    S::Future: Send,
{
    const TYPE: EntityType;

    #[tracing::instrument(skip(svc))]
    async fn lookup(svc: &mut S, mbid: &Mbid) -> Result<Self, MusicBrainzError> {
        let lookup_url = url::Url::parse(&format!(
            "https://musicbrainz.org/ws/2/{}/{}",
            Self::TYPE.to_string().to_kebab_case(),
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
        related: EntityType,
        limit: usize,
        offset: usize,
    ) -> Result<Self, MusicBrainzError> {
        todo!()
    }
}

#[derive(Builder, Debug)]
pub struct Client<S> {
    svc: S,
    headers: reqwest::header::HeaderMap,
    user_agent: String,
}

impl<S> Client<S> where S: Service<Request> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
