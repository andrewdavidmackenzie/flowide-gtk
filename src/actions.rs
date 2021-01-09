use url::Url;

use flowclib::compiler::compile;
use flowclib::compiler::loader;
use flowclib::compiler::compile_wasm;
use flowclib::generator::generate;
use flowclib::model::process::Process::FlowProcess;
use flowrlib::coordinator::{Submission, Coordinator};
use flowrstructs::manifest::{DEFAULT_MANIFEST_FILENAME, Manifest};
use provider::content::provider::MetaProvider;

use crate::log_error;
use crate::UICONTEXT;
use crate::ide_runtime_client::IDERuntimeClient;
use crate::ui_context::UIContext;

/// Background action that compiles a flow on a thread and then updates the UI with the resulting
/// compiled flow manifest
pub fn compile_flow() {
    std::thread::spawn(move || {
        match UICONTEXT.try_lock() {
            Ok(ref mut context) => {
                match context.flow {
                    Some(ref flow) => {
                        let flow_clone = flow.clone();
                        let flow_url = flow.source_url.clone();
                        UIContext::message("Compiling flow");
                        match compile::compile(&flow_clone) {
                            Ok(mut tables) => {
                                UIContext::message("Compiling provided implementations");
                                match compile_wasm::compile_supplied_implementations(&mut tables, false) {
                                    Ok(result) => {
                                        UIContext::message(&result);
                                        let mut manifest_url = Url::parse(&flow_url).unwrap();
                                        manifest_url = manifest_url.join(&format!("{}.json", DEFAULT_MANIFEST_FILENAME)).unwrap();
                                        UIContext::message(&format!("Creating flow manifest at: {}", manifest_url.to_string()));
                                        match generate::create_manifest(&flow, true, &manifest_url.to_string(), &tables) {
                                            Ok(manifest) => {
                                                context.set_manifest(Some(manifest_url.to_string()), Some(manifest))
                                            }
                                            Err(e) => {
                                                UIContext::ui_error(&e.to_string());
                                                UIContext::message("Creation of flow manifest failed");
                                            }
                                        }
                                    }
                                    Err(e) => UIContext::ui_error(&e.to_string())
                                }
                            }
                            Err(e) => UIContext::ui_error(&e.to_string())
                        }
                    }
                    _ => {
                        UIContext::ui_error("No flow loaded to compile");
                        UIContext::message("Flow compilation failed");
                    }
                }
            }
            _ => log_error("Could not access ui context")
        }
    });
}

/// Load a flow from 'url' in a background thread and then update the UI with a JSON representation
/// of it.
pub fn open_flow(url: String) {
    std::thread::spawn(move || {
        let provider = MetaProvider {};

        match loader::load(&url, &provider) {
            Ok(FlowProcess(flow)) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => context.set_flow(Some(flow)),
                    _ => log_error("Could not get access to uicontext")
                }
            }
            Ok(_) => UIContext::ui_error(&format!("Process loaded from Url: '{}' was not of type 'Flow'", url)),
            _ => UIContext::ui_error(&format!("Could not load flow from Url: '{}'", url))
        }
    });
}

/// A background action that opens an existing compiled flow manifest on a thread and updates the UI
/// with it.
pub fn open_manifest(url: String) {
    std::thread::spawn(move || {
        let provider = MetaProvider {};
        match Manifest::load(&provider, &url) {
            Ok((manifest, _)) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => context.set_manifest(Some(url), Some(manifest)),
                    Err(_) => log_error("Could not lock UI Context")
                }
            }
            Err(e) => UIContext::ui_error(&format!("Error loading manifest from url '{}': {}",
                                                   url, &e.to_string()))
        }
    });
}

/// Background action that executes a compiled flow manifest on a thread passing the supplied array
/// of arguments to the runtime functions for the flow to use.
/// This may result in output to stdout, stderr or other runtime functions that will be reflected on the UI.
pub fn run_manifest(args: Vec<String>) {
    std::thread::spawn(move || {
        match UICONTEXT.try_lock() {
            Ok(ref mut context) => {
                match &context.manifest_url {
                    Some(manifest_url) => {
                        match Coordinator::server(1, true /* native */, false, false, None) {
                            Ok(runtime_connection) => {
                                UIContext::clear_pre_run();
                                UIContext::message(&format!("Submitting manifest for execution with args: '{:?}'", args));
                                let submission = Submission::new(&manifest_url.to_string(), 1);
                                IDERuntimeClient::start(runtime_connection, submission, args);
                            }
                            Err(e) => UIContext::ui_error(&format!("Could not make connection to server: {}", e))
                        }
                    }
                    _ => UIContext::ui_error("No manifest loaded to run")
                }
            }
            _ => log_error("Could not get access to uicontext and client")
        }
    });
}