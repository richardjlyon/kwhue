use crate::hue::bridge::Bridge;
use colored::*;
use itertools::Itertools;

/// COMMAND: List all lights
///
pub async fn all(bridge: &Bridge) {
    let lights = bridge.lights().await.unwrap();
    for light_id in lights.keys().sorted() {
        let name = lights[light_id].name.clone();
        let state = match lights[light_id].state.on {
            true => "ON".bright_yellow(),
            false => "OFF".bright_blue(),
        };
        let reachable = match lights[light_id].state.reachable {
            true => "OK".green(),
            false => "UNREACHABLE".red(),
        };

        println!("[{light_id:>2}] {name:<20} - {state:<3} ({reachable})");
    }
}
