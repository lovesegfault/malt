use discogs::Discogs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN").ok();
    let client = Discogs::new("malt", token)?;

    let release = client.get_release(11873130).await.unwrap();
    dbg!(release);

    let master_release = client.get_master_release(38722).await.unwrap();
    dbg!(master_release);
    Ok(())
}
