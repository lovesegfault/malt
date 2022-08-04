use anyhow::Context;
use musicbrainz::{Artist, Client, Entity, Mbid};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut client = Client::new()?;

    let reference = tokio::fs::read_to_string("./assets/artists.txt").await?;

    for id in reference.lines() {
        let id = Mbid::try_from(id).context("Parse MBID from artist list")?;
        let artist = Artist::lookup(&mut client, &id).await?;
        tracing::info!("Looked up release {}", artist.name)
    }

    Ok(())
}
