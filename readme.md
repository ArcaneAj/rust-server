# ArcaneAJ's sandbox rust server

A small example rust server using [warp] that can be run as a standalone exe or as a serverless azure functions handler

## Usage

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

The Azure Functions Core Tools <https://learn.microsoft.com/en-us/azure/azure-functions/functions-run-local?tabs=v3%2Cwindows%2Ccsharp%2Cportal%2Cbash> version 3.x. Use the func --version command to check that it is correctly installed.

### Running locally

To build:

```bash
cargo build [--release]
```

To start:
As an azure funtion:

```bash
func start
```

Standalone:

```bash
cargo run [--release]
```

### To view:

As an azure function:

http://localhost:7071/api/HttpExample

Standalone:

http://localhost:3000/api/HttpExample

Uses the warp crate to handle http requests, with accompanying host.json to allow the output executable to be used as a handler for an azure function

[warp]: https://docs.rs/warp/latest/warp/
