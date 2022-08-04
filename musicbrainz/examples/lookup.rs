use std::fmt::Debug;

use clap::{Parser, ValueEnum};
use erased_serde::{serialize_trait_object, Serialize};
use musicbrainz::{Area, Client, Entity, Mbid, Release, ReleaseGroup, Artist};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

trait ErasedEntity: Serialize + Debug {}

serialize_trait_object!(ErasedEntity);

impl<E: Debug + Entity + Serialize> ErasedEntity for E {}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, EnumString, ValueEnum, EnumIter)]
#[strum(ascii_case_insensitive)]
enum EntityType {
    Area,
    Artist,
    Event,
    Genre,
    Instrument,
    Label,
    Place,
    Recording,
    Release,
    ReleaseGroup,
    Series,
    Url,
    Work,
}

impl EntityType {
    async fn lookup(
        &self,
        client: &mut Client,
        mbid: &Mbid,
    ) -> anyhow::Result<Box<dyn ErasedEntity>> {
        match self {
            EntityType::Area => Ok(Box::new(Area::lookup(client, mbid).await?)),
            EntityType::Artist => Ok(Box::new(Artist::lookup(client, mbid).await?)),
            EntityType::Release => Ok(Box::new(Release::lookup(client, mbid).await?)),
            EntityType::ReleaseGroup => Ok(Box::new(ReleaseGroup::lookup(client, mbid).await?)),
            _ => anyhow::bail!("Unimplemented entity {}", self),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    json: bool,
    #[clap(short, long, value_enum)]
    entity: Option<EntityType>,
    mbid: Mbid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let args = Args::parse();

    let mut client = Client::new()?;

    let mut entities: Vec<Box<dyn ErasedEntity>> = Vec::new();

    if let Some(etype) = args.entity {
        entities.push(etype.lookup(&mut client, &args.mbid).await?);
    } else {
        for etype in EntityType::iter() {
            match etype.lookup(&mut client, &args.mbid).await {
                Ok(entity) => entities.push(entity),
                Err(_) => tracing::warn!("no {} with MBID {}", etype, &args.mbid),
            }
        }
        if entities.is_empty() {
            tracing::error!(
                "MBID {} is not valid for any known MusicBrainz entity",
                &args.mbid
            );
            anyhow::bail!("Invalid MBID {}", &args.mbid);
        }
    }

    for entity in entities {
        if args.json {
            println!("{}", serde_json::to_string_pretty(&entity)?);
        } else {
            println!("{:#?}", entity);
        }
    }

    Ok(())
}
