# Hello, Library Crate!

> *Prerequisite: As always, we assume that we are working inside a terminal session running in our usual Docker container (as previously explained in [Development Environment Setup](../1_project_setup/setup.md)).*
>
> *Source code at [`rust_playground/rs-ws/my_lib](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/my_lib)*

A *library crate* provides reusable functionality that can be shared across multiple projects or binaries.

To create a new library crate, we can run:

```bash
cd rs-ws
cargo new my_lib --lib
```

resulting in the creation of a crate having only one source file `rs-ws/my_lib/src/lib.rs`:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

As we can see, by default the library crate has an `add` function, as well as an `it_works` test function, which is a unit test for `add`.

As for a [binary crate](./hello_bin.md), we can format:

```bash
cargo fmt --verbose --package my_lib
```

and we can run Clippy lints:

```bash
cargo clippy --verbose --package my_lib --release
```

and we can check the crate:

```bash
cargo check --package my_lib --release
```

or we can build it:

```bash
cargo build --package my_lib --release
```

In addition, we can run the tests:

```bash
cargo test --package my_lib --release
```

Get a line coverage report:

```bash
cargo llvm-cov --package my_lib --release
```

And, last but not least, we can automatically generate the documentation:

```bash
cargo doc --package my_lib --no-deps --release
```

that will end up in `rust_playground/target/doc/my_lib`, and we can serve the docs by:

```bash
python3 -m http.server 8080 -d /root/rust_playground/target/doc
```

and read them by opening a browser and navigating to [http://localhost:8080/my_lib/index.html](http://localhost:8080/my_lib/index.html).

## Create a binary crate depending on this library

> *Source code at [`rust_playground/rs-ws/my_app](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/my_app)*

Now, we can create a new binary crate that depends on the library crate we just created. It is really easy to do so in a workspace.

Let us start by creating `my_app`:

```bash
cd rs-ws
cargo new my_app --bin
```

Then, navigate to its `Cargo.toml` file (at [`rust_playground/rs-ws/my_app/Cargo.toml](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/my_app/Cargo.toml)), and under the `dependencies` section, add a dependency on `my_lib` as follows:

```toml
[dependencies]
my_lib = { path = "../my_lib" }
```

Now, we can modify the [`rust_playground/rs-ws/my_app/src/main.rs](https://github.com/ilarioazzollini/rust_playground/tree/main/rs-ws/my_app/src/main.rs) file as follows:

```rust,noplayground
fn main() {
    let number_1: u64 = 2;
    let number_2: u64 = 1;

    let result: u64 = my_lib::add(number_1, number_2);
    println!(
        "Added numbers '{}' and '{}', and got the result: '{}'",
        number_1, number_2, result
    );
}
```

and we should be able to compile it and run it:

```bash
cargo run --package my_app --release
```

## Useful References

There are many important aspects to consider while developing a library crate. Some useful references are:
- [Packages, Crates, and Modules - The Book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Writing Automated Tests - The Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Making Useful Documentation Comments - The Book](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
- [The rustdoc book](https://doc.rust-lang.org/rustdoc/references.html)
