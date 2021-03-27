//! The `flowide` is an IDE for `flow` programs.
use std::{env, process};
use std::sync::{Arc, Mutex};
use log::error;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Justification, MenuItem, ScrolledWindow, TextBuffer, Widget, WidgetExt, WindowPosition};
use gtk::prelude::*;
use gtk_rs_state::gtk_refs;
use lazy_static::lazy_static;

// Modules
mod ide_runtime_client;
mod menu;
mod ui_context;
mod actions;
mod options;
mod build_ui;

// For logging errors related with the UI that suggest displaying them on the UI maybe impossible
pub fn log_error(message: &str) {
    error!("UI message: {}", message);
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