# Hello, Binary Crate!

*Prerequisite: As always, we assume that we are working inside a terminal session running in our usual Docker container (as previously explained in [Development Environment Setup](../1_project_setup/setup.md)).*

Here we are, ready to say our "Hello, World!" in Rust. In order to do so, we will create our first *binary crate*. A binary crate produces an executable program.

We can create our new binary crate inside our Cargo workspace by running the [cargo new](https://doc.rust-lang.org/cargo/commands/cargo-new.html) command:

```bash
cd rs-ws
cargo new hello_world --bin
```

resulting in the creation of a crate having only one source file `rs-ws/hello_world/src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Then, we can [cargo fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html) the crate by:

```bash
cargo fmt --verbose --package hello_world
```

We can [cargo clippy](https://doc.rust-lang.org/cargo/commands/cargo-clippy.html) the crate by:

```bash
cargo clippy --verbose --package hello_world
```

We can [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) the crate by:

```bash
cargo check --package hello_world --release
```

Or [cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html):

```bash
cargo build --package hello_world --release
```

And finally, [cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html):

```bash
cargo run --package hello_world --release
```

>Notice that if we omit the `--release` flag, the target will be checked/built/run with the default `unoptimized + debuginfo` profile
