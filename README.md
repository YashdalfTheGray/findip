# findip

A Rust utility that looks for changes in the dynamic IP and tells someone about it

## Development

There are a couple of commands that are noteworthy since this package contains a binary as well as a library

- `cargo run --bin findip -- <arguments>` will build and run the binary with the given arguments
- `cargo build --bin findip --release` will build an executable ready to be published
- `cargo build --lib` will build a development version of the library portion
- `cargo build --lib --release` will build a release version of the library portion

The built binaries can be found within the target folder, under either `debug` for dev builds or `release` for release builds. The binary is named `findip`.

Because the library is used by the binary in this project, building the binary also builds the library parts of the code but, in case building both is necessary for some reason, a simple `cargo build` will build both things. And sticking the `--release` flag after the command will build everything for release.

## What is this?

It is a utility that finds the external IP address of the node that it is running on. It's supposed to be as lightweight as possible and supposed to run as a service or Docker container.

## This already exists, doesn't it?

It, yeah, it probably does. But I wanted to start this project because of a few reasons

- There is this new Rust AWS SDK to experiment with
- I wanted to build something that finds the external ID and figure out how to do it
- I wanted to ship a library and an executable all in one project

## Design

Like stated above, I wanted to ship a library and executable all in one project. So this project contains a binary and a library. The separation of concerns is as follows

- the binary does validation on the command line arguments and calls the library functions
- the library provides a function to find the external IP address as well as a trait that serves as a notifier so that the functionality can be extended

There are a few built in notifiers

- S3 - write to an S3 bucket with the nodename as the top level folder
- REST API - call a REST API with the external IP address, supports (or should) basic auth
- Text file - output the external IP to a text file
- Console output - just print the external IP to the console

The trait has an option to notify on some kind of schedule or notify only when there is a change.

## References

- [Library and binary in the same Rust project](https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary)
- [`Cargo.toml` format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [`clap` docs](https://docs.rs/clap/2.33.3/clap/)
- [`cron_parser` docs](https://docs.rs/cron-parser/0.7.9/cron_parser/)
- [`reqwest` docs](https://docs.rs/reqwest/0.11.3/reqwest/index.html)
- IP Address finder "endpoints"
  - https://api.ipify.org/ (plaintext)
  - https://diagnostic.opendns.com/myip (plaintext)
  - https://ipinfo.io/ip (HTML)
