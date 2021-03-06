[![Build Status](https://travis-ci.org/Metaswitch/iobuffer.svg?branch=master)](https://travis-ci.org/Metaswitch/iobuffer)
[![Current Version](https://img.shields.io/crates/v/iobuffer.svg)](https://crates.io/crates/iobuffer)
[![License](https://img.shields.io/github/license/Metaswitch/iobuffer.svg)](LICENSE)

# iobuffer

This repository contains a Rust crate `iobuffer`.
This is a memory-based buffer which implements both the `std::io::Write` and `std::io::Write` traits.

It is useful in testing - for crates whose interface takes a`std::io::Read` or `std::io::Write`,
using an `iobuffer::IoBuffer` instance allows tests to have full access to what has been read or written by the library.

See the documentation (as generated by `cargo doc`) or [source](src/lib.rs) for more information.

[Documentation (crates.io)](https://docs.rs/iobuffer).
