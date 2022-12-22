use kwhue::hue::client::Bridge;
use mdns::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bridge = Bridge::new().await;
    println!("IP: {}", bridge.ip_address);

    println!("{:#?}", bridge.config_info);

    Ok(())
}
