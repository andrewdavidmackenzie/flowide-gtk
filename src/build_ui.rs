use std::process;
use std::sync::{Arc, Mutex};

use gtk::{Application, MenuItem, WidgetExt};
use gtk::prelude::*;
use gtk_rs_state::gtk_refs;
use lazy_static::lazy_static;
use url::Url;

use crate::{actions, ui_layout, log_error};
use crate::ui_context::UiContext;

// Tabs/Pages in the notebook
pub const FLOW_GRAPH_PAGE: i32 = 0;
pub const _FLOW_JSON_PAGE: i32 = 1;
pub const MANIFEST_PAGE: i32 = 2;

lazy_static! {
    pub static ref UICONTEXT: Arc<Mutex<UiContext >> = Arc::new(Mutex::new(UiContext::new()));
}

// The `gtk_refs` structure contains references to all the widgets we wish to refer to after
// initial layout
gtk_refs!(
    pub mod widgets;   // The macro emits a new module with this name
    struct WidgetRefs; // The macro emits a struct with this name containing references to following fields
    app_window: gtk::ApplicationWindow,
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

fn run_action(run: &MenuItem, args_buffer: gtk::TextBuffer) {
    run.connect_activate(move |_| {
        let mut args: Vec<String> = vec!();
        let (start, end) = args_buffer.get_bounds();
        if let Some(arg_string) = args_buffer.get_text(&start, &end, false) {
            if !arg_string.trim().is_empty() {
                args = arg_string.split(' ').map(|s| s.to_string()).collect();
            }
        }

        match UICONTEXT.try_lock() {
            Ok(ref mut context) => {
                match &context.manifest_url {
                    Some(manifest_url) => {
                        // Argument at index zero is the flow name
                        args.insert(0, context.manifest.as_ref().unwrap().get_metadata().name.clone());
                        actions::run_manifest(manifest_url.into(), args);
                    }
                    _ => UiContext::ui_error("No manifest loaded to run")
                }
            }
            _ => log_error("Could not get access to uicontext")
        }
    });
}

pub fn build_ui(application: &Application, url: &Option<Url>, flow_args: &[String], _stdin_file: &Option<String>) {
    let widget_refs = ui_layout::create(application);

    widget_refs.app_window.set_title(env!("CARGO_PKG_NAME"));

    widget_refs.app_window.connect_delete_event(move |_, _| {
        process::exit(1);
        // This is the recommended code but it causes an error message on exit currently
        // gtk::main_quit();
        // Inhibit(false)
    });

    if !flow_args.is_empty() {
        widget_refs.args_buffer.set_text(&flow_args.join(" "));
    }

    // wire up the run action that needs the menu item and the args widget
    run_action(&widget_refs.run_manifest_menu, widget_refs.args_buffer.clone());

    // do any action prior to running application
    if let Some(ref flow_url) = url {
        actions::open_flow(flow_url.to_string());
    }

    widgets::init_storage(widget_refs);
}