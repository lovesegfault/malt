use serde::{Deserialize, Serialize};

use crate::{Entity, Mbid};

/// A release group, just as the name suggests, is used to group several different releases into a
/// single logical entity. Every release belongs to one, and only one release group.
///
/// Both release groups and releases are "albums" in a general sense, but with an important
/// difference: a release is something you can buy as media such as a CD or a vinyl record, while a
/// release group embraces the overall concept of an album -- it doesn't matter how many CDs or
/// editions/versions it had.
///
/// When an artist says "We've released our new album", they're talking about a release group. When
/// their publisher says "This new album gets released next week in Japan and next month in
/// Europe", they're referring to the different releases that belong in the above mentioned release
/// group.
///
/// MusicBrainz automatically considers every release in the database to be part of a release
/// group, even if this group only contains the one release. When a brand new release is added to
/// the database, a new release group is automatically added as well.
///
/// # See Also
/// [Upstream documentation.](https://musicbrainz.org/doc/Release_Group)
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ReleaseGroup {
    /// The title of a release group is usually very similar, if not the same, as the titles of the
    /// releases contained within it.
    pub title: String,
    /// [MBID](https://musicbrainz.org/doc/MusicBrainz_Identifier)
    pub id: Mbid,
    pub first_release_date: String,
    /// The type of a release group describes what kind of release group it is.
    pub primary_type: Option<ReleaseGroupPrimaryType>,
    pub primary_type_id: Option<Mbid>,
    /// More specific release group types.
    pub secondary_types: Vec<ReleaseGroupSecondaryType>,
    pub secondary_type_ids: Vec<Mbid>,
    pub disambiguation: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReleaseGroupPrimaryType {
    /// An album, perhaps better defined as a "Long Play" (LP) release, generally consists of
    /// previously unreleased material (unless this type is combined with secondary types which
    /// change that, such as "Compilation").
    Album,
    /// A single has different definitions depending on the market it is released for.
    ///
    /// * In the US market, a single typically has one main song and possibly a handful of
    ///   additional tracks or remixes of the main track; the single is usually named after its
    ///   main song; the single is primarily released to get radio play and to promote release
    ///   sales.
    /// * The U.K. market (also Australia and Europe) is similar to the US market, however singles
    ///   are often released as a two disc set, with each disc sold separately. They also sometimes
    ///   have a longer version of the single (often combining the tracks from the two disc
    ///   version) which is very similar to the US style single, and this is referred to as a
    ///   "maxi-single". (In some cases the maxi-single is longer than the release the single comes
    ///   from!)
    /// * The Japanese market is much more single driven. The defining factor is typically the
    ///   length of the single and the price it is sold at. Up until 1995 it was common that these
    ///   singles would be released using a mini-cd format, which is basically a much smaller CD
    ///   typically 8 cm in diameter. Around 1995 the 8cm single was phased out, and the standard
    ///   12cm CD single is more common now; generally re-releases of singles from pre-1995 will be
    ///   released on the 12cm format, even if they were originally released on the 8cm format.
    ///   Japanese singles often come with karaoke ("instrumental") versions of the songs and also
    ///   have maxi-singles like the UK with remixed versions of the songs. Sometimes a maxi-single
    ///   will have more tracks than an EP but as it's all alternate versions of the same 2-3 songs
    ///   it is still classified as a single.
    ///
    /// There are other variations of the single called a "split single" where songs by two
    /// different artists are released on the one disc, typically vinyl. The term "B-Side" comes
    /// from the era when singles were released on 7 inch (or sometimes 12 inch) vinyl with a song
    /// on each side, and so side A is the track that the single is named for, and the other side -
    /// side B - would contain a bonus song, or sometimes even the same song.
    Single,
    /// An EP is a so-called "Extended Play" release and often contains the letters EP in the
    /// title. Generally an EP will be shorter than a full length release (an LP or "Long Play")
    /// and the tracks are usually exclusive to the EP, in other words the tracks don't come from a
    /// previously issued release. EP is fairly difficult to define; usually it should only be
    /// assumed that a release is an EP if the artist defines it as such.
    #[serde(rename = "EP")]
    Ep,
    /// An episodic release that was originally broadcast via radio, television, or the Internet,
    /// including podcasts.
    Broadcast,
    /// Any release that does not fit or can't decisively be placed in any of the categories above.
    Other
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReleaseGroupSecondaryType {
    /// An audio drama is an audio-only performance of a play (often, but not always, meant for
    /// radio). Unlike audiobooks, it usually has multiple performers rather than a main narrator.
    #[serde(rename = "Audio drama")]
    AudioDrama,
    /// An audiobook is a book read by a narrator without music.
    Audiobook,
    /// A compilation, for the purposes of the MusicBrainz database, covers the following types of
    /// releases:
    ///
    /// * a collection of recordings from various old sources (not necessarily released) combined
    ///   together. For example a "best of", retrospective or rarities type release.
    /// * a various artists song collection, usually based on a general theme ("Songs for Lovers"),
    ///   a particular time period ("Hits of 1998"), or some other kind of grouping ("Songs From
    ///   the Movies", the "Café del Mar" series, etc).
    ///
    /// The MusicBrainz project does not generally consider the following to be compilations:
    ///
    /// * a reissue of an album, even if it includes bonus tracks.
    /// * a tribute release containing covers of music by another artist.
    /// * a classical release containing new recordings of works by a classical artist.
    /// * a split release containing new music by several artists
    ///
    /// Compilation should be used in addition to, not instead of, other types: for example, a
    /// various artists soundtrack using pre-released music should be marked as both a soundtrack
    /// and a compilation. As a general rule, always select every secondary type that applies.
    Compilation,
    Demo,
    /// A DJ-mix is a sequence of several recordings played one after the other, each one modified
    /// so that they blend together into a continuous flow of music. A DJ mix release requires that
    /// the recordings be modified in some manner, and the DJ who does this modification is usually
    /// (although not always) credited in a fairly prominent way.
    #[serde(rename = "DJ-mix")]
    DjMix,
    /// An interview release contains an interview, generally with an artist.
    Interview,
    /// A release that was recorded live.
    Live,
    /// Promotional in nature (but not necessarily free), mixtapes and street albums are often
    /// released by artists to promote new artists, or upcoming studio albums by prominent artists.
    /// They are also sometimes used to keep fans' attention between studio releases and are most
    /// common in rap & hip hop genres. They are often not sanctioned by the artist's label, may
    /// lack proper sample or song clearances and vary widely in production and recording quality.
    /// While mixtapes are generally DJ-mixed, they are distinct from commercial DJ mixes (which
    /// are usually deemed compilations) and are defined by having a significant proportion of new
    /// material, including original production or original vocals over top of other artists'
    /// instrumentals. They are distinct from demos in that they are designed for release directly
    /// to the public and fans; not to labels.
    #[serde(rename = "Mixtape/Street")]
    MixtapeStreet,
    /// A release that primarily contains remixed material.
    Remix,
    /// A soundtrack is the musical score to a movie, TV series, stage show, video game, or other
    /// medium. Video game CDs with audio tracks should be classified as soundtracks because the
    /// musical properties of the CDs are more interesting to MusicBrainz than their data
    /// properties.
    Soundtrack,
    /// Non-music spoken word releases.
    Spokenword,
}

impl Entity for ReleaseGroup {
    const NAME: &'static str = "release-group";
}
