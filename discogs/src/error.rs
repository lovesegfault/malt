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
    BuildGETRequest(#[source] reqwest::Error),
    #[error("Unknown response to GET request")]
    UnknownGETResponse(reqwest::StatusCode),

    #[error("Failed to get release '{id}'")]
    GetRelease {
        id: u64,
        #[source]
        source: reqwest::Error,
    },
    #[error("Release '{id}' not found")]
    ReleaseNotFound { id: u64 },
    #[error("Failed to deserialize release '{id}'")]
    ReleaseDeserialization {
        id: u64,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to get master release '{id}'")]
    GetMasterRelease {
        id: u64,
        #[source]
        source: reqwest::Error,
    },
    #[error("Master release '{id}' not found")]
    MasterReleaseNotFound { id: u64 },
    #[error("Failed to deserialize master release '{id}'")]
    MasterReleaseDeserialization {
        id: u64,
        #[source]
        source: reqwest::Error,
    },
}
