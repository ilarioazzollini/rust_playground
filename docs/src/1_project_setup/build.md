# Build the Project

Regardless of whether you are using VS Code with Dev Containers or running `docker run` manually, from this section onward we assume that you are working inside a terminal session running in the Docker container (as previously explained in [Development Environment Setup](./setup.md)).

To manage projects of arbitrary size, this repository is organized as a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). A workspace makes it easier to develop and maintain multiple related crates within a single project. This approach allows multiple binaries, libraries, examples, and experiments to coexist in a single repository while sharing dependencies and build configuration.

In this repository, the `rs-ws` directory acts as the workspace root and contains all the crates that we will develop throughout this book.

## Build

To check all crates in the workspace, run:

```bash
cargo check --workspace --all-targets
```

Or, to directly build all crates in the workspace, run:

```bash
cargo build --workspace --all-targets
```

> `cargo check` checks a local package and all of its dependencies for errors. This will essentially compile the packages without performing the final step of code generation, which is faster than running `cargo build`. 

*References*:
- [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
- [cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html)

## Format

To format all crates in the workspace, run:

```bash
cargo fmt --verbose --all
```

*References*:
- [cargo fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html)
- [rustfmt](https://github.com/rust-lang/rustfmt)


## Run lints (Clippy)

Clippy is a collection of lints to catch common mistakes and improve our Rust code. It is a superset of the default [rustc lints](https://doc.rust-lang.org/rustc/lints/index.html?highlight=lint#lints).

```bash
cargo clippy --verbose --all-targets --release
```

*References*:
- [cargo clippy](https://doc.rust-lang.org/cargo/commands/cargo-clippy.html)
- [Clippy Documentation](https://doc.rust-lang.org/clippy/index.html)

## Run tests

To execute all unit tests and integration tests across the workspace, run:

```bash
cargo test --workspace --all-targets
```

*References*:
- [cargo test](https://doc.rust-lang.org/cargo/guide/tests.html)

## Line coverage report

To generate a code coverage report, run:

```bash
cargo llvm-cov --workspace --lib --all-features
```

*References*:
- [cargo-llvm-cov crate](https://crates.io/crates/cargo-llvm-cov)

## Generate the docs

Rust can automatically generate documentation from comments written in the source code.

To generate the documentation for the entire workspace, run:

```bash
cargo doc --workspace --no-deps
```

*References*:
- [cargo doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
- [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
