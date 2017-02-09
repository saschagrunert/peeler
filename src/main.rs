//! Peel your network traffic

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate peel_ip;
extern crate pnet;
extern crate mowl;

#[macro_use]
mod errors;

use clap::App;
use errors::{error, PeelerResult};
use log::LogLevel;
use peel_ip::prelude::*;
use pnet::datalink::interfaces;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        error!("Main function call failed: {}", e);
        exit(1);
    }
}

fn run() -> PeelerResult<()> {
    // Load the command line arguments
    let yaml = load_yaml!("cli.yaml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Set the verbosity level
    let log_level = match matches.occurrences_of("verbose") {
        0 => LogLevel::Error,
        1 => LogLevel::Warn,
        2 => LogLevel::Info,
        3 => LogLevel::Debug,
        _ => LogLevel::Trace,
    };
    match mowl::init_with_level(log_level) {
        Err(_) => warn!("Log level already set"),
        Ok(_) => warn!("Log level set to: {}", log_level),
    }

    // Check the provided command line arguments
    let interface_name = matches.value_of("interface")
        .ok_or_else(|| error("No concrete interface selected"))?;

    // Find the network interface with the provided name
    let selected_interface = interfaces()
        .into_iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .ok_or_else(|| error("No valid interface found"))?;

    // Create the peel instance
    let mut peel = PeelIp::default();

    Ok(())
}
