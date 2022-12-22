use kwhue::hue::client::Bridge;
use mdns::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bridge = Bridge::new().await;
    println!("IP: {}", bridge.ip_address);

    let info = bridge.config().await.unwrap();

    println!("{:#?}", info);

    Ok(())
}
