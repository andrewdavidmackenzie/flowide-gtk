[![Build Status](https://travis-ci.org/andrewdavidmackenzie/flowide-gtk.svg?branch=master)](https://travis-ci.org/andrewdavidmackenzie/flowide-gtk)
[![Generic badge](https://img.shields.io/badge/macos-supported-Green.svg)](https://shields.io/)
[![Generic badge](https://img.shields.io/badge/linux-supported-Green.svg)](https://shields.io/)
[![Generic badge](https://img.shields.io/badge/Rust-stable-Green.svg)](https://shields.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# `flowide`

`flowide` is an experimental visual IDE (using gtk3) for [flow](http://github.com/andrewdavidmackenzie/flow) 
programs.

Currently, it allows you to:
  * Load a flow description by selecting a `toml` flow description file via a file dialog
    * The flow JSON representation is shown in text in the "Flow (json)" tab
  * Compile the flow to it's manifest format (also json) using `flowclib`
    * The manifest is shown in JSON text in the "Manifest" tab
  * Load a pre-compiled JSON manifest directly
    * JSON is shown in text in the "Manifest" tab
  * Run the compiled flow from its manifest
    * STDERR and STDOUT are shown in two tabs

# Example UI

The UI is still very basic, but here it is after having loaded the "fibonacci" flow from context.toml description
file, compiled it to manifest.json manifest format (using `flowlibc`) and then the outout on the `STDOUT` tab when 
running it (using `flowrlib`).
![UI sample](images/running_fibonacci.png)

# Command line usage

You can find the latest up-to-date command line usage using:
* `cargo run -- --help` or
* `flowide --help` if you have installed it using `cargo install flowide-gtk`

Either will show you the usage, which currently is:
```bash
flowide 0.32.0

USAGE:
    flowide [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --stdin <STDIN_FILENAME>         Read STDIN from the named file
    -v, --verbosity <VERBOSITY_LEVEL>    Set verbosity level for output (trace, debug, info, warn, error (default))

ARGS:
    <FLOW>            the name of the 'flow' definition file to open
    <flow_args>...    Arguments that will get passed onto the flow if it is executed
```

# Building
## Installing dependencies
You can have the Makrfile install the required dependencies on Mac OS X or Linux using
* `make config`

## Building and Running
You can build or run using the make targets:
* `make build`
* `make run`

or just use cargo:
* `cargo build`
* `cargo run`

## Running tests
There are not many tests yet, but you can run all there are using:
* `make test`
* `cargo test`

## Do it all with `make`
If you run just `make`, the default make target will run:
* `cargo build test clippy`

Where `clippy` is the rust `cargo` component that runs a number of lint checks on the code.

# Platforms supported
`flowide` is developed mainly on Mac OS X, but CI builds are done for Mac
and Linux. But since there is no automatic UI testing done in 
CI, all it ensures is that it builds and unit tests pass on both platforms.