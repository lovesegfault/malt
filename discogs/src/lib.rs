use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use url::Url;

mod error;

pub use error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub duration: String, // TODO: parse this
    pub position: String,
    pub title: String,
    pub type_: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String
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
    pub resource_url: Url
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Image {
    pub height: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub resource_url: String,
    pub uri: String,
    pub width: u64,
    #[serde(skip)] // TODO: I don't know what format this should be
    uri150: (),
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
pub struct Release {
    pub artists: Vec<Artist>,
    pub artists_sort: String,
    pub community: ReleaseCommunity,
    pub companies: Vec<Label>,
    pub country: String,
    pub data_quality: String,
    pub date_added: DateTime<Utc>,
    pub date_changed: DateTime<Utc>,
    pub estimated_weight: u64,
    pub extraartists: Vec<Artist>,
    pub format_quantity: u64,
    pub formats: Vec<Format>,
    pub genres: Vec<String>,
    pub id: u64,
    pub identifiers: Vec<Identifier>,
    pub images: Vec<Image>,
    pub labels: Vec<Label>,
    pub lowest_price: f64,
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
    pub year: u64,
    #[serde(skip)]
    series: Vec<()>, // TODO: I don't know what format this should be
    #[serde(skip)]
    videos: (), // TODO: I am lazy
}

pub struct Discogs {
    pub base_url: Url,
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
        Ok(Self { base_url, client, token })
    }

    fn create_request(&self, append: String) -> Result<reqwest::Request, Error> {

        todo!()
    }

    pub async fn get_release(&self, release_id: u64) -> Result<Release, Error> {
        let request_url = self.base_url.join(&format!("releases/{}", release_id)).map_err(Error::UrlCompose)?;
        let response = self.client.get(request_url).send().await.map_err(Error::GetRelease)?;
        match response.status() {
            StatusCode::OK => response.json::<Release>().await.map_err(Error::ReleaseDeserialization),
            StatusCode::NOT_FOUND => Err(Error::ReleaseNotFound(release_id)),
            s => Err(Error::UnknownAPIResponse(s))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
