use serde::{Deserialize, Serialize};

use crate::{Area, Entity, Mbid};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[serde(rename = "artist", rename_all = "kebab-case")]
pub struct Artist {
    pub id: Mbid,
    pub name: String,
    pub sort_name: String,
    pub life_span: ArtistLifeSpan,
    pub disambiguation: String,

    #[serde(rename = "type")]
    pub a_type: Option<ArtistType>,
    pub type_id: Option<Mbid>,

    // #[serde(skip)]
    // ipi: (),
    // #[serde(rename = "ipi-list")]
    // pub ipis: Vec<String>, // FIXME: Create an IPI crate https://en.wikipedia.org/wiki/Interested_Parties_Information
    // pub isnis: Vec<String>, // FIXME:Create a ISNI crate https://en.wikipedia.org/wiki/International_Standard_Name_Identifier

    // pub gender: Option<ArtistGender>,
    // #[serde(rename = "gender-id")]
    // pub gender_id: Option<Mbid>,

    pub area: Option<ArtistArea>,

    // pub begin_area: Option<ArtistArea>,
    // pub end_area: Option<ArtistArea>,
    // pub country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
#[serde(rename_all = "kebab-case")]
pub struct ArtistArea {
    pub id: Mbid,
    pub name: String,
    #[serde(rename = "iso-3166-1-code-list")]
    pub iso_3166_1_codes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum ArtistGender {
    Male,
    Female,
    #[serde(rename = "Not applicable")]
    NotApplicable,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub enum ArtistType {
    Character,
    Group,
    Person,
    Other,
    Orchestra,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(feature = "strict", serde(deny_unknown_fields))]
pub struct ArtistLifeSpan {
    pub begin: Option<String>,
    pub ended: bool,
    pub end: Option<String>,
}

impl crate::private::Sealed for Artist {}

impl Entity for Artist {
    const NAME: &'static str = "artist";
}
