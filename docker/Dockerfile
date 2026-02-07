FROM ubuntu:24.04

# Docker build arguments
ARG RUST_VERSION=stable

# Environment variables
ENV HOME=/root

# Set working directory
WORKDIR ${HOME}

# Install packages without prompting the user to answer any questions
ENV DEBIAN_FRONTEND="noninteractive"

# Timezone and locale
ENV TZ="Europe/Rome"
RUN apt-get update && apt-get install -y tzdata locales && \
    rm -rf /var/lib/apt/lists/*
RUN locale-gen en_US en_US.UTF-8 && \
    update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8
ENV LANG=en_US.UTF-8

# Install system dependencies required by rustup and common crates
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    ca-certificates \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Configure rustup
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install Rust via rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- \
    -y \
    --no-modify-path \
    --default-toolchain ${RUST_VERSION}

# Install additional Rust components
RUN rustup component add \
    rustfmt \
    clippy \
    rust-analyzer

# Install crates
RUN cargo install mdbook
