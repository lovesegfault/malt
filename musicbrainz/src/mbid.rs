use uuid::Uuid;

/// One of MusicBrainz' aims is to be the universal lingua franca for music by providing a reliable
/// and unambiguous form of music identification; this music identification is performed through
/// the use of MusicBrainz Identifiers (MBIDs).
///
/// In a nutshell, an MBID is a 36 character [Universally Unique
/// Identifier](https://en.wikipedia.org/wiki/Universally_Unique_Identifier) that is permanently
/// assigned to each entity in the database, i.e. artists, release groups, releases, recordings,
/// works, labels, areas, places and URLs. MBIDs are also assigned to Tracks, though tracks do not
/// share many other properties of entities. For example, the artist Queen has an artist MBID of
/// `0383dadf-2a4e-4d10-a46a-e9e041da8eb3`, and their song Bohemian Rhapsody has a recording MBID of
/// `b1a9c0e9-d987-4042-ae91-78d6a3267d69`.
///
/// An entity can have more than one MBID. When an entity is merged into another, its MBIDs
/// redirect to the other entity.
///
/// # See Also
/// [Upstream documentation](https://musicbrainz.org/doc/MusicBrainz_Identifier).
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Mbid(Uuid);

impl std::fmt::Debug for Mbid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}

impl std::fmt::Display for Mbid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}

impl From<Uuid> for Mbid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Mbid> for Uuid {
    fn from(mbid: Mbid) -> Self {
        mbid.0
    }
}

impl<'a> TryFrom<&'a str> for Mbid {
    type Error = <Uuid as TryFrom<&'a str>>::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Uuid::try_from(value).map(Mbid)
    }
}

impl<'a> TryFrom<&'a String> for Mbid {
    type Error = <Mbid as TryFrom<&'a str>>::Error;
    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Mbid::try_from(value.as_str())
    }
}

impl Mbid {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use regex::Regex;

    proptest! {
        #[test]
        fn to_and_from_uuid(raw: [u8; 16]) {
            let uuid = Uuid::from_slice(&raw).unwrap();
            let mbid = Mbid::from(uuid);
            assert_eq!(mbid.as_uuid(), &uuid)
        }

        #[test]
        fn from_string(s in "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}") {
            let _ = Mbid::try_from(&s).unwrap();
        }

        #[test]
        fn to_string(raw: [u8; 16]) {
            lazy_static::lazy_static! {
                static ref RE: Regex  = Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap();
            }

            let mbid = Mbid(Uuid::from_slice(&raw).unwrap());
            let s = mbid.to_string();
            assert!(RE.is_match(&s))
        }

        #[test]
        fn to_and_from_string(raw: [u8; 16]) {
            let mbid = Mbid::from(Uuid::from_slice(&raw).unwrap());
            let s = mbid.to_string();
            let new = Mbid::try_from(&s).unwrap();
            assert_eq!(mbid, new);
        }
    }
}
