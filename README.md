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

- ~~There is this new Rust AWS SDK to experiment with~~ Turns out, it only supports environment variables for auth, nothing else and I wanted a different way of configuring auth. So we're using good ol' rusoto! ❤️
- I wanted to build something that finds the external IP and figure out how to do it
- I wanted to ship a library and an executable all in one project, which I haven't done with Rust yet

## Design

Like stated above, I wanted to ship a library and executable all in one project. So this project contains a binary and a library. The separation of concerns is as follows

- the binary does validation on the command line arguments and calls the library functions
- the library provides a function to find the external IP address as well as a trait that serves as a notifier so that the functionality can be extended

There are a few built in notifiers

- S3 - write to an S3 bucket with the nodename as the top level folder
- REST API - call a REST API with the external IP address, supports (or should) basic auth
- Text file - output the external IP to a text file
- Console output - just print the external IP to the console

The trait has an option to notify on some kind of schedule and optionally, notify only when there is a change.

## Config file structure

The executable takes a YAML based config file using the `--config-file-name` or `-c` flag. This config file declares how often the check is run, what notifier is used and whether the notifier runs only when changes are detected or every time.

It also includes details about configuring the notifiers, like the credentials to use for S3, the file name to write to in case of the text file notifier, or the REST API endpoint to call with the detials.

Eventually, this config file will be able to support multiple notifiers running concurrently but for now, you're limited to choosing one. The required keys are as follows:

```yaml
cron: (cron expression)
notifyOnChangeOnly: true|false
notifiers:
  - notifierType: s3|file|restApi|stdout
    properties: (see below)
```

The config file supports some other configuration options as well like where to put the logs and which services to use to find the IP address to report. These keys are mentioned here because they are optional and defaults will be loaded for them as necessary when the config file is parsed in.

The defaults for the `services` key are

```yaml
services:
  - https://api.ipify.org/"
  - https://diagnostic.opendns.com/myip
```

The defaults for the `logging_config` key is

```yaml
logging_config:
  log_file: /tmp/ip_notifier.log,
  log_level: info,
  decorate: true,
```

### Notifiers

As stated above, at least for now, this config file is not valid because it uses more than one notifier but it is useful as a way to show how the config file is structured and what options are available. This example notifies every 12 hours and even if there isn't a change to the IP address. This configuration also includes an optional key called `services` if you wanted to customize what services to use to check for the host's public IP address.

```yaml
notifiers:
  - notifierType: s3
    properties:
      access_key_id: something
      secret_access_key: something
      assumeRoleArn: roleArn
      region: us-west-2
      bucketName: bucketName
  - notifierType: file
    properties:
      filePath: testfile.log
      overwrite: false
  - notifierType: restApi
    properties:
      url: https://something.com/some/api
      method: POST
      headers:
        Content-Type: application/json
        Authorization: 'Bearer mysecrettoken'
      body:
        ip: '{{TOKEN_IP_ADDRESS}}'
  - notifierType: stdout
```

You can also use the string `{{TOKEN_IP_ADDRESS}}` as a placeholder for the external IP address as part of the configuration. `findip` will replace this with the actual IP address when it is run.

### Simplest valid example

This example outputs the IP address to stdout and only prints out the IP address to stdout when a change is detected. It checks everyday at 6am.

```yaml
---
cron: '0 0 6 * * ?'
notifyOnChangeOnly: true
notifiers:
  - notifierType: stdout
```

## Coverage (this method doesn't work, need to figure out something else)

Okay, so coverage within Rust is a little wild. You have basically 2 options

- use a package called tarpaulin which is a package that only works on linux
- use the nightly Rust toolchain and use the LLVM-based native instrumentation by setting it up manually and installing a bunch of things.

We are going to use the second option here, set up the nightly toolchain and install the required things to make the coverage work. The guide is linked in the references section below. It is important to note that we're only doing this to get the test coverage but you can also generate runtime coverage. The compiler will inject LLVM counter statements into a binary build as well and output a `.profraw` file.

This process only works (for now) with the Rust nightly toolchain. Accordingly, there is a `rust-toolchain` file with the word `nightly` in it. A thing to note here is that you need a fairly new nightly to run this as of this writing so it is a good idea to run `rustup update nightly` to bring down the latest nightly before following the steps. This tells Cargo to use the nightly toolchain. Then the process is as follows

1. We have to install the `rustfilt` demangler - `cargo install rustfilt`
1. Then we have to clean the project - `cargo clean`
1. Then we rebuild with the right flags - `RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="./coverage/findip-%m.profraw" cargo test`
1. Next, we install the coverage tools - `rustup component add llvm-tools-preview && cargo install cargo-binutils`
1. Once the tests are run, the instrumented tests output a profile file for each test module. We then have to merge it all together using `cargo profdata -- merge -sparse ./coverage/findip-*.profraw -o ./coverage/findip.profdata`
1. We are almost done, we only have to generate the report and then show it. Generating the report can be done using the following command

   ```sh
   cargo cov -- report \
     --use-color \
     --ignore-filename-regex='/.cargo/registry' \
     --instr-profile=./coverage/findip.profdata \
     --object target/debug/deps/findip_lib-<some_hash> \
     --object target/debug/deps/findip
   ```

1. The generated report can be seen using the following command

   ```sh
   cargo cov -- show \
     --use-color \
     --ignore-filename-regex='/.cargo/registry' \
     --instr-profile=./coverage/findip.profdata \
     --object target/debug/deps/findip_lib-<some_hash> \
     --object target/debug/deps/findip
     --show-instantiations \
     --show-line-counts-or-regions \
     --Xdemangler=rustfilt | less -R
   ```

## References

- [Library and binary in the same Rust project](https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary)
- [`Cargo.toml` format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [`clap` docs](https://docs.rs/clap/2.33.3/clap/)
- [`cron_parser` docs](https://docs.rs/cron-parser/0.7.9/cron_parser/)
- [`fern` docs](https://docs.rs/fern/0.6.0/fern/)
- [`reqwest` docs](https://docs.rs/reqwest/0.11.3/reqwest/index.html)
- [`rusoto` docs](https://www.rusoto.org/)
- [`serde` docs](https://serde.rs/)
- [`serde-yaml` docs](https://docs.rs/serde_yaml/0.8.17/serde_yaml/)
- [Rust LLVM based coverage](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html)
- [`tarpaulin`](https://github.com/xd009642/tarpaulin)
- IP Address finder "endpoints"
  - https://api.ipify.org/ (plaintext)
  - https://diagnostic.opendns.com/myip (plaintext)
  - https://ipinfo.io/ip (HTML)
