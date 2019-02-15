# Simple Rust example

- `app` simulates a binary project.
- `somepackage` represents an external library.

App's `main.rs` imports code from its own crate and from the external package. External package is comprised of both rust as c code.
A build script `build.rs` tells Cargo how to deal with the foreign c code.
