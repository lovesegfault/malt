use reqwest::Client;
use reqwest::StatusCode;
use url::Url;

mod error;

pub use error::Error;

pub struct Discogs {
    pub base_url: Url,
    client: Client,
}

impl Discogs {
    pub fn new<S: Into<String>>(user_agent: S) -> Result<Self, Error> {
        let base_url = Url::parse("https://api.discogs.com").map_err(Error::InvalidBaseUrl)?;
        let client = Client::builder()
            .user_agent(user_agent.into())
            .build()
            .map_err(Error::FaildToCreateClient)?;
        Ok(Self { base_url, client })
    }

    pub async fn get_release(&self, release_id: u64) -> Result<(), Error> {
        let request_url = self.base_url.join(&format!("releases/{}", release_id)).map_err(Error::UrlCompose)?;
        let response = self.client.get(request_url).send().await.map_err(Error::GetRelease)?;
        match response.status() {
            StatusCode::OK => todo!(),
            StatusCode::NOT_FOUND => Err(Error::ReleaseNotFound(release_id)),
            s => todo!()
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
