use gtk::{ButtonsType, DialogFlags, TextBufferExt, WidgetExt, MessageDialog, MessageType};
use gtk::prelude::*;

use flowclib::model::flow::Flow;
use flowrlib::loader::Loader;
use flowrstructs::manifest::Manifest;

use crate::{widgets, FLOW_GRAPH_PAGE};
use crate::widgets::WidgetRefs;
use std::rc::Rc;
use crate::MANIFEST_PAGE;

pub struct UIContext {
    pub loader: Option<Loader>,
    pub flow: Option<Flow>,
    pub manifest: Option<Manifest>,
    pub manifest_url: Option<String>,
}

impl UIContext {
    pub fn new() -> Self {
        UIContext {
            loader: None,
            flow: None,
            manifest: None,
            manifest_url: None,
        }
    }

    // Set the flow url and flow object into the `UIContext` for later use
    pub fn set_flow(&mut self, flow: Option<Flow>) {
        self.flow = flow;

        if let Some(flow_loaded) = &self.flow {
            UIContext::message(&format!("Flow loaded from '{:?}'", flow_loaded.source_url));
        }

        // Serialize the flow into toml for ui display - or clear if None
        match &self.flow {
            Some(flow_found) => {
                // enable menu item that can be used to compile the loaded flow
                // clear contents of manifest and other widgets
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.compile_flow_menu().set_sensitive(true);
                    Self::clear_manifest_contents(&refs);
                    Self::clear_stdout(&refs);
                    Self::clear_stderr(&refs);
                });

                Self::set_flow_graph_contents(flow_found);

                // TODO also serialize to toml
                // but it looks like their is an ambiguity as it reports an error
                // see https://stackoverflow.com/questions/57560593/why-do-i-get-an-unsupportedtype-error-when-serializing-to-toml-with-a-manually-i
                match serde_json::to_string_pretty(&flow_found) {
                    Ok(flow_content) => self.set_flow_json_contents(Some(flow_content)),
                    Err(e) => {
                        UIContext::ui_error(&format!("Error serializing flow to toml: `{}`", &e.to_string()));
                        self.set_flow_json_contents(None);
                    }
                }
            }
            None => {
                // disable menu item that can be used to compile the loaded flow
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.compile_flow_menu().set_sensitive(false);
                    Self::clear_flow_graph_contents(&refs);
                });

                self.set_flow_json_contents(None);
            }
        };
    }

    fn set_flow_graph_contents(_graph: &Flow) {
        widgets::do_in_gtk_eventloop(|_refs| {
                    // TODO
        });
    }

    // Show the text representing the flow in toml, or clear the text widget
    fn set_flow_json_contents(&mut self, content: Option<String>) {
        widgets::do_in_gtk_eventloop(|refs| {
            match content {
                Some(text) => {
                    refs.flow_buffer().set_text(&text);
                    // Select flow graph view tab when new flow loaded
                    refs.flow_notebook().set_property_page(FLOW_GRAPH_PAGE);
                },
                None => Self::clear_flow_json_contents(&refs)
            }
        });
    }

    // Set the manifest url (where the compiled manifest is) and manifest object into the
    // `UIContext` for later use
    pub fn set_manifest(&mut self, url: Option<String>, manifest: Option<Manifest>) {
        self.manifest_url = url;
        self.manifest = manifest;

        if let Some(manifest_url) = &self.manifest_url {
            UIContext::message(&format!("Compiled flow Manifest at '{:?}'", manifest_url));
        }

        match &self.manifest {
            Some(manifest_found) => {
                // We have a valid manifest so enable running of it, clear other widgets
                widgets::do_in_gtk_eventloop(|refs| {
                    Self::enable_manifest_run(&refs, true);
                    Self::clear_stdout(&refs);
                    Self::clear_stderr(&refs);
                });

                // TODO combinator here
                match serde_json::to_string_pretty(manifest_found) {
                    Ok(manifest_content) => Self::set_manifest_contents(Some(manifest_content)),
                    Err(e) => {
                        Self::set_manifest_contents(None);
                        UIContext::ui_error(&format!("Could not convert manifest to Json for display: {}",
                                                     e));
                    }
                }
            }
            None => {
                widgets::do_in_gtk_eventloop(|refs| {
                    Self::enable_manifest_run(&refs, false);
                    Self::set_manifest_contents(None);
                });
            }
        }
    }

    // Enable or Disable any UI elements that are used to trigger running of the compiled manifest
    fn enable_manifest_run(refs: &Rc<WidgetRefs>, enable: bool) {
        refs.run_manifest_menu().set_sensitive(enable);
    }

    // Show the text representing the manifest in json, or clear the text widget
    fn set_manifest_contents(content: Option<String>) {
        widgets::do_in_gtk_eventloop(|refs| {
            match content {
                Some(text) => {
                    refs.manifest_buffer().set_text(&text);
                    refs.flow_notebook().set_property_page(MANIFEST_PAGE); // Select manifest page
                },
                None => Self::clear_manifest_contents(&refs)
            }
        });
    }

    fn clear_flow_graph_contents(_refs: &Rc<WidgetRefs>) {
        // let (mut start, mut end) = refs.flow_buffer().get_bounds();
        // refs.flow_buffer().delete(&mut start, &mut end);
    }

    fn clear_flow_json_contents(refs: &Rc<WidgetRefs>) {
        let (mut start, mut end) = refs.flow_buffer().get_bounds();
        refs.flow_buffer().delete(&mut start, &mut end);
    }

    fn clear_manifest_contents(refs: &Rc<WidgetRefs>) {
        let (mut start, mut end) = refs.manifest_buffer().get_bounds();
        refs.manifest_buffer().delete(&mut start, &mut end);
    }

    fn clear_stdout(refs: &Rc<WidgetRefs>) {
        let (mut start, mut end) = refs.stdout().get_bounds();
        refs.stdout().delete(&mut start, &mut end);
    }

    fn clear_stderr(refs: &Rc<WidgetRefs>) {
        let (mut start, mut end) = refs.stdout().get_bounds();
        refs.stdout().delete(&mut start, &mut end);
    }

    pub fn clear_pre_run() {
        widgets::do_in_gtk_eventloop(|refs| {
            Self::clear_stdout(&refs);
            Self::clear_stderr(&refs);
        });
    }

    // Pop-up a message dialog to display the error and an OK button
    pub fn ui_error(message: &str) {
        widgets::do_in_gtk_eventloop(|refs| {
            MessageDialog::new(Some(&refs.app_window()),
                               DialogFlags::MODAL,
                               MessageType::Error,
                               ButtonsType::Ok,
                               message).run();
        });
    }

    pub fn message(message: &str) {
        widgets::do_in_gtk_eventloop(|refs| {
            refs.status_message().set_label(message);
        });
    }
}