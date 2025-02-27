# 🦀 Rust Training Project

🎯 A collection of Rust programming exercises and examples for learning and practice.

## 📚 Features

- 🎮 SDL3 integration for graphics
- 🔧 Configured with Clippy for best practices
- 📝 Neural Network and Machine Learning exercises
- ⚡ Optimized dependencies in dev profile

## 🚀 Getting Started

```bash
cargo fmt
cargo clippy
cargo test
cargo run --bin ts-training
```

## 🛠️ Built With

- Rust 2021 Edition
- SDL3 Library

## 📖 License

This project is available for learning and practice purposes.

---

The excercises is the folder containing all possible excersises and related documentation or assignment.

the mod.rs files are maintained by the tools/update_mods.ts script

The excersices can by used then by
```
mod exercises;

mod general_exercises {
    pub use super::exercises::general::prelude::*;
}

fn main() {
    println!("narcissistic: {}", general_exercises::narcissistic(153));
}
```

---

we have example of integration tests where we need the lib.rs file.

---

We can also run sub-application under current project context (see the bin/README.md for more details).

---

