use discogs::Discogs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN").ok();
    let client = Discogs::new("malt", token)?;
    let release = client.get_release(249504).await?;
    dbg!(release);
    Ok(())
}
