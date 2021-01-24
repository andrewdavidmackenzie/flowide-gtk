use std::env;

use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gtk::{AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, FileChooserAction,
          FileChooserDialog, FileFilter, Menu, MenuBar, MenuItem, ResponseType,
          WidgetExt};
use gtk::prelude::*;

use flowclib::deserializers::deserializer_helper;
use std::path::{Path, PathBuf};

use crate::actions;

/// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn open_action<F: 'static>(window: &ApplicationWindow, open: &MenuItem, action_function: F)
    where F: Fn(String) {
    let accepted_extensions = deserializer_helper::get_accepted_extensions();

    let window_weak = window.downgrade();
    open.connect_activate(move |_| unsafe {
        let window = upgrade_weak!(window_weak);
        let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                            FileChooserAction::Open);
        dialog.add_buttons(&[
            ("Open", ResponseType::Ok),
            ("Cancel", ResponseType::Cancel)
        ]);

        dialog.set_select_multiple(false);
        let filter = FileFilter::new();
        for extension in accepted_extensions {
            filter.add_pattern(&format!("*.{}", extension));
        }
        dialog.set_filter(&filter);
        dialog.run();
        let uris = dialog.get_uris();
        dialog.destroy();

        if let Some(uri) = uris.get(0) {
            action_function(uri.to_string());
        }
    });
}

fn compile_action(compile: &MenuItem) {
    compile.connect_activate(move |_| {
        actions::compile_flow();
    });
}

// Return a Path(PathBuf) to a resource file that is part of the application
fn resource(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("resources").join(path)
}

fn about_dialog() -> AboutDialog {
    let p = AboutDialog::new();
    p.set_program_name(env!("CARGO_PKG_NAME"));
    p.set_website_label(Some("Flow website"));
    p.set_website(Some(env!("CARGO_PKG_HOMEPAGE")));
    p.set_title(&format!("About {}", env!("CARGO_PKG_NAME")));
    p.set_version(Some(env!("CARGO_PKG_VERSION")));
    p.set_comments(Some(&format!("flowclib version: {}\nflowrlib version: {}",
                                 flowclib::info::version(), flowrlib::info::version())));
    let resource_path = resource("icons/png/128x128.png");
    match Pixbuf::from_file(&resource_path) {
        Ok(image) => p.set_logo(Some(&image)),
        _ => crate::log_error(&format!("Could not load logo from resource a path '{}'",
                                       resource_path.display()))
    }

    // TODO
    //CARGO_PKG_DESCRIPTION
    //CARGO_PKG_REPOSITORY

    p.set_authors(&[env!("CARGO_PKG_AUTHORS")]);

    p
}

// Flow Menu
fn flow_menu(app_window: &ApplicationWindow, accelerator_group: &AccelGroup) -> (MenuItem, MenuItem) {
    let flow_menu = Menu::new();
    let flow = MenuItem::with_label("Flow");
    let open_flow_menu_item = MenuItem::with_label("Open");
    let compile_flow_menu_item = MenuItem::with_label("Compile");
    let quit = MenuItem::with_label("Quit");
    let window_weak = app_window.downgrade();
    quit.connect_activate(move |_| unsafe {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });
    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", accelerator_group, key, modifier, AccelFlags::VISIBLE);

    compile_flow_menu_item.set_sensitive(false);
    flow_menu.append(&open_flow_menu_item);
    flow_menu.append(&compile_flow_menu_item);
    flow_menu.append(&quit);
    flow.set_submenu(Some(&flow_menu));
    open_action(app_window, &open_flow_menu_item, actions::open_flow);
    compile_action(&compile_flow_menu_item);
    compile_flow_menu_item.set_sensitive(false);
    (flow, compile_flow_menu_item)
}

// Manifest Menu
fn manifest_menu(app_window: &ApplicationWindow, accelerator_group: &AccelGroup) -> (MenuItem, MenuItem) {
    let manifest_menu = Menu::new();
    let manifest = MenuItem::with_label("Manifest");
    let open_manifest_menu = MenuItem::with_label("Open");
    let run_manifest_menu = MenuItem::with_label("Run");
    run_manifest_menu.set_sensitive(false);
    manifest_menu.append(&open_manifest_menu);
    manifest_menu.append(&run_manifest_menu);
    manifest.set_submenu(Some(&manifest_menu));
    open_action(app_window, &open_manifest_menu, actions::open_manifest);
    let (key, modifier) = gtk::accelerator_parse("<Primary>R");
    run_manifest_menu.add_accelerator("activate", accelerator_group, key, modifier, AccelFlags::VISIBLE);
    run_manifest_menu.set_sensitive(false);
    (manifest, run_manifest_menu)
}


// Help Menu
fn help_menu(app_window: &ApplicationWindow, _accelerator_group: &AccelGroup) -> MenuItem {
    let help_menu = Menu::new();
    let help = MenuItem::with_label("Help");
    let about = MenuItem::with_label("About");
    help_menu.append(&about);
    help.set_submenu(Some(&help_menu));

    let window_weak = app_window.downgrade();
    about.connect_activate(move |_| unsafe {
        let ad = about_dialog();
        let window = upgrade_weak!(window_weak);
        ad.set_transient_for(Some(&window));
        ad.run();
        ad.destroy();
    });

    help
}

// Create a Menu bar with the submenus on it
pub fn menu_bar(app_window: &ApplicationWindow) -> (MenuBar, AccelGroup, MenuItem, MenuItem) {
    let accelerator_group = AccelGroup::new();
    let menu_bar = MenuBar::new();

    // Create and append a "Flow" menu
    let (flow_menu, compile_flow_menu_item) = flow_menu(&app_window, &accelerator_group);
    menu_bar.append(&flow_menu);

    // Create and append a "Manifest" menu
    let (manifest_menu, run_manifest_menu) = manifest_menu(&app_window, &accelerator_group);
    menu_bar.append(&manifest_menu);

    // Create and add an "Help" menu
    let help_menu = help_menu(&app_window, &accelerator_group);
    menu_bar.append(&help_menu);

    (menu_bar, accelerator_group, compile_flow_menu_item, run_manifest_menu)
}