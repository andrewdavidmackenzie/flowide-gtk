use url::Url;

use flowclib::compiler::compile;
use flowclib::compiler::loader;
use flowclib::compiler::compile_wasm;
use flowclib::generator::generate;
use flowclib::model::process::Process::FlowProcess;
use flowrlib::coordinator::{Submission, Coordinator};
use flowrstructs::manifest::{DEFAULT_MANIFEST_FILENAME, Manifest};
use provider::content::provider::{MetaProvider, Provider};

use crate::{log_error, log_warn};
use crate::build_ui::UICONTEXT;
use crate::ide_runtime_client::IdeRuntimeClient;
use crate::ui_context::UiContext;
use std::env;
use simpath::Simpath;

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
                        UiContext::message("Compiling flow");
                        match compile::compile(&flow_clone) {
                            Ok(mut tables) => {
                                UiContext::message("Compiling provided implementations");
                                match compile_wasm::compile_supplied_implementations(&mut tables, false) {
                                    Ok(result) => {
                                        UiContext::message(&result);
                                        let mut manifest_url = Url::parse(&flow_url).unwrap();
                                        manifest_url = manifest_url.join(&format!("{}.json", DEFAULT_MANIFEST_FILENAME)).unwrap();
                                        UiContext::message(&format!("Creating flow manifest at: {}", manifest_url.to_string()));
                                        match generate::create_manifest(&flow, true, &manifest_url.to_string(), &tables) {
                                            Ok(manifest) => context.set_manifest(Some(manifest_url.to_string()), Some(manifest)),
                                            Err(e) => {
                                                UiContext::ui_error(&e.to_string());
                                                UiContext::message("Creation of flow manifest failed");
                                            }
                                        }
                                    }
                                    Err(e) => UiContext::ui_error(&e.to_string())
                                }
                            }
                            Err(e) => UiContext::ui_error(&e.to_string())
                        }
                    }
                    _ => {
                        UiContext::ui_error("No flow loaded to compile");
                        UiContext::message("Flow compilation failed");
                    }
                }
            }
            _ => log_error("Could not access ui context")
        }
    });
}

/*
    For the lib provider, libraries maybe installed in multiple places in the file system.
    In order to find the content, a FLOW_LIB_PATH environment variable can be configured with a
    list of directories in which to look for the library in question.

    Using the "FLOW_LIB_PATH" environment variable attempt to locate the library's root folder
    in the file system.
 */
pub fn get_lib_search_path() -> Simpath {
    // TODO read additional entries from UI
    let search_path_additions: &[String] = &[];

    let lib_search_path = Simpath::new_with_separator("FLOW_LIB_PATH", ',');

    if env::var("FLOW_LIB_PATH").is_err() && search_path_additions.is_empty() {
        log_warn("'FLOW_LIB_PATH' is not set and no LIB_DIRS supplied, so it is possible libraries referenced will not be found");
    }

    // for addition in search_path_additions {
    //     lib_search_path.add(addition);
    //     UIContext::message(&format!("'{}' added to the Library Search Path", addition));
    // }

    lib_search_path
}

/// Load a flow from 'url' in a background thread and then update the UI with a JSON representation
/// of it.
pub fn open_flow(url: String) {
    std::thread::spawn(move || {
        let provider = &MetaProvider::new(get_lib_search_path()) as &dyn Provider;

        match loader::load(&url, provider) {
            Ok(FlowProcess(flow)) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => context.set_flow(Some(flow)),
                    _ => log_error("Could not get access to uicontext")
                }
            }
            Ok(_) => UiContext::ui_error(&format!("Process loaded from Url: '{}' was not of type 'Flow'", url)),
            Err(e) => UiContext::ui_error(&format!("Could not load flow from Url: '{}'. {}", url, e.to_string()))
        }
    });
}

/// A background action that opens an existing compiled flow manifest on a thread and updates the UI
/// with it.
pub fn open_manifest(url: String) {
    std::thread::spawn(move || {
        let provider = &MetaProvider::new(get_lib_search_path()) as &dyn Provider;
        match Manifest::load(provider, &url) {
            Ok((manifest, _)) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => context.set_manifest(Some(url), Some(manifest)),
                    Err(_) => log_error("Could not lock UI Context")
                }
            }
            Err(e) => UiContext::ui_error(&format!("Error loading manifest from url '{}': {}",
                                                   url, &e.to_string()))
        }
    });
}

/// Background action that executes a compiled flow manifest on a thread passing the supplied array
/// of arguments to the runtime functions for the flow to use.
/// This may result in output to stdout, stderr or other runtime functions that will be reflected on the UI.
pub fn run_manifest(manifest_url: String, args: Vec<String>) {
    std::thread::spawn(move || {
        match Coordinator::server(1, get_lib_search_path(), true /* native */, false, false, None) {
            Ok(runtime_connection) => {
                UiContext::clear_pre_run();
                UiContext::message(&format!("Submitting manifest for execution with args: '{:?}'", args));
                let submission = Submission::new(&manifest_url, 1);
                IdeRuntimeClient::start(runtime_connection, submission, args);
            }
            Err(e) => UiContext::ui_error(&format!("Could not make connection to server: {}", e))
        }
    });
}