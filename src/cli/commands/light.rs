// use crate::hue::{lights::LightState, Bridge};
use crate::hue::api::get_light_state::LightStateBuilder;
use crate::hue::{api::get_light_state::LightState, Bridge};
use colored::*;
use itertools::Itertools;
use tokio::time::{sleep, Duration};

/// COMMAND: List all lights
///
#[tracing::instrument(skip(bridge))]
pub async fn all(bridge: &Bridge) {
    tracing::debug!("listing all lights");
    let lights = bridge.lights().await.unwrap();

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
    let current_state = bridge.get_light_state(id).await.unwrap();
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
    let current_state = bridge.get_light_state(id).await.unwrap();
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
    let current_state = bridge.get_light_state(id).await.unwrap();
    let new_state = LightState {
        on: Some(!current_state.on.unwrap()),
        ..current_state
    };
    let endpoint = format!("lights/{}/state", id);
    bridge.put(&endpoint, &new_state).await.unwrap();
}

pub async fn boogie(bridge: &Bridge, id: &u32) {
    let mut state = LightStateBuilder::default()
        .on(true)
        .brightness(254)
        .hue(0)
        .build()
        .unwrap();

    for hue in (0..65535).step(100) {
        state.hue = Some(hue);
        match bridge.set_light_state(&6, &state).await {
            Ok(_) => println!("hue: {}", hue),
            Err(err) => println!("err: {:?}", err),
        };
        sleep(Duration::from_millis(10)).await;
    }
}
