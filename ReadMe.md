<h3 align="center">📈Simple Server Watcher</h3>
A lightweight server monitoring agent written in Rust that collects system metrics and POSTs them to a remote endpoint at a configurable interval.

#### Table of Contents
- [About The Project](#about-the-project)
  - [Key Features:](#key-features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Author](#author)

## About The Project
Simple Server Watcher runs as a background agent on any machine you want to monitor. Every few seconds it collects a snapshot of system health — RAM, swap, disk usage, and running processes — serializes it to JSON, writes it locally, and forwards it to a configured HTTP endpoint.

### Key Features:
* RAM and swap usage (total, used, available)
* Per-disk stats (type, capacity, usage, filesystem)
* Per-process stats (name, PID, memory, virtual memory, runtime, user)
* Saves a local copy of the latest snapshot to ./data/data.json
* POSTs JSON data to a configurable remote endpoint
* Configurable polling interval (default: 5 seconds)
* Cross-platform (Linux, macOS, Windows)

## Getting Started
### Prerequisites
This is an example of how to list things you need to use the software and how to install them.
* [Rust + Cargo](https://rustup.rs)
  
Check Cargo is installed
```sh
cargo --help
```

Should show the following 
```txt
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]
       cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...

Options:
  -V, --version                  Print version info and exit
      --list                     List installed commands
      --explain <CODE>           Provide a detailed explanation of a rustc error message
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring [possible values: auto, always, never]
  -C <DIRECTORY>                 Change to DIRECTORY before doing anything (nightly-only)
      --locked                   Assert that `Cargo.lock` will remain unchanged
      --offline                  Run without accessing the network
      --frozen                   Equivalent to specifying both --locked and --offline
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Commands:
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary
    ...         See all commands with --list

See 'cargo help <command>' for more information on a specific command.
```

### Installation

1. Clone this [repo](https://github.com/ArloFilley/lmc-simulator)
   ```sh
   git clone https://github.com/ArloFilley/simple-server-watcher.git
   cd simple-server-watcher
   ```
2. Configure your key and endpoint in `src/main.rs`
   ```rust
   let key = String::from("your-secret-key");
   let endpoint = String::from("https://your-endpoint.com/data");
   ```
4. Build and run
   ```sh
   cargo build --release
   ./target/release/simple-server-watcher
   ```

## Usage

Once running, the watcher will collect and POST system info every 5 seconds. The JSON payload includes a key field for authenticating with your endpoint, along with all collected metrics.
The latest snapshot is also saved locally to `./data/data.json` for debugging or local consumption.

## License

Distributed under the MIT License. See [LICENSE.txt](/LISCENSE.txt) for more information.

## Author
Arlo Filley - [Contact Links](https://github.com/ArloFilley/ArloFilley?tab=readme-ov-file#-contact-me)