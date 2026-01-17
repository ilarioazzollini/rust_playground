# rust-playground

All the terminal commands in this readme should be launched from the root directory of this repository, unless otherwise specified. Therefore, first of all we should open a terminal, navigate to our favorite folder where we want to clone this repo, and simply:q

```bash
git clone https://github.com/ilarioazzollini/rust-playground.git && cd rust-playground
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
    -t rust-playground:${RUST_VERSION} \
    .
```

or without it

```bash
docker build \
    -f docker/Dockerfile \
    -t rust-playground:latest \
    .
```

## 2. Run the Docker container

The following command can be used to run a container from the image we built in the previous section. We can substitute `rust-playground:latest` with `rust-playground:${RUST_VERSION}` if we want to run a container from a different image (always assuming that we already have that image locally).

```bash
docker run \
    -it \
    --rm \
    --privileged \
    --network=host \
    -v .:/root/rust-playground \
    -w /root/rust-playground \
    rust-playground:latest \
    bash
```

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
cargo run --bin rust-playground
```

In a similar way, we can run any other binary executable application. For instance, we can run the one associated with the source file `src/bin/hello_world.rs` by:

```bash
cargo run --bin hello_world
```
