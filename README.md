# rust_playground

All the terminal commands in this readme should be launched from the root directory of this repository, unless otherwise specified.

In order to get started, we should open a terminal, navigate to our favorite folder where we want to clone this repo, and simply:

```bash
git clone https://github.com/ilarioazzollini/rust_playground.git && cd rust_playground
```

## 1. Build the Docker container image

First of all, we can **optionally** choose a specific Rust version we want to work with:

```bash
export RUST_VERSION=1.92.0
```

If no version is explicitly set, the latest `stable` version of Rust will be used.

Then, we can build the Docker container image either with a Rust version

```bash
docker build \
    --build-arg RUST_VERSION=${RUST_VERSION} \
    -f docker/Dockerfile \
    -t rust_playground:${RUST_VERSION} \
    .
```

or without it

```bash
docker build \
    -f docker/Dockerfile \
    -t rust_playground:latest \
    .
```

## 2. Run the Docker container

The following command can be used to run a container from the image we built in the previous section. We can substitute `rust_playground:${RUST_VERSION}` in the following command if we want to use a different tag (for instance `rust_playground:latest`).

```bash
docker run \
    -it \
    --rm \
    --privileged \
    --network=host \
    -v .:/root/rust_playground \
    -w /root/rust_playground \
    rust_playground:${RUST_VERSION} \
    bash
```

> If we want to launch a complete development environment using Visual Studio Code instead, remember to change the image field accordingly in the `.devcontainer/devcontainer.json` file:
> ```
> "image": "rust_playground:1.92.0",
> ```

## 3. Rust project development

In this section we are going to assume that we are in a terminal inside a running Docker container (as explained in the previous section). In order to handle arbitrarily big projects, we are going to organize this repo as a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), which can greatly help managing multiple related packages that are developed in tandem.

### 3.1 Build the entire project

We can build the whole project (workspace) by simply:

```bash
cargo build --workspace --all-targets
```

### 3.2 Binary crates

Add a new binary crate by:

```bash
cd rs-ws
cargo new hello_world --bin
```

and run it by:
```bash
cargo run --bin hello_world
```

### 3.3 Add a library crate

Add a new library crate by:

```bash
cd rs-ws
cargo new utilities --lib
```

### 3.3 Run the tests

Run all unit tests and integration tests by simply:

```bash
cargo test --workspace --all-targets
```

### 3.4 Coverage

```bash
cargo llvm-cov --workspace --lib --all-features
```

### 3.5 Generate the docs

Generate and immediately open the documentation

```bash
cargo doc --workspace --no-deps --open
```

### 3.6 The markdown book

In order to create our book the first time

```bash
mkdir docs
cd docs
mdbook init
```

Build the book, and then serve it

```bash
cd rust_playground
rm -rf docs/book
mdbook build docs
python3 -m http.server 8080 -d /root/rust_playground/docs/book
```

Or simply manually open the file `/root/rust_playground/docs/book/index.html`.

I am taking inspiration from [The Rust Programming Language Book](https://github.com/rust-lang/book/tree/main).

## 4. The Rust Language Book

We can open our offline version of *The Rust Language Book* from inside the container by simply

```bash
python3 -m http.server 8080 -d $(rustup doc --book --path | xargs dirname)
```
