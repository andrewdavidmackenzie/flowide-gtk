//! The `flowide` is an IDE for `flow` programs.
use std::process;
use log::{warn, error};

use gio::prelude::*;
use gtk::Application;
use build_ui::build_ui;

mod ide_runtime_client;
mod menu;
mod ui_context;
mod actions;
mod options;
mod build_ui;
mod ui_layout;
mod toolbar;
mod notebook;

// For logging errors related with the UI that suggest displaying them on the UI maybe impossible
pub fn log_error(message: &str) {
    error!("UI error: {}", message);
}

pub fn log_warn(message: &str) {
    warn!("UI warning: {}", message);
}

fn main() {
    if gtk::init().is_ok() {
        if let Ok(application) = Application::new(Some("net.mackenzie-serres.flow.ide"), Default::default()) {
            let (url, flow_args, stdin_file) = options::parse_args();
            application.connect_activate(move |app| build_ui(app, &url, &flow_args, &stdin_file));
            process::exit(application.run(&[]));
        }
    }

    eprintln!("Failed to initialize GTK application");
    process::exit(-1);
}