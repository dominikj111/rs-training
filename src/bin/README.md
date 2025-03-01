# Rust Training Binary Executables

This folder contains standalone executable applications that can be run independently within the rs-training project context. Each binary serves a specific purpose such as demonstrations, tests, or benchmarks.

## Running the Binaries

You can run any binary using Cargo with the following command:

```bash
cargo run --bin <binary_name>
```

## Available Binaries

### conversions

A demonstration application showcasing various type conversion techniques in Rust. This binary illustrates best practices for handling data type conversions and transformations.

```bash
cargo run --bin conversions
```

### sdl3_test

A test application for the Simple DirectMedia Layer (SDL3) library. This binary demonstrates how to use SDL3 for graphics, audio, and input handling in Rust applications.

```bash
cargo run --bin sdl3_test
```

## Adding New Binaries

To add a new binary to this collection, create a new `.rs` file in this directory. The filename will become the binary name that can be executed with `cargo run --bin <filename>`.
