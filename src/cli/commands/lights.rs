use crate::hue::{bridge::Bridge, lights::LightState};
use colored::*;
use itertools::Itertools;
use tracing::info;

/// COMMAND: List all lights
///
#[tracing::instrument(fields(ip = %bridge.ip_address), skip(bridge))]
pub async fn all(bridge: &Bridge) {
    tracing::debug!("listing all lights");
    let lights = bridge.lights().await.unwrap();
    // ALEXANDER Sort every time, or on the structure some how?
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

/// COMMAND: Toggle light with id
///
pub async fn toggle(bridge: &Bridge, id: &u32) {
    info!(id, "Toggling");

    // get current state of light with id
    let current_state = bridge.get_state(id).await.unwrap();
    println!("{:#?}", current_state);

    let new_state = LightState {
        on: !current_state.on,
        ..current_state
    };
    println!("{:#?}", new_state);

    // set new state
}
