# Guessing Game

> *Prerequisite: As always, we assume that we are working inside a terminal session running in our usual Docker container (as previously explained in [Development Environment Setup](../1_project_setup/setup.md)).*
>
> *Source code at [`rust_playground/rs-ws/guessing_game](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/guessing_game)*

Some parts of the program, as well as some parts of this doc page, are taken from [The Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

## Setup

First of all, we create a new binary crate:

```bash
cd rs-ws
cargo new guessing_game --bin
```

You can find the code in the source file [`rust_playground/rs-ws/guessing_game/src/main.rs`](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/guessing_game/src/main.rs).

This app will need a dependency on the [rand crate](https://crates.io/crates/rand). In order to add the dependency, we can either manually modify the [`rust_playground/rs-ws/guessing_game/Cargo.toml`](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/guessing_game/Cargo.toml) file or use [cargo add](https://doc.rust-lang.org/cargo/commands/cargo-add.html) (and maybe update them after a while by means of [cargo update](https://doc.rust-lang.org/cargo/commands/cargo-update.html))

In this case, let us follow the Rust book and directly modify the toml file:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

and then, the next time we are going to build the project, Cargo will automatically take care of it.

## Build and Run

We can build and run the `guessing_game` package by:

```bash
cargo run --package guessing_game --release
```

And we can play the guessing game, which will look like this:
```bash
-----------------------------
Welcome to the Guessing Game!

Rules:
- I have generated a random integer number within the range of 0 to 100 inclusive
- You have 10 attempts to guess the number
- For each attempt, I'll tell you whether your guess is lower, higher, or the exact number
-----------------------------

Attempt #1. Please input your guess and press <Enter>.
50
LOWER!

Attempt #2. Please input your guess and press <Enter>.
80
LOWER!

Attempt #3. Please input your guess and press <Enter>.
90
YOU WON!

-------------------
End of the program.
-------------------
```
