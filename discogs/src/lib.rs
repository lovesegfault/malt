use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use url::Url;

mod error;

pub use error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    // TODO: parse this
    pub duration: String,
    pub position: String,
    pub title: String,
    pub type_: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub resource_url: Url,
    pub username: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReleaseRating {
    pub average: f64,
    pub count: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReleaseCommunity {
    pub contributors: Vec<User>,
    pub data_quality: String,
    pub have: u64,
    pub rating: ReleaseRating,
    pub status: String,
    pub submitter: User,
    pub want: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Format {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Label {
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: u64,
    pub name: String,
    pub resource_url: Url,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Image {
    pub height: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub resource_url: String,
    pub width: u64,

    // FIXME: These should be a Url, but if the user doesn't send a token, we receive an empty
    // string, which isn't a valid Url and I don't want to figure out how to get it to serialize
    // into Option<Url> right now
    pub uri: String,
    pub uri150: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub anv: String,
    pub id: u64,
    pub join: String,
    pub name: String,
    pub resource_url: Url,
    pub role: String,
    pub tracks: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Video {
    uri: Url,
    title: String,
    description: String,
    duration: u64,
    embed: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Release {
    pub artists: Vec<Artist>,
    pub artists_sort: String,
    pub community: ReleaseCommunity,
    pub companies: Vec<Label>,
    pub country: String,
    pub data_quality: String,
    pub date_added: DateTime<Utc>,
    pub date_changed: DateTime<Utc>,
    pub estimated_weight: Option<u64>,
    pub extraartists: Vec<Artist>,
    pub format_quantity: u64,
    pub formats: Vec<Format>,
    pub genres: Vec<String>,
    pub id: u64,
    pub identifiers: Vec<Identifier>,
    pub images: Vec<Image>,
    pub labels: Vec<Label>,
    pub lowest_price: Option<f64>,
    pub master_id: u64,
    pub master_url: Url,
    pub notes: String,
    pub num_for_sale: u64,
    pub released: String,
    pub released_formatted: String,
    pub resource_url: Url,
    pub status: String,
    pub styles: Vec<String>,
    pub thumb: Option<String>,
    pub title: String,
    pub tracklist: Vec<Track>,
    pub uri: Url,
    pub videos: Vec<Video>,
    pub year: u64,

    // FIXME: I don't know what type this should be
    #[serde(skip)]
    series: (),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MasterRelease {
    pub artists: Vec<Artist>,
    pub data_quality: String,
    pub genres: Vec<String>,
    pub id: u64,
    pub images: Vec<Image>,
    pub lowest_price: Option<f64>,
    pub main_release: u64,
    pub main_release_url: Url,
    pub most_recent_release: u64,
    pub most_recent_release_url: Url,
    pub num_for_sale: u64,
    pub resource_url: Url,
    pub styles: Vec<String>,
    pub title: String,
    pub tracklist: Vec<Track>,
    pub uri: Url,
    pub versions_url: Url,
    pub videos: Vec<Video>,
    pub year: u64,
}

pub struct Discogs {
    pub base_url: Url,
    // FIXME: We need to rate limit this to 60 requests per minute
    // c.f. https://www.discogs.com/developers#page:home,header:home-rate-limiting
    client: Client,
    /// Personal access token
    token: Option<String>,
}

impl Discogs {
    pub fn new<S: Into<String>>(user_agent: S, token: Option<String>) -> Result<Self, Error> {
        let base_url = Url::parse("https://api.discogs.com").map_err(Error::InvalidBaseUrl)?;
        let client = Client::builder()
            .user_agent(user_agent.into())
            .build()
            .map_err(Error::FaildToCreateClient)?;
        Ok(Self {
            base_url,
            client,
            token,
        })
    }

    fn create_request(&self, append: &str) -> Result<reqwest::Request, Error> {
        let request_url = self
            .base_url
            .join(append)
            .map_err(Error::CreateRequestUrl)?;
        let mut request_builder = self.client.get(request_url);
        if let Some(token) = &self.token {
            request_builder =
                request_builder.header("Authorization", format!("Discogs token={}", token));
        }
        request_builder.build().map_err(Error::BuildGETRequest)
    }

    pub async fn get_release(&self, id: u64) -> Result<Release, Error> {
        let request = self.create_request(&format!("releases/{}", id))?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| Error::GetRelease { id, source: e })?;
        match response.status() {
            StatusCode::OK => response
                .json::<Release>()
                .await
                .map_err(|e| Error::ReleaseDeserialization { id, source: e }),
            StatusCode::NOT_FOUND => Err(Error::ReleaseNotFound { id }),
            s => Err(Error::UnknownGETResponse(s)),
        }
    }

    pub async fn get_master_release(&self, id: u64) -> Result<MasterRelease, Error> {
        let request = self.create_request(&format!("masters/{}", id))?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| Error::GetMasterRelease { id, source: e })?;
        match response.status() {
            StatusCode::OK => response
                .json::<MasterRelease>()
                .await
                .map_err(|e| Error::MasterReleaseDeserialization { id, source: e }),
            StatusCode::NOT_FOUND => Err(Error::MasterReleaseNotFound { id }),
            s => Err(Error::UnknownGETResponse(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    // A living list of interesting test subjects:
    // releases
    // 11873130
    //
    // master releases
    // 38722
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
