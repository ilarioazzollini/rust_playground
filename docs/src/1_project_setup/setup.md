# Development Environment Setup

As mentioned earlier, we will rely on Docker to containerize our development environment. This approach ensures that everyone works in the same reproducible environment, regardless of their host operating system.

The machine on which Docker is installed — and from which we build Docker images and run Docker containers — will be referred to as the *host PC*. Regardless of the host PC's operating system, all development work in this book will take place inside an **Ubuntu Docker container**.

## 1. Clone the repository

To get started, open a terminal and navigate to the directory where you want to clone this repository, then run:

```bash
git clone https://github.com/ilarioazzollini/rust_playground.git
cd rust_playground
```

>  From this point on, **all terminal commands in this guide should be executed from the root directory of this repository unless otherwise specified**.

## 2. Build the Docker container image

Now, we can build a new Docker container image by running:

```bash
docker build -f docker/Dockerfile -t rust_playground:latest .
```

If the build completes successfully, the image should appear in the list of available images:

```bash
docker image ls
```

If you want more details about this image, you can refer to the file [`rust_playground/docker/Dockerfile`](https://github.com/ilarioazzollini/rust_playground/blob/main/docker/Dockerfile)

## 3. Run a Docker container

Next, run a container from the image we just built:

```bash
docker run -it --rm --privileged --network=host -v .:/root/rust_playground -w /root/rust_playground --name rust-dev-cont rust_playground:latest bash
```

You should now be inside a terminal session running in the Docker container.

If you want to check the container's status, open another terminal on the host PC and run:

```bash
docker container ls
```

You should see a running container named `rust-dev-cont`.

Now return to the terminal inside the container. If everything worked correctly, your current working directory should be `/root/rust_playground`. You can verify this by running:

```bash
pwd
```

You should also see the contents of the rust_playground repository by running:

```bash
ls
```

This works because the repository directory on the host PC is mounted inside the container using the volume flag `-v .:/root/rust_playground`. This means the folder is shared between the host and the container.

We can also verify that the Rust toolchain is correctly installed inside the container:

```bash
cargo --version
rustc --version
```

When you are done, you can exit the container by running:

```bash
exit
```

> At this point, the development environment is fully functional and we can start writing and running Rust code.

## 4. VS Code with Dev Containers

A fully integrated development setup can be achieved using VS Code and the Dev Containers extension.

First of all, we need to open a terminal and install (or update) the required VS Code extensions:

```bash
code --force --install-extension ms-azuretools.vscode-containers
code --force --install-extension ms-vscode-remote.remote-containers
```

Then, navigate to the root directory of the `rust_playground` repository:

```bash
cd <rust_playground>
```

And open the project in VS Code:

```bash
code .
```

Open the Command Palette (`Ctrl+Shift+P`, or `Cmd+Shift+P` on Mac OS), type

```
Dev Containers: Reopen in Container
```

and select it.

VS Code will close the current window, start a `rust_playground:latest` Docker container, and reopen the project inside the container. During this process, VS Code will also install the extensions defined in the file [`rust_playground/.devcontainer/devcontainer.json`](https://github.com/ilarioazzollini/rust_playground/blob/main/.devcontainer/devcontainer.json).

This setup provides useful development features such as:

- IntelliSense
- inline error reporting
- automatic formatting
- integrated debugging tools

Any terminal opened from this VS Code instance will automatically run inside the container.

> If you modify the Docker image and rebuild it, remember to also rebuild the Dev Container. In that case, use `Dev Containers: Rebuild Without Cache and Reopen in Container` from the Command Palette.
