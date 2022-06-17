use serde::{Deserialize, Serialize};
use musicbrainz::{scripts::Script, languages::Language};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Release {
    release_events: Vec<()>,
    status: Option<ReleaseStatus>,
    #[serde(skip)]
    cover_art_archive: (),
    packaging: Option<ReleasePackaging>,
    asin: Option<String>,
    quality: ReleaseQuality,
    barcode: String,
    country: Country,
    disambiguation: String,
    packaging_id: String,
    id: String,
    status_id: String,
    text_representation: ReleaseTextRepresentation,
}

#[derive(Deserialize, Serialize)]
enum Country {}

#[derive(Deserialize, Serialize)]
struct ReleaseTextRepresentation {
    language: Language,
    script: Script,
}

#[derive(Deserialize, Serialize)]
enum ReleaseStatus {
    Official,
    Promotion,
    Bootleg,
    #[serde(rename = "Pseudo-Release")]
    PseudoRelease,
    Withdrawn,
    Cancelled,
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ReleaseQuality {
    High,
    Normal,
    Low,
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
