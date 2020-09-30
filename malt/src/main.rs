use discogs::Discogs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Discogs::new("malt")?;
    let release = client.get_release(249504).await?;
    dbg!(release);
    Ok(())
}
