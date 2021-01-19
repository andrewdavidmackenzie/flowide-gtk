use std::process;
use std::sync::{Arc, Mutex};

use gtk::{Application, MenuItem, WidgetExt};
use gtk::prelude::*;
use gtk_rs_state::gtk_refs;
use lazy_static::lazy_static;
use url::Url;

use crate::{actions, ui_layout, ui_layout_glade};
use crate::ui_context::UIContext;

// Tabs/Pages in the notebook
pub const FLOW_GRAPH_PAGE: i32 = 0;
pub const _FLOW_JSON_PAGE: i32 = 1;
pub const MANIFEST_PAGE: i32 = 2;

lazy_static! {
    pub static ref UICONTEXT: Arc<Mutex<UIContext>> = Arc::new(Mutex::new(UIContext::new()));
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
        actions::run_manifest(args);
    });
}

pub fn build_ui(application: &Application, url: &Option<Url>, flow_args: &Vec<String>, _stdin_file: &Option<String>) {
    let widget_refs = ui_layout_glade::create(application);

    ////// Actions
    widget_refs.app_window.set_title(env!("CARGO_PKG_NAME"));

    widget_refs.app_window.connect_delete_event(move |_, _| {
        process::exit(1);
        // This is the recommended code but it causes an error message on exit currently
        // gtk::main_quit();
        // Inhibit(false)
    });

    if !flow_args.is_empty() {
        &widget_refs.args_buffer.set_text(&flow_args.join(" "));
    }

    // wire up the run action that needs the menu item and the args widget
    run_action(&widget_refs.run_manifest_menu, widget_refs.args_buffer.clone());

    // do any action prior to running application
    if let Some(ref flow_url) = url {
        actions::open_flow(flow_url.to_string());
    }

    widgets::init_storage(widget_refs);
}

// let button: Button = builder.get_object("button").expect("Couldn't get button");
// let entry: Entry = builder.get_object("entry").expect("Couldn't get entry");
//
// button.connect_clicked(glib::clone!(@weak window, @weak entry => move |_| {
//         let dialog = Dialog::with_buttons(Some("Hello!"),
//                                               Some(&window),
//                                               gtk::DialogFlags::MODAL,
//                                               &[("No", ResponseType::No),
//                                                 ("Yes", ResponseType::Yes),
//                                                 ("Custom", ResponseType::Other(0))]);
//
//         dialog.connect_response(glib::clone!(@weak entry => move |dialog, response| {
//             entry.set_text(&format!("Clicked {}", response));
//             dialog.close();
//         }));
//         dialog.show_all();
//     }));
//
// let button_font: Button = builder
// .get_object("button_font")
// .expect("Couldn't get button_font");
// button_font.connect_clicked(glib::clone!(@weak window => move |_| {
//         let dialog = FontChooserDialog::new(Some("Font chooser test"), Some(&window));
//
//         dialog.connect_response(|dialog, _| dialog.close());
//         dialog.show_all();
//     }));
//
// let button_recent: Button = builder
// .get_object("button_recent")
// .expect("Couldn't get button_recent");
// button_recent.connect_clicked(glib::clone!(@weak window => move |_| {
//         let dialog = RecentChooserDialog::new(Some("Recent chooser test"), Some(&window));
//         dialog.add_buttons(&[
//             ("Ok", ResponseType::Ok),
//             ("Cancel", ResponseType::Cancel)
//         ]);
//
//         dialog.connect_response(|dialog, _| dialog.close());
//         dialog.show_all();
//     }));
//
// let file_button: Button = builder
// .get_object("file_button")
// .expect("Couldn't get file_button");
// file_button.connect_clicked(glib::clone!(@weak window => move |_| {
//         // entry.set_text("Clicked!");
//         let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
//                                             FileChooserAction::Open);
//         dialog.add_buttons(&[
//             ("Open", ResponseType::Ok),
//             ("Cancel", ResponseType::Cancel)
//         ]);
//
//         dialog.set_select_multiple(true);
//
//         dialog.connect_response(|dialog, response| {
//             if response == ResponseType::Ok {
//                 let files = dialog.get_filenames();
//                 println!("Files: {:?}", files);
//             }
//             dialog.close();
//         });
//         dialog.show_all();
//     }));
//
// let app_button: Button = builder
// .get_object("app_button")
// .expect("Couldn't get app_button");
// app_button.connect_clicked(glib::clone!(@weak window => move |_| {
//         // entry.set_text("Clicked!");
//         let dialog = AppChooserDialog::new_for_content_type(Some(&window),
//                                                             gtk::DialogFlags::MODAL,
//                                                             "sh");
//
//         dialog.connect_response(|dialog, _| dialog.close());
//         dialog.show_all();
//     }));
//
// let switch: Switch = builder.get_object("switch").expect("Couldn't get switch");
// switch.connect_changed_active(glib::clone!(@weak entry => move |switch| {
//         if switch.get_active() {
//             entry.set_text("Switch On");
//         } else {
//             entry.set_text("Switch Off");
//         }
//     }));
//
// let button_about: Button = builder
// .get_object("button_about")
// .expect("Couldn't get button_about");
// let dialog: AboutDialog = builder.get_object("dialog").expect("Couldn't get dialog");
// button_about.connect_clicked(move |x| about_clicked(x, &dialog));
//
// app_window.connect_key_press_event(
// glib::clone!(@weak entry => @default-return Inhibit(false), move |_, key| {
//             let keyval = key.get_keyval();
//             let keystate = key.get_state();
//
//             println!("key pressed: {} / {:?}", keyval, keystate);
//             println!("text: {}", entry.get_text());
//
//             if keystate.intersects(gdk::ModifierType::CONTROL_MASK) {
//                 println!("You pressed Ctrl!");
//             }
//
//             Inhibit(false)
//         }),
// );