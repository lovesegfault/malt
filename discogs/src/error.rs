use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Attempt to create client with invalid base url")]
    InvalidBaseUrl(#[source] url::ParseError),
    #[error("Failed to create Discogs client")]
    FaildToCreateClient(#[source] reqwest::Error),
    #[error("Failed to compose URLs")]
    UrlCompose(#[source] url::ParseError),
    #[error("Failed to get Release")]
    GetRelease(#[source] reqwest::Error),
    #[error("Release '{0}' not found")]
    ReleaseNotFound(u64),
}
