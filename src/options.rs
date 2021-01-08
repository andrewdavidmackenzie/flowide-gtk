use clap::{ArgMatches, AppSettings, App, Arg};
use std::env;
use url::Url;
use simplog::simplog::SimpleLogger;

// Parse the command line arguments using clap
fn get_matches<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::TrailingVarArg)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbosity")
            .takes_value(true)
            .value_name("VERBOSITY_LEVEL")
            .help("Set verbosity level for output (trace, debug, info, warn, error (default))"))
        .arg(Arg::with_name("stdin")
            .short("i")
            .long("stdin")
            .takes_value(true)
            .value_name("STDIN_FILENAME")
            .help("Read STDIN from the named file"))
        .arg(Arg::with_name("FLOW")
            .help("the name of the 'flow' definition file to open")
            .required(false)
            .index(1))
        .arg(Arg::with_name("flow_args")
            .help("Arguments that will get passed onto the flow if it is executed")
            .multiple(true))
        .get_matches()
}

// Parse the command line arguments
pub fn parse_args() -> (Option<Url>, Vec<String>, Option<String>, ) {
    let matches = get_matches();
    let mut url = None;
    if let Ok(cwd) = env::current_dir() {
        if let Ok(cwd_url) = Url::from_directory_path(cwd) {
            if let Some(flow_url) = matches.value_of("FLOW") {
                url = cwd_url.join(flow_url).ok();
            }
        }
    };

    let mut flow_args: Vec<String> = vec!();
    if let Some(args) = matches.values_of("flow_args") {
        flow_args = args.map(|a| a.to_string()).collect();
    }

    SimpleLogger::init_prefix(matches.value_of("verbosity"), false);

    let stdin_file= matches.value_of("stdin").map(String::from);

    (url, flow_args, stdin_file)
}