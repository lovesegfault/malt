use anyhow::Context;
use musicbrainz::{Client, Entity, Mbid, Release};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut client = Client::new()?;

    let reference = tokio::fs::read_to_string("./assets/releases.txt").await?;

    for id in reference.lines() {
        let id = Mbid::try_from(id).context("Parse MBID from release list")?;
        let release = Release::lookup(&mut client, &id).await?;
        // alternatively
        // let b: Release = client.lookup(&id).await?;
        tracing::info!("Looked up release {}", release.title)
    }

    Ok(())
}
