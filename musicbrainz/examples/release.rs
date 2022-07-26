use std::error::Error;

use anyhow::Context;
use musicbrainz::{musicbrainz_service, Area};
use reqwest::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use tower::{retry::Policy, Service, ServiceExt};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Release {
    title: String,
    release_events: Option<Vec<ReleaseEvent>>,
    status: Option<ReleaseStatus>,
    cover_art_archive: CoverArtArchive,
    packaging: Option<ReleasePackaging>,
    asin: Option<String>,
    quality: ReleaseQuality,
    barcode: Option<String>,
    country: Option<String>,
    disambiguation: String,
    packaging_id: Option<String>,
    id: String,
    status_id: Option<String>,
    text_representation: ReleaseTextRepresentation,
}

impl Release {
    #[tracing::instrument(skip(svc))]
    pub async fn lookup<S>(svc: &mut S, id: &str) -> anyhow::Result<Self>
    where
        S: Service<Request, Response = Response>,
        S::Error: 'static + Send + Sync + Error,
    {
        let lookup_url = url::Url::parse("https://musicbrainz.org/ws/2/release/")?.join(id)?;
        tracing::debug!(%lookup_url);

        let json = svc
            .ready()
            .await
            .context("wait for client to be ready")?
            .call(Request::new(Method::GET, lookup_url))
            .await
            .context("GET release")?
            .text()
            .await
            .context("parse release as UTF-8")?;

        let jd = &mut serde_json::Deserializer::from_str(&json);
        let release: Release = match serde_path_to_error::deserialize(jd) {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(?e);
                tracing::error!("{}", e.path().to_string());
                tracing::error!("{}", serde_json::to_string_pretty(&json).unwrap());
                panic!("Failed to deserialize release"); // FIXME
            }
        };
        Ok(release)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CoverArtArchive {
    front: bool,
    artwork: bool,
    count: u64,
    darkened: bool,
    back: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseTextRepresentation {
    language: Option<String>,
    script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseEvent {
    area: Option<Area>,
    // #[serde(with = "time::serde::iso8601")]
    // date: time::Date,
    date: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum ReleaseStatus {
    Official,
    Promotion,
    Bootleg,
    #[serde(rename = "Pseudo-Release")]
    PseudoRelease,
    Withdrawn,
    Cancelled,
}

#[derive(Debug, Deserialize, Serialize)]
enum ReleasePackaging {
    Book,
    Box,
    #[serde(rename = "Cardboard/Paper Sleeve")]
    CardboardPaperSleeve,
    #[serde(rename = "Cardboard/Paper Sleeve")]
    CassetteCase,
    #[serde(rename = "Clamshell Case")]
    ClamshellCase,
    Digibook,
    Digipak,
    #[serde(rename = "Discbox Slider")]
    DiscboxSlider,
    Fatbox,
    #[serde(rename = "Gatefold Cover")]
    GatefoldCover,
    #[serde(rename = "Jewel Case")]
    JewelCase,
    #[serde(rename = "Keep Case")]
    KeepCase,
    Longbox,
    #[serde(rename = "Metal Tin")]
    MetalTin,
    #[serde(rename = "Plastic Sleeve")]
    PlasticSleeve,
    Slidepack,
    #[serde(rename = "Slim Jewel Case")]
    SlimJewelCase,
    #[serde(rename = "Snap Case")]
    SnapCase,
    SnapPack,
    #[serde(rename = "Super Jewel Box")]
    SuperJewelBox,
    Other,
    None,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ReleaseQuality {
    High,
    Normal,
    Low,
}

#[derive(Clone)]
struct Attempts(usize);

impl Attempts {
    fn subtract(&self) -> std::future::Ready<Self> {
        std::future::ready(Self(self.0 - 1))
    }
}

impl<E: std::fmt::Debug> Policy<Request, Response, E> for Attempts {
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
                    Some(std::future::ready(Attempts(self.0 - 1)))
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut service = musicbrainz_service()?;

    let reference = tokio::fs::read_to_string("./assets/releases.txt").await?;

    for id in reference.lines() {
        let release = Release::lookup(&mut service, id).await?;
        tracing::info!("Looked up release {}", release.title)
    }

    Ok(())
}
