# 🦀 Rust Training Project

A comprehensive collection of Rust programming exercises, challenges, and examples for learning and practice. This project focuses on implementing solutions with minimal dependencies, emphasizing Rust's built-in features and standard library.

## 📚 Project Overview

This repository contains various Rust exercises and implementations designed to help you learn Rust programming concepts through practical examples. The project is structured to provide:

- **General Programming Exercises**: Common algorithm challenges and coding problems
- **100 Days of Code Challenge**: A series of daily programming challenges
- **Neural Network and Machine Learning**: Basic implementations (work in progress)
- **Standalone Applications**: Example applications in the `bin` directory

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)
- SDL3 library (for graphics examples)
  - On macOS: `brew install sdl3`

### Installation

Clone the repository:

```bash
git clone <repository-url>
cd rs-training
```

### Running the Project

This project is structured with binaries, so you need to use the `--bin` flag to run any component:

```bash
# Run the main application
cargo run --bin rs_training

# Run specific binary applications
cargo run --bin conversions
cargo run --bin sdl3_test
```

### Running Tests

Execute the test suite to verify implementations:

```bash
cargo test
```

## 📖 Project Structure

### `/src/exercises/general`

Contains general programming exercises, each with its own implementation and test file:

- `narcissistic_numbers`: Check if a number is narcissistic (Armstrong number)
- `two_sum`: Find two numbers in an array that add up to a target
- `break_camelcase`: Break camel case text with spaces
- And more...

### `/src/exercises/one_hundred_days_challenge.rs`

A collection of daily coding challenges including:

- Two Sum problem
- Matrix manipulation
- Pascal's Triangle generation
- Dutch National Flag algorithm
- Stock price optimization

### `/src/bin`

Standalone applications and utilities:

- `conversions`: Example application demonstrating various type conversions
- `sdl3_test`: Test application for SDL3 graphics library

## 🔍 Usage Examples

Here's how to use the exercises in your Rust code:

```rust
// Import the exercises module
mod exercises;

// Use the prelude to access all general exercises
use exercises::general::prelude::*;

// Or import specific exercises
mod exercises_general {
    pub use super::exercises::general::narcissistic_numbers::narcissistic;
}

fn main() {
    // Using the two_sum function from prelude
    println!("{:?}", two_sum(&[2, 2, 3, 2, 5], 4)); // Output: (0, 1)

    // Using a specific function
    println!("narcissistic: {}", exercises_general::narcissistic(153)); // Output: true
}
```

## 🛠️ Built With

- Rust 2021 Edition
- SDL3 Library for graphics examples
- Clippy for code quality and best practices

## 🧪 Code Quality Notes

This project is primarily test-oriented, with many functions that are only used in tests. To satisfy Clippy's linting rules while maintaining the test-focused structure, the following attributes are used in various files:

- `#![allow(dead_code)]` - Used in modules where functions are implemented for testing purposes but not directly used in the main application
- `#![allow(unused_imports)]` - Used where imports are needed for the module structure but might not be directly used in the current context

These attributes help maintain a clean Clippy report without having to modify the educational nature of the codebase.

## 📝 Project Maintenance

The `mod.rs` files in the exercises folder are maintained by the `tools/update_mods.ts` script, which automatically updates module declarations when new files are added to the exercises directory.

## 📄 License

This project is available for learning and practice purposes.
