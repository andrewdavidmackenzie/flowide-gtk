use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Justification, ScrolledWindow, TextBuffer, Widget, WidgetExt, WindowPosition};
use gtk::prelude::*;

fn flow_graph_viewer(title: &str, notebook: &mut gtk::Notebook) -> ScrolledWindow {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    // let view = gtk::TextView::new();
    // view.set_editable(false);
    // scroll.add(&view);
    let label = gtk::Label::new(Some(title));
    notebook.append_page(&scroll, Some(&label));
    scroll
}

fn flow_json_viewer(title: &str, notebook: &mut gtk::Notebook) -> TextBuffer {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    scroll.add(&view);
    let label = gtk::Label::new(Some(title));
    notebook.append_page(&scroll, Some(&label));
    view.get_buffer().unwrap()
}

fn manifest_viewer(title: &str, notebook: &mut gtk::Notebook) -> TextBuffer {
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let view = gtk::TextView::new();
    view.set_editable(false);
    scroll.add(&view);
    let label = gtk::Label::new(Some(title));
    notebook.append_page(&scroll, Some(&label));
    view.get_buffer().unwrap()
}

pub fn create_tabs(notebook: &mut gtk::Notebook) -> (TextBuffer, TextBuffer ) {
    let _flow_graph_view = flow_graph_viewer("Flow", notebook);
    let flow_buffer = flow_json_viewer("Flow (json)", notebook);
    let manifest_buffer = manifest_viewer("Manifest", notebook);
    (flow_buffer, manifest_buffer)
}