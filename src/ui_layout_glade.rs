use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

use crate::build_ui::widgets;

pub fn create(application: &gtk::Application) -> widgets::WidgetRefs {
    let glade_src = include_str!("ui.glade");
    let builder = Builder::from_string(glade_src);

    let app_window: ApplicationWindow = builder.get_object("app_window").expect("Couldn't get app_window");

    app_window.show_all();

    widgets::WidgetRefs {
        app_window,
        flow_buffer: builder.get_object("flow_buffer").expect("Couldn't get flow_buffer"),
        manifest_buffer: builder.get_object("manifest_buffer").expect("Couldn't get manifest_buffer"),
        flow_notebook: builder.get_object("flow_notebook").expect("Couldn't get flow_notebook"),
        args_buffer: builder.get_object("args_buffer").expect("Couldn't get args_buffer"),
        stdout: builder.get_object("stdout_buffer").expect("Couldn't get stdout_buffer"),
        stderr: builder.get_object("stderr_buffer").expect("Couldn't get stderr_buffer"),
        compile_flow_menu: builder.get_object("compile_flow_menu").expect("Couldn't get compile_flow_menu"),
        run_manifest_menu: builder.get_object("run_manifest_menu").expect("Couldn't get run_manifest_menu"),
        status_message: builder.get_object("status_message").expect("Couldn't get status_message"),
    }
}