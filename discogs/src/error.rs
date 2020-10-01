use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Attempt to create client with invalid base url")]
    InvalidBaseUrl(#[source] url::ParseError),
    #[error("Failed to create Discogs client")]
    FaildToCreateClient(#[source] reqwest::Error),

    #[error("Failed to compose base URL")]
    CreateRequestUrl(#[source] url::ParseError),
    #[error("Failed to build GET request")]
    BuildRequest(#[source] reqwest::Error),
    #[error("Unknown error during API request")]
    UnknownAPIResponse(reqwest::StatusCode),

    #[error("Failed to get Release")]
    GetRelease(#[source] reqwest::Error),
    #[error("Release '{0}' not found")]
    ReleaseNotFound(u64),
    #[error("Failed to deserialize release")]
    ReleaseDeserialization(#[source] reqwest::Error),
}
