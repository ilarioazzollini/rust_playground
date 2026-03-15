# Build the Project

Regardless of whether you are using VS Code with Dev Containers or running `docker run` manually, from this section onward we assume that you are working inside a terminal session running in the Docker container (as previously explained in [Development Environment Setup](./setup.md)).

To manage projects of arbitrary size, this repository is organized as a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). A workspace makes it easier to develop and maintain multiple related crates within a single project. This approach allows multiple binaries, libraries, examples, and experiments to coexist in a single repository while sharing dependencies and build configuration.

In this repository, the `rs-ws` directory acts as the workspace root and contains all the crates that we will develop throughout this book.

## Binary crates

A binary crate produces an executable program.

To create a new binary crate, run:

```bash
cd rs-ws
cargo new hello_world --bin
```

We can then build and run it with:

```bash
cargo run --bin hello_world
```

## Library crates

A library crate provides reusable functionality that can be shared across multiple projects or binaries.

To create a new library crate, run:

```bash
cd rs-ws
cargo new utilities --lib
```

## Build the entire workspace

To build all crates in the workspace, run:

```bash
cargo build --workspace --all-targets
```

This command compiles every crate and all supported targets in the workspace.

## Run the tests

To execute all unit tests and integration tests across the workspace, run:

```bash
cargo test --workspace --all-targets
```

References:
- [cargo test](https://doc.rust-lang.org/cargo/guide/tests.html)

## Coverage

To generate a code coverage report, run:

```bash
cargo llvm-cov --workspace --lib --all-features
```

References:
- [cargo-llvm-cov crate](https://crates.io/crates/cargo-llvm-cov)

## Generate the docs

Rust can automatically generate documentation from comments written in the source code.

To generate the documentation for the entire workspace, run:

```bash
cargo doc --workspace --no-deps
```



References:
- [cargo doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
- [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
