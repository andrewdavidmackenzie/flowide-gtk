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
// mod cli_debug_client;//#![deny(missing_docs)]

use ui_context::UIContext;
use url::Url;

// Tabs/Pages in the notebook
const FLOW_GRAPH_PAGE: i32 = 0;
const _FLOW_JSON_PAGE: i32 = 1;
const MANIFEST_PAGE: i32 = 2;

lazy_static! {
    static ref UICONTEXT: Arc<Mutex<UIContext>> = Arc::new(Mutex::new(UIContext::new()));
}

gtk_refs!(
    pub mod widgets;   // The macro emits a new module with this name
    struct WidgetRefs; // The macro emits a struct with this name containing references to following fields
    app_window: gtk::ApplicationWindow,
    main_window: gtk::Box,
    flow_buffer: gtk::TextBuffer,
    manifest_buffer: gtk::TextBuffer,
    flow_notebook: gtk::Notebook,
    args_buffer: gtk::TextBuffer,
    stdout: gtk::TextBuffer,
    stderr: gtk::TextBuffer,
    compile_flow_menu: gtk::MenuItem,
    run_manifest_menu: gtk::MenuItem,
    status_message: gtk::Label
);

fn stdio() -> (ScrolledWindow, TextBuffer) {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    view.set_monospace(true);
    scroll.add(&view);
    (scroll, view.get_buffer().unwrap())
}

fn flow_graph_viewer() -> ScrolledWindow {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    // let view = gtk::TextView::new();
    // view.set_editable(false);
    // scroll.add(&view);
    scroll
}

fn flow_json_viewer() -> (ScrolledWindow, TextBuffer) {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    scroll.add(&view);
    (scroll, view.get_buffer().unwrap())
}

fn manifest_viewer() -> (ScrolledWindow, TextBuffer) {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    scroll.add(&view);
    (scroll, view.get_buffer().unwrap())
}

fn create_tab<P: IsA<Widget>>(notebook: &mut gtk::Notebook, title: &str, child: &P) -> u32 {
    let label = gtk::Label::new(Some(title));
    notebook.append_page(child, Some(&label))
}

// Create the main window, stacking up the Menu bar and other UI items
fn main_window(app_window: &ApplicationWindow,
               flow_args: &Vec<String>,
               compile_flow_menu: MenuItem,
               run_manifest_menu: &MenuItem) -> widgets::WidgetRefs {
    let main_window = gtk::Box::new(gtk::Orientation::Vertical, 4);
    main_window.set_border_width(3);
    main_window.set_vexpand(true);
    main_window.set_hexpand(true);

    // args bar
    let args_bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    args_bar.set_vexpand(false);
    args_bar.set_hexpand(true);
    // with a label
    let label = gtk::Label::new(Some("Args:"));
    label.set_margin_end(10);
    args_bar.pack_start(&label, false, true, 0);
    // and a text view in it
    let args_view = gtk::TextView::new();
    args_bar.pack_start(&args_view, true, true, 0);
    main_window.pack_start(&args_bar, false, true, 0);
    let args_buffer = args_view.get_buffer().unwrap();
    if !flow_args.is_empty() {
        args_buffer.set_text(&flow_args.join(" "));
    }

    // Notebook for flow and manifest content
    let mut flow_notebook = gtk::Notebook::new();
    let flow_graph_view = flow_graph_viewer();
    let _ = create_tab(&mut flow_notebook, "Flow", &flow_graph_view);
    let (flow_json_view, flow_buffer) = flow_json_viewer();
    let _ = create_tab(&mut flow_notebook, "Flow (json)", &flow_json_view);
    let (manifest_view, manifest_buffer) = manifest_viewer();
    let _ = create_tab(&mut flow_notebook, "Manifest", &manifest_view);
    main_window.pack_start(&flow_notebook, true, true, 0);

    let mut notebook = gtk::Notebook::new();
    let (stdout_view, stdout_buffer) = stdio();
    let (stderr_view, stderr_buffer) = stdio();
    let _ = create_tab(&mut notebook, "STDOUT", &stdout_view);
    let _ = create_tab(&mut notebook, "STDERR", &stderr_view);
    main_window.pack_start(&notebook, true, true, 0);

    // Status bar at the bottom
    let status_bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    status_bar.set_border_width(1);
    status_bar.set_margin_bottom(0);
    status_bar.set_margin_top(0);
    status_bar.set_vexpand(false);
    status_bar.set_hexpand(true);
    let status_message = gtk::Label::new(Some("Ready"));
    status_message.set_justify(Justification::Right);
    status_message.set_margin_top(0);
    status_message.set_margin_bottom(0);
    status_message.set_xalign(1.0);
    status_bar.pack_start(&status_message, true, true, 0);
    main_window.pack_start(&status_bar, false, true, 0);

    widgets::WidgetRefs {
        app_window: app_window.clone(),
        main_window,
        flow_buffer,
        manifest_buffer,
        flow_notebook,
        args_buffer,
        stdout: stdout_buffer,
        stderr: stderr_buffer,
        compile_flow_menu,
        run_manifest_menu: run_manifest_menu.clone(),
        status_message,
    }
}

fn run_action(run: &MenuItem, args_buffer: gtk::TextBuffer) {
    run.connect_activate(move |_| {
        let mut args: Vec<String> = vec!();
        let (start, end) = args_buffer.get_bounds();
        if let Some(arg_string) = args_buffer.get_text(&start, &end, false) {
            if !arg_string.trim().is_empty() {
                args = arg_string.split(' ').map(|s| s.to_string()).collect();
            }
        }
        actions::run_manifest(args);
    });
}

fn build_ui(application: &Application, url: &Option<Url>, flow_args: &Vec<String>, _stdin_file: &Option<String>) {
    let app_window = ApplicationWindow::new(application);
    app_window.set_title(env!("CARGO_PKG_NAME"));
    app_window.set_position(WindowPosition::Center);
    app_window.set_size_request(600, 400);

    app_window.connect_delete_event(move |_, _| {
        process::exit(1);
        // This is the recommended code but it causes an error message on exit currently
        // gtk::main_quit();
        // Inhibit(false)
    });

    // Create menu bar
    let (menu_bar, accelerator_group, compile_flow_menu, run_manifest_menu) = menu::menu_bar(&app_window);

    // Add menu accelerators
    app_window.add_accel_group(&accelerator_group);

    // Create the main window
    let widget_refs = main_window(&app_window, flow_args, compile_flow_menu, &run_manifest_menu);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&widget_refs.main_window, true, true, 0);

    app_window.add(&v_box);

    app_window.show_all();

    // wire up the run action that needs the menu item and the args widget
    run_action(&run_manifest_menu, widget_refs.args_buffer.clone());

    widgets::init_storage(widget_refs);

    // do any action prior to running application
    if let Some(ref flow_url) = url {
        actions::open_flow(flow_url.to_string());
    }
}

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