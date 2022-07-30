use anyhow::Context;
use musicbrainz::{Client, Entity, Mbid, ReleaseGroup};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut client = Client::new()?;

    let reference = tokio::fs::read_to_string("./assets/release-groups.txt").await?;

    for id in reference.lines() {
        let id = Mbid::try_from(id).context("Parse MBID from release list")?;
        let release_group = ReleaseGroup::lookup(&mut client, &id).await?;
        tracing::info!("Looked up release {}", release_group.title)
    }

    Ok(())
}
