pub mod cli;
pub mod error;
pub mod hue;

use crate::hue::new_user::UsernameResponse;

const CONFIG_NAME: &str = "user";

/// Get the user configuration data
pub fn get_user_cfg() -> UsernameResponse {
    let cfg: UsernameResponse = confy::load(CONFIG_NAME, None).unwrap();

    cfg
}

pub fn store_user_cfg(cfg: &UsernameResponse) {
    confy::store(CONFIG_NAME, None, cfg).unwrap();
}
