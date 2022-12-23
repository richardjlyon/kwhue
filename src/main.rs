use kwhue::get_user_cfg;
use kwhue::hue::bridge::Bridge;
use mdns::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bridge = Bridge::new().await;

    // bridge.new_user().await;

    let lights = bridge.lights().await.unwrap();

    println!("{:#?}", lights);

    Ok(())
}
