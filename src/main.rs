use kwhue::hue::bridge::Bridge;
use mdns::Error;
use kwhue::get_user_cfg;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bridge = Bridge::new().await;

    // bridge.new_user().await;

    let cfg = get_user_cfg();
    println!("{:#?}", cfg);

    Ok(())
}
