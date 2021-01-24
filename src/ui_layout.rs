use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Justification, ScrolledWindow, TextBuffer, Widget, WidgetExt, WindowPosition};
use gtk::prelude::*;

use crate::menu;
use crate::build_ui::widgets;
use crate::notebook;

fn stdio() -> (ScrolledWindow, TextBuffer) {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    view.set_monospace(true);
    scroll.add(&view);
    (scroll, view.get_buffer().unwrap())
}

pub fn create(application: &Application) -> widgets::WidgetRefs {
    let app_window = ApplicationWindow::new(application);
    app_window.set_position(WindowPosition::Center);
    app_window.set_size_request(600, 400);

    // Create menu bar
    let (menu_bar, accelerator_group, compile_flow_menu, run_manifest_menu) = menu::menu_bar(&app_window);

    // Add menu accelerators
    app_window.add_accel_group(&accelerator_group);

    // Create the main window
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

    // Notebook for flow and manifest content
    let mut flow_notebook = gtk::Notebook::new();
    let (flow_buffer, manifest_buffer) = notebook::create_tabs(&mut flow_notebook);
    main_window.pack_start(&flow_notebook, true, true, 0);

    let mut notebook = gtk::Notebook::new();
    let (stdout_view, stdout_buffer) = stdio();
    let label = gtk::Label::new(Some("STDOUT"));
    notebook.append_page(&stdout_view, Some(&label));
    let (stderr_view, stderr_buffer) = stdio();
    let label = gtk::Label::new(Some("STDERR"));
    notebook.append_page(&stderr_view, Some(&label));
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

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&main_window, true, true, 0);

    app_window.add(&v_box);

    app_window.show_all();

    widgets::WidgetRefs {
        app_window,
        flow_buffer,
        manifest_buffer,
        flow_notebook,
        args_buffer,
        stdout: stdout_buffer,
        stderr: stderr_buffer,
        compile_flow_menu,
        run_manifest_menu,
        status_message,
    }
}