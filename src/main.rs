//! The `flowide` is a prototype of an IDE for `flow` programs.

use std::env;
use std::sync::{Arc, Mutex};

use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gtk::{AboutDialog, AccelFlags, AccelGroup, Application, ApplicationWindow, FileChooserAction,
          FileChooserDialog, FileFilter, Menu, MenuBar, MenuItem, ResponseType, ScrolledWindow,
          TextBuffer, WidgetExt, WindowPosition, Widget};
use gtk::prelude::*;
use gtk_rs_state::gtk_refs;
use lazy_static::lazy_static;

use flowclib::deserializers::deserializer_helper;
use ui_context::UIContext;
use std::path::{Path, PathBuf};

mod ide_runtime_client;

mod ui_context;
mod actions;
// mod cli_debug_client;//#![deny(missing_docs)]

lazy_static! {
    static ref UICONTEXT: Arc<Mutex<UIContext>> = Arc::new(Mutex::new(UIContext::new()));
}

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

gtk_refs!(
    pub mod widgets;                // The macro emits a new module with this name
    struct WidgetRefs;              // The macro emits a struct with this name containing:
    app_window: gtk::ApplicationWindow,
    main_window: gtk::Box,
    flow_buffer: gtk::TextBuffer,
    manifest_buffer: gtk::TextBuffer,
    stdout: gtk::TextBuffer,
    stderr: gtk::TextBuffer,
    compile_flow_menu: gtk::MenuItem,
    run_manifest_menu: gtk::MenuItem
);

// Return a Path(PathBuf) to a resource file that is part of the application
fn resource(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
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
    if let Ok(image) = Pixbuf::from_file(resource("icons/png/128x128.png")) {
        p.set_logo(Some(&image));
    }

    // TODO
    //CARGO_PKG_DESCRIPTION
    //CARGO_PKG_REPOSITORY

    p.set_authors(&[env!("CARGO_PKG_AUTHORS")]);

    p
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

fn run_action(run: &MenuItem) {
    run.connect_activate(move |_| {
        // TODO open a dialog or read from a textview the args to pass to the flow being run
        actions::run_manifest(vec!());
    });
}

fn compile_action(compile: &MenuItem) {
    compile.connect_activate(move |_| {
        actions::compile_flow();
    });
}

// IDE Menu
fn ide_menu(app_window: &ApplicationWindow, accelerator_group: &AccelGroup) -> MenuItem {
    let ide_menu = Menu::new();
    let ide = MenuItem::with_label("IDE");
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");
    ide_menu.append(&about);
    ide_menu.append(&quit);
    ide.set_submenu(Some(&ide_menu));
    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", accelerator_group, key, modifier, AccelFlags::VISIBLE);

    let window_weak = app_window.downgrade();
    quit.connect_activate(move |_| unsafe {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });
    let window_weak = app_window.downgrade();
    about.connect_activate(move |_| unsafe {
        let ad = about_dialog();
        let window = upgrade_weak!(window_weak);
        ad.set_transient_for(Some(&window));
        ad.run();
        ad.destroy();
    });

    ide
}

// Flow Menu
fn flow_menu(app_window: &ApplicationWindow) -> (MenuItem, MenuItem) {
    let flow_menu = Menu::new();
    let flow = MenuItem::with_label("Flow");
    let open_flow_menu_item = MenuItem::with_label("Open");
    let compile_flow_menu_item = MenuItem::with_label("Compile");
    compile_flow_menu_item.set_sensitive(false);
    flow_menu.append(&open_flow_menu_item);
    flow_menu.append(&compile_flow_menu_item);
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
    run_action(&run_manifest_menu);
    let (key, modifier) = gtk::accelerator_parse("<Primary>R");
    run_manifest_menu.add_accelerator("activate", accelerator_group, key, modifier, AccelFlags::VISIBLE);
    run_manifest_menu.set_sensitive(false);
    (manifest, run_manifest_menu)
}

// Create a Menu bar with the submenus on it
fn menu_bar(app_window: &ApplicationWindow) -> (MenuBar, AccelGroup, MenuItem, MenuItem) {
    let accelerator_group = AccelGroup::new();
    let menu_bar = MenuBar::new();

    // Create and add an "IDE" menu that has a quit accelerator
    let ide_menu = ide_menu(&app_window, &accelerator_group);
    menu_bar.append(&ide_menu);

    // Create and append a "Flow" menu
    let (flow_menu, compile_flow_menu_item) = flow_menu(&app_window);
    menu_bar.append(&flow_menu);

    // Create and append a "Manifest" menu
    let (manifest_menu, run_manifest_menu) = manifest_menu(&app_window, &accelerator_group);
    menu_bar.append(&manifest_menu);

    (menu_bar, accelerator_group, compile_flow_menu_item, run_manifest_menu)
}

fn stdio() -> (ScrolledWindow, TextBuffer) {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    scroll.add(&view);
    (scroll, view.get_buffer().unwrap())
}

fn flow_viewer() -> (ScrolledWindow, TextBuffer) {
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


// TODO get args from a view
//                let (start, end) = runtime_context.args.get_bounds();
//                let arg_string = runtime_context.args.get_text(&start, &end, false).unwrap().to_string();
//                let args: Vec<String> = arg_string.split(' ').map(|s| s.to_string()).collect();
//                Response::Args(args)

fn create_tab<P: IsA<Widget>>(notebook: &mut gtk::Notebook, title: &str, child: &P) -> u32 {
    let label = gtk::Label::new(Some(title));
    notebook.append_page(child, Some(&label))
}

fn main_window(app_window: &ApplicationWindow,
               accelerator_group: AccelGroup,
               compile_flow_menu: MenuItem,
               run_manifest_menu: MenuItem) -> widgets::WidgetRefs {
    app_window.add_accel_group(&accelerator_group);

    let main_window = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_window.set_border_width(6);
    main_window.set_vexpand(true);
    main_window.set_hexpand(true);

    let (flow_view, flow_buffer) = flow_viewer();
    let (manifest_view, manifest_buffer) = manifest_viewer();
    let (stdout_view, stdout_buffer) = stdio();
    let (stderr_view, stderr_buffer) = stdio();

    main_window.pack_start(&flow_view, true, true, 0);
    main_window.pack_start(&manifest_view, true, true, 0);

    let mut notebook = gtk::Notebook::new();
    let _ = create_tab(&mut notebook, "STDOUT", &stdout_view);
    let _ = create_tab(&mut notebook, "STDERR", &stderr_view);
    main_window.pack_start(&notebook, true, true, 0);

    widgets::WidgetRefs {
        app_window: app_window.clone(),
        main_window,
        flow_buffer,
        manifest_buffer,
        stdout: stdout_buffer,
        stderr: stderr_buffer,
        compile_flow_menu,
        run_manifest_menu
    }
}

fn build_ui(application: &Application) {
    let app_window = ApplicationWindow::new(application);
    app_window.set_title(env!("CARGO_PKG_NAME"));
    app_window.set_position(WindowPosition::Center);
    app_window.set_size_request(600, 400);

    app_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let (menu_bar, accelerator_group, compile_flow_menu, run_manifest_menu) = menu_bar(&app_window);
    let widget_refs = main_window(&app_window, accelerator_group, compile_flow_menu, run_manifest_menu);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&widget_refs.main_window, true, true, 0);

    app_window.add(&v_box);

    app_window.show_all();

    widgets::init_storage(widget_refs);
}

pub fn ui_error(message: &str) {
    println!("UI message: {}", message);
}

// For logging errors related with the UI that suggest displaying them on the UI maybe impossible
pub fn log_error(message: &str) {
    println!("UI message: {}", message);
}

pub fn message(message: &str) {
    println!("UI message: {}", message);
}

fn main() -> Result<(), String> {
    if gtk::init().is_err() {
        return Err("Failed to initialize GTK.".to_string());
    }

// TODO  read log level from UI or args and log to a logging UI widget?
//    let log_level_arg = get_log_level(&document);
//    init_logging(log_level_arg);

    let application = Application::new(Some("net.mackenzie-serres.flow.ide"), Default::default())
        .map_err(|_| "failed to initialize GTK application")?;

    application.connect_activate(move |app| build_ui(app));

    application.run(&std::env::args().collect::<Vec<_>>());

    Ok(())
}