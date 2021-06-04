# findip

A Rust utility that looks for changes in the dynamic IP and tells someone about it

## What is this?

It is a utility that finds the external IP address of the node that it is running on. It's supposed to be as lightweight as possible and supposed to run as a service or Docker container.

## This already exists, doesn't it?

It, yeah, it probably does. But I wanted to start this project because of a few reasons

- There is this new Rust AWS SDK to experiment with
- I wanted to build something that finds the external ID and figure out how to do it
- I wanted to ship a library and an executable all in one project

## References

- [Library and binary in the same Rust project](https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary)
