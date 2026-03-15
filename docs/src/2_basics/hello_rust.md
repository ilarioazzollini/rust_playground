# Hello, Rust World!

*Prerequisite: As always, we assume that we are working inside a terminal session running in our usual Docker container (as previously explained in [Development Environment Setup](../1_project_setup/setup.md)).*

Here we are, ready to say our "Hello, World!" in Rust. This is basically needed to just getting familiar with creating a new crate, build it and run it.

As explained in [Build the Project](../1_project_setup/build.md), we can create our new binary crate inside our Cargo workspace by running:

```bash
cd rs-ws
cargo new hello_rust --bin
```

resulting in the creation of a crate having only one source file `rs-ws/hello_rust/src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

We can [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) the crate by:

```bash
cargo check --bin hello_world --release
```

Or [cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html):

```bash
cargo build --bin hello_world --release
```

And finally, [cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html):

```bash
cargo run --bin hello_world --release
```

>Notice that if we omit the `--release` flag, the target will be checked/built/run with the default `unoptimized + debuginfo` profile
