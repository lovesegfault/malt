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
#[derive(serde::Deserialize, serde::Serialize)]
pub struct MBID(Uuid);

impl std::fmt::Debug for MBID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}

impl std::fmt::Display for MBID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}

impl From<Uuid> for MBID {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MBID> for Uuid {
    fn from(mbid: MBID) -> Self {
        mbid.0
    }
}

impl MBID {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}
