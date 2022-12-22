use kwhue::hue::bridge_ipaddr;
use mdns::Error;

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ip_addr = bridge_ipaddr().await.unwrap();
    println!("Hue bridge: {}", ip_addr);

    Ok(())
}
