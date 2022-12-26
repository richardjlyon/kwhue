//! Library for Hue lights.
//!
//! This crate provides a library andcommand line application for contolling hue
//! lights.
//!
//! # Example
//! ```
//! use kwhue::hue::bridge::Bridge;
//!
//! async fn main() -> Result<(), AppError> {
//!   let bridge = Bridge::new().await;
//!
//!   Ok(())
//! }
//!
pub mod cli;
pub mod config;
pub mod error;
pub mod hue;
