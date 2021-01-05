use url::Url;

use flowclib::compiler::compile;
use flowclib::compiler::loader;
use flowclib::generator::generate;
use flowclib::model::flow::Flow;
use flowclib::model::process::Process::FlowProcess;
use flowrlib::coordinator::Submission;
use flowrstructs::manifest::{DEFAULT_MANIFEST_FILENAME, Manifest};
use provider::content::provider::MetaProvider;

use crate::{message, ui_error};
use crate::UICONTEXT;

fn manifest_url(flow_url_str: &str) -> String {
    let flow_url = Url::parse(&flow_url_str).unwrap();
    flow_url.join(DEFAULT_MANIFEST_FILENAME).unwrap().to_string()
}

pub fn compile_flow() {
    std::thread::spawn(move || {
        match UICONTEXT.try_lock() {
            Ok(ref mut context) => {
                match (&context.flow, &context.flow_url) {
                    (Some(ref flow), Some(ref flow_url_str)) => {
                        let flow_clone = flow.clone();
                        let flow_url_clone = flow_url_str.clone();
                        message("Compiling flow");
                        match compile::compile(&flow_clone) {
                            Ok(tables) => {
                                //                        info!("==== Compiler phase: Compiling provided implementations");
                                //                        compile_supplied_implementations(&mut tables, provided_implementations, release)?;
                                match generate::create_manifest(&flow, true, &flow_url_clone, &tables) {
                                    Ok(manifest) => context.set_manifest(Some(manifest_url(&flow_url_clone)), Some(manifest)),
                                    Err(e) => message(&e.to_string())
                                }
                            }
                            Err(e) => message(&e.to_string())
                        }
                    }
                    _ => message("No flow loaded to compile")
                }
            }
            _ => message("Could not access ui context")
        }
    });
}

fn load_flow_from_url(url: &str) -> Result<Flow, String> {
    let provider = MetaProvider {};

    match loader::load(url, &provider)
        .map_err(|e| format!("Could not load flow context: '{}'", e.to_string()))? {
        FlowProcess(flow) => Ok(flow),
        _ => Err(format!("Process loaded from Url: '{}' was not of type 'Flow'", url))
    }
}

pub fn open_flow(url: String) {
    std::thread::spawn(move || {
        match load_flow_from_url(&url) {
            Ok(flow) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => {
                        // set flow_url and flow object into the context for later use
                        context.set_flow(Some(url), Some(flow));
                    }
                    _ => ui_error("Could not get access to uicontext")
                }
            }
            Err(e) => ui_error(&e)
        }
    });
}

pub fn open_manifest(url: String) {
    std::thread::spawn(move || {
        let provider = MetaProvider {};
        match Manifest::load(&provider, &url) {
            Ok((manifest, _)) => {
                match UICONTEXT.try_lock() {
                    Ok(mut context) => {
                        context.set_manifest(Some(url), Some(manifest));
                    }
                    Err(_) => ui_error("Could not lock UI Context")
                }
            }
            Err(e) => ui_error(&format!("Error loading manifest from url '{}': {}",
                                        url, &e.to_string()))
        }
    });
}

fn set_args(arg: Vec<String>) {
    match UICONTEXT.try_lock() {
        Ok(ref mut context) => {
            let mut guard = context.client.lock().unwrap();
            guard.set_args(arg);
        }
        _ => message("Could not get access to uicontext and client")
    }
}

pub fn run_manifest(args: Vec<String>) {
    std::thread::spawn(move || {
        match UICONTEXT.try_lock() {
            Ok(ref mut context) => {
                match &context.manifest_url {
                    Some(manifest_url) => {
                        set_args(args);
                        // let debug_client = CLI_DEBUG_CLIENT;
                        let _submission = Submission::new(&manifest_url.to_string(),
                                                          1,
                                                          false);
                        // let mut coordinator = Coordinator::new(1);
                        // coordinator.init();
                        //
                        // coordinator.submit(submission);
                        message("Submitted flow for execution");
                    }
                    _ => message("No manifest loaded to run")
                }
            }
            _ => message("Could not get access to uicontext and client")
        }
    });
}