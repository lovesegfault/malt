use discogs::Discogs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Discogs::new("malt")?;
    client.get_release(249504).await?;
    Ok(())
}
