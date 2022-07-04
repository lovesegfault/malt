use musicbrainz::{countries::Country, languages::Language, scripts::Script};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Release {
    title: String,
    release_events: Vec<ReleaseEvent>,
    status: Option<ReleaseStatus>,
    cover_art_archive: CoverArtArchive,
    packaging: Option<ReleasePackaging>,
    asin: Option<String>,
    quality: ReleaseQuality,
    barcode: String,
    country: Country,
    disambiguation: String,
    packaging_id: Option<String>,
    id: String,
    status_id: String,
    text_representation: ReleaseTextRepresentation,
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
    language: Language,
    script: Script,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseEvent {
    area: Area,
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
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let client = reqwest::ClientBuilder::new()
        .user_agent("musicbrainz-rs/0.0.0 (https://github.com/lovesegfault/musicbrainz-rs)")
        .build()?;
    let json = client
        .get("https://musicbrainz.org/ws/2/release/ffbaf5f8-f9e6-444c-8376-be9b0796b229?fmt=json")
        .send()
        .await?
        .text()
        .await?;
    tracing::debug!(json);

    let jd = &mut serde_json::Deserializer::from_str(&json);
    let release: Release = match serde_path_to_error::deserialize(jd) {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(?e);
            tracing::error!("{}", e.path().to_string());
            panic!("Faild to deserialize release");
        }
    };
    tracing::debug!(?release);

    Ok(())
}
