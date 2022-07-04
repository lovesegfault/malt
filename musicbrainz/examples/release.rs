use std::{error::Error, sync::Arc};

use anyhow::Context;
use musicbrainz::{countries::Country, languages::Language, scripts::Script};
use reqwest::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use tower::{Service, ServiceExt};

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
    country: Option<Country>,
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
        let api_url = url::Url::parse("https://musicbrainz.org/ws/2/")?;
        let release_url = api_url.join("release/")?;
        let lookup_url = release_url.join(id)?;
        tracing::debug!(%lookup_url);

        let req = Request::new(Method::GET, lookup_url);
        let json = svc
            .ready()
            .await
            .context("wait for client to be ready")?
            .call(req)
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
    language: Option<Language>,
    script: Option<Script>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseEvent {
    area: Option<Area>,
    // #[serde(with = "time::serde::iso8601")]
    // date: time::Date,
    date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Area {
    name: String,
    id: String,
    type_id: Option<()>, // FIXME
    iso_3166_1_codes: Vec<Country>,
    #[serde(rename = "type")]
    a_type: Option<()>,
    disambiguation: String,
    sort_name: String,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::ClientBuilder::new()
        .user_agent("musicbrainz-rs/0.0.0 (https://github.com/lovesegfault/musicbrainz-rs)")
        .default_headers(headers)
        .build()?;

    // TODO: Add retry
    let mut service = tower::ServiceBuilder::new()
        .buffer(100)
        .rate_limit(1, std::time::Duration::from_secs(1))
        .service(client)
        .map_err(Arc::<dyn Error + Send + Sync>::from);

    let reference = tokio::fs::read_to_string("./assets/releases.txt").await?;

    for id in reference.lines() {
        let release = Release::lookup(&mut service, id).await?;
        tracing::info!("Looked up release {}", release.title)
    }

    Ok(())
}
