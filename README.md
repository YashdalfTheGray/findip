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

The trait has an option to notify on some kind of schedule or notify only when there is a change.

The executable takes a YAML based config file using the `--config-file-name` or `-c` flag. This config file declares how often the check is run, what notifier is used and whether the notifier runs only when changes are detected or every time.

It also includes details about configuring the notifiers, like the credentials to use for S3, the file name to write to in case of the text file notifier, or the REST API endpoint to call with the detials.

Eventually, this config file will be able to support multiple notifiers running concurrently but for now, you're limited to choosing one.

## Config file structure

### Example with all the notifiers

As stated above, at least for now, this config file is not valid because it uses more than one notifier but it is useful as a way to show how the config file is structured and what options are available. This example notifies every 12 hours and even if there isn't a change to the IP address.

```yaml
---
cron: "0 0 */12 ? * *"
notifyOnChangeOnly: false
notifiers:
  - notifierType: s3
    properties:
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
        Authorization: "Bearer mysecrettoken"
      body:
        ip: "{{TOKEN_IP_ADDRESS}}"
  - notifierType: stdout
```

You can also use the string `{{TOKEN_IP_ADDRESS}}` as a placeholder for the external IP address as part of the configuration. `findip` will replace this with the actual IP address when it is run.

### Simplest valid example

This example outputs the IP address to stdout and only prints out the IP address to stdout when a change is detected. It checks everyday at 6am.

```yaml
---
cron: "0 0 6 * * ?"
notifyOnChangeOnly: true
notifiers:
  - notifierType: stdout
```

## References

- [Library and binary in the same Rust project](https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary)
- [`Cargo.toml` format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [`clap` docs](https://docs.rs/clap/2.33.3/clap/)
- [`cron_parser` docs](https://docs.rs/cron-parser/0.7.9/cron_parser/)
- [`reqwest` docs](https://docs.rs/reqwest/0.11.3/reqwest/index.html)
- [`serde` docs](https://serde.rs/)
- [`serde-yaml` docs](https://docs.rs/serde_yaml/0.8.17/serde_yaml/)
- IP Address finder "endpoints"
  - https://api.ipify.org/ (plaintext)
  - https://diagnostic.opendns.com/myip (plaintext)
  - https://ipinfo.io/ip (HTML)
