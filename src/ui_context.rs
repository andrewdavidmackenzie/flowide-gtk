use std::sync::{Arc, Mutex};

use gtk::{TextBufferExt, WidgetExt};

use flowclib::model::flow::Flow;
use flowrlib::loader::Loader;
use flowrstructs::manifest::Manifest;

use crate::ide_runtime_client::IDERuntimeClient;
use crate::{widgets, ui_error, message};
use toml::Value;

pub struct UIContext {
    pub loader: Option<Loader>,
    pub flow: Option<Flow>,
    pub flow_url: Option<String>,
    pub manifest: Option<Manifest>,
    pub manifest_url: Option<String>,
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
            }
        };
    }

    // Show the text representing the flow in toml, or clear the text widget
    fn set_flow_contents(&mut self, content: Option<Value>) {
        widgets::do_in_gtk_eventloop(|refs| {
            match content {
                Some(text) => refs.manifest_buffer().set_text(&text.to_string()),
                None => {
                    let (mut start, mut end) = refs.flow_buffer().get_bounds();
                    refs.flow_buffer().delete(&mut start, &mut end);
                }
            }
        });
    }

    pub fn set_manifest(&mut self, url: Option<String>, manifest: Option<Manifest>) {
        message(&format!("Manifest url set to '{:?}'", &url));
        self.manifest_url = url;
        self.manifest = manifest;

        match &self.manifest {
            Some(manifest_found) => {
                // We have a valid manifest so enable running of it
                Self::enable_manifest_run(true);

                // TODO combinator here
                match serde_json::to_string_pretty(manifest_found) {
                    Ok(manifest_content) => Self::set_manifest_contents(Some(manifest_content)),
                    Err(e) => {
                        Self::set_manifest_contents(None);
                        ui_error(&format!("Could not convert manifest to Json for display: {}",
                                          e));
                    }
                }
            }
            None => {
                Self::enable_manifest_run(false);
                Self::set_manifest_contents(None);
            }
        }
    }

    fn enable_manifest_run(enable: bool) {
        widgets::do_in_gtk_eventloop(|refs| {
            refs.run_manifest_menu().set_sensitive(enable);
        });
    }

    // Show the text representing the manifest in json, or clear the text widget
    fn set_manifest_contents(content: Option<String>) {
        widgets::do_in_gtk_eventloop(|refs| {
            match content {
                Some(text) => refs.manifest_buffer().set_text(&text),
                None => {
                    let (mut start, mut end) = refs.manifest_buffer().get_bounds();
                    refs.manifest_buffer().delete(&mut start, &mut end);
                }
            }
        });
    }
}