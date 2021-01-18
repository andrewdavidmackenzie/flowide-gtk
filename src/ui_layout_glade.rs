#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

use gtk::prelude::*;
use gtk::{gdk, glib};
use gtk::{
    AboutDialog, AppChooserDialog, ApplicationWindow, Builder, Button, Dialog, Entry,
    FileChooserAction, FileChooserDialog, FontChooserDialog, RecentChooserDialog, ResponseType,
    Scale, SpinButton, Spinner, Switch, Window,
};

use std::env::args;

// fn about_clicked(button: &Button, dialog: &AboutDialog) {
//     if let Some(window) = button
//         .get_toplevel()
//         .and_then(|w| w.downcast::<Window>().ok())
//     {
//         dialog.set_transient_for(Some(&window));
//     }
//
//     // We only want to hide the dialog when it's closed and not completely destroy it
//     // as otherwise we can't show it again a second time.
//     dialog.connect_delete_event(|dialog, _| {
//         dialog.hide();
//         gtk::Inhibit(true)
//     });
//     //
//     // println!("Authors: {:?}", dialog.get_authors());
//     // println!("Artists: {:?}", dialog.get_artists());
//     // println!("Documenters: {:?}", dialog.get_documenters());
//
//     // println!(
//     //     "Major: {}, Minor: {}",
//     //     gtk::get_major_version(),
//     //     gtk::get_minor_version()
//     // );
//
//     dialog.show_all();
// }

pub fn create(application: &gtk::Application) -> widgets::WidgetRefs {
    let glade_src = include_str!("ui.glade");
    let builder = Builder::from_string(glade_src);

    // let spinner: Spinner = builder.get_object("spinner").expect("Couldn't get spinner");
    // spinner.start();

    let app_window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");

    app_window.show_all();

    widgets::WidgetRefs {
        app_window,
        main_window: builder.get_object("main_window").expect("Couldn't get main_window"),
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