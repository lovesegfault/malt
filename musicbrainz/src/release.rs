use serde::{Serialize, Deserialize};
use crate::{Area, Mbid, Entity};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Release {
    pub title: String,
    pub release_events: Option<Vec<ReleaseEvent>>,
    pub status: Option<ReleaseStatus>,
    pub cover_art_archive: CoverArtArchive,
    pub packaging: Option<ReleasePackaging>,
    pub asin: Option<String>,
    pub quality: ReleaseQuality,
    pub barcode: Option<String>,
    pub country: Option<String>,
    pub disambiguation: String,
    pub packaging_id: Option<Mbid>,
    pub id: Mbid,
    pub status_id: Option<Mbid>,
    pub text_representation: ReleaseTextRepresentation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoverArtArchive {
    pub front: bool,
    pub artwork: bool,
    pub count: u64,
    pub darkened: bool,
    pub back: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseTextRepresentation {
    pub language: Option<String>,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseEvent {
    pub area: Option<Area>,
    // #[serde(with = "time::serde::iso8601")]
    // date: time::Date,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ReleaseStatus {
    Official,
    Promotion,
    Bootleg,
    #[serde(rename = "Pseudo-Release")]
    PseudoRelease,
    Withdrawn,
    Cancelled,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ReleasePackaging {
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
pub enum ReleaseQuality {
    High,
    Normal,
    Low,
}

impl<S> Entity<S> for Release
where
    S: tower::Service<reqwest::Request, Response = reqwest::Response, Error = std::sync::Arc<dyn std::error::Error + Send + Sync>> + Send,
    S::Future: Send,
{
    const NAME: &'static str = "release";
}
