use kwhue::hue::bridge::Bridge;
use mdns::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bridge = Bridge::new().await;

    // println!("{:#?}", bridge.config_info);

    bridge.new_user("Richard").await;

    Ok(())
}
