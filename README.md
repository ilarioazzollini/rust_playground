# rust_playground

All the terminal commands in this readme should be launched from the root directory of this repository, unless otherwise specified. Therefore, first of all we should open a terminal, navigate to our favorite folder where we want to clone this repo, and simply:q

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

In this section we are going to assume that we are in a terminal inside a running Docker container (as explained in the previous section).

### 3.1 Build the entire project

We can build our current version of the whole project (both the lib and all the available binaries) by:

```bash
cargo build
```

### 3.2 Run a binary executable application

We can run the main binary executable (associated to `src/main.rs`) by simply:

```bash
cargo run --bin rust_playground
```

In a similar way, we can run any other binary executable application. For instance, we can run the one associated with the source file `src/bin/hello_world.rs` by:

```bash
cargo run --bin hello_world
```

### 3.3 Run the tests

Run both unit tests and integration tests...

```bash
cargo test
```

### 3.4 Generate the docs

Generate and immediately open the documentation

```bash
cargo doc
```

### 3.5 The markdown book

In order to create our book the first time

```bash
mkdir docs
cd docs
mdbook init
```

Build both the book and docs

```bash
cd rust_playground
cargo doc
cp -r target/doc docs/doc
cd docs
mdbook build
```

And then open `docs/book/index.html`.
