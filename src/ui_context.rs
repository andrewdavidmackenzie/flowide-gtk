use std::sync::{Arc, Mutex};

use url::Url;

use gtk::{TextBufferExt, WidgetExt};

use flowclib::model::flow::Flow;
use flowrlib::loader::Loader;
use flowrstructs::manifest::Manifest;

use crate::ide_runtime_client::IDERuntimeClient;
use crate::{widgets, ui_error};
use toml::Value;

pub struct UIContext {
    pub loader: Option<Loader>,
    pub flow: Option<Flow>,
    pub flow_url: Option<String>,
    pub manifest: Option<Manifest>,
    pub manifest_url: Option<Url>,
    pub client: Arc<Mutex<IDERuntimeClient>>,
}

impl UIContext {
    pub fn new() -> Self {
        UIContext {
            loader: None,
            flow: None,
            flow_url: None,
            manifest: None,
            manifest_url: None,
            client: Arc::new(Mutex::new(IDERuntimeClient::new(true))),
        }
    }

    // Set the flow url and flow object into the `UIContext` for later use
    pub fn set_flow(&mut self, url: Option<String>, flow: Option<Flow>) {
        self.flow = flow;
        self.flow_url = url;

        // Serialize the flow into toml for ui display - or clear if None
        match &self.flow {
            Some(flow_found) => {
                // enable menu item that can be used to compile the loaded flow
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.compile_flow_menu().set_sensitive(true);
                });

                match toml::Value::try_from(flow_found) {
                    Ok(flow_content) => self.set_flow_contents(Some(flow_content)),
                    Err(e) => {
                        ui_error(&format!("Error serializing flow to toml: `{}`", &e.to_string()));
                        self.set_flow_contents(None);
                    }
                }
            }
            None => {
                // disable menu item that can be used to compile the loaded flow
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.compile_flow_menu().set_sensitive(false);
                });

                self.set_flow_contents(None)
            },
        };
    }

    // Show the text representing the flow in toml, or clear the text widget
    pub fn set_flow_contents(&mut self, content: Option<Value>) {
        match content {
            Some(text_value) => {
                widgets::do_in_gtk_eventloop(|refs| {
                    // show the toml representation of the flow in the text buffer
                    refs.flow_buffer().set_text(&text_value.to_string());
                });
            }
            None => {
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.flow_buffer().set_text("");
                });
            }
        }
    }
}