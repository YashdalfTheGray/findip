# findip

A Rust utility that looks for changes in the dynamic IP and tells someone about it

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
