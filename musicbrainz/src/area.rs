use serde::{Deserialize, Serialize};

use crate::{Entity, Mbid};

/// Areas are geographic regions or settlements.
///
/// # See Also
/// [Upstream documentation.](https://musicbrainz.org/doc/Area)
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Area {
    /// The name of the area
    pub name: String,
    pub sort_name: String,
    /// [MBID](https://musicbrainz.org/doc/MusicBrainz_Identifier)
    pub id: Mbid,
    #[serde(rename = "type")]
    /// The type of area, such as country, city, etc.
    pub a_type: Option<AreaType>,
    pub type_id: Option<Mbid>,
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) codes for the area
    #[serde(default)]
    pub iso_3166_1_codes: Vec<String>,
    pub disambiguation: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AreaType {
    Country,
    Subdivision,
    County,
    Municipality,
    City,
    District,
    Island
}

impl crate::private::Sealed for Area {}

impl Entity for Area {
    const NAME: &'static str = "area";
}
