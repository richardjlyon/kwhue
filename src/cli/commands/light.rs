// use crate::hue::{lights::LightState, Bridge};
use colored::*;
use itertools::Itertools;

use crate::hue::{api::lights_schemas::LightState, Bridge};

/// COMMAND: List all lights
///
#[tracing::instrument(skip(bridge))]
pub async fn all(bridge: &Bridge) {
    tracing::debug!("listing all lights");
    let lights = bridge.get_lights().await.unwrap();

    for light_id in lights.keys().sorted() {
        let name = lights[light_id].name.clone();
        let state = match lights[light_id].state.on.unwrap() {
            true => "ON".bright_yellow(),
            false => "OFF".bright_blue(),
        };
        let reachable = match lights[light_id].state.reachable.unwrap() {
            true => "OK".green(),
            false => "UNREACHABLE".red(),
        };

        println!("[{light_id:>2}] {name:<20} - {state:<3} ({reachable})");
    }
}

/// COMMAND: Turn on light with id
///
pub async fn on(bridge: &Bridge, id: &u32) {
    let current_state = bridge.get_state_for_light(id).await.unwrap();
    println!("{:#?}", current_state);

    let new_state = LightState {
        on: Some(true),
        ..current_state
    };
    let endpoint = format!("lights/{}/state", id);
    bridge.put(&endpoint, &new_state).await.unwrap();
}

/// COMMAND: Turn off light with id
///
pub async fn off(bridge: &Bridge, id: &u32) {
    let current_state = bridge.get_state_for_light(id).await.unwrap();
    let new_state = LightState {
        on: Some(false),
        ..current_state
    };
    let endpoint = format!("lights/{}/state", id);
    bridge.put(&endpoint, &new_state).await.unwrap();
}

/// COMMAND: Toggle light with id
///
pub async fn toggle(bridge: &Bridge, id: &u32) {
    let current_state = bridge.get_state_for_light(id).await.unwrap();
    let new_state = LightState {
        on: Some(!current_state.on.unwrap()),
        ..current_state
    };
    let endpoint = format!("lights/{}/state", id);
    bridge.put(&endpoint, &new_state).await.unwrap();
}
