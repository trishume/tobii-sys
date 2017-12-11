# Tobii Stream Engine Rust Bindings

This is a Rust crate containing FFI bindings and a few helpers for working with the [Tobii Stream Engine](https://tobii.github.io/stream_engine/) library for interfacing with Tobii eye trackers like the 4C.

The linker parameters are currently set up with the alpha SDK for macOS, which you can't get unless you ask, but with some tweaking of the linker attributes it should work on Windows as well. The bindings are for the older version of the API in the macOS version, but they shouldn't be difficult to update.
