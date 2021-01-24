//! The `flowide` is an IDE for `flow` programs.
use std::process;

use gio::prelude::*;
use gtk::Application;
use log::error;

// Modules
mod ide_runtime_client;
mod menu;
mod ui_context;
mod actions;
mod options;
mod build_ui;
mod ui_layout;
// mod cli_debug_client;//#![deny(missing_docs)]
mod notebook;
mod toolbar;

// For logging errors related with the UI that suggest displaying them on the UI maybe impossible
pub fn log_error(message: &str) {
    error!("UI message: {}", message);
}

fn main() {
    if gtk::init().is_ok() {
        if let Ok(application) = Application::new(Some("net.mackenzie-serres.flow.ide"), Default::default()) {
            let (url, flow_args, stdin_file) = options::parse_args();
            application.connect_activate(move |app| build_ui::build_ui(app, &url, &flow_args, &stdin_file));
            process::exit(application.run(&[]));
        }
    }

    eprintln!("Failed to initialize GTK application");
    process::exit(-1);
}