FROM ubuntu:18.04
MAINTAINER Vytautas Astrauskas "vastrauskas@gmail.com"

ENV DEBIAN_FRONTEND noninteractive

# Install prerequisites
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        cmake \
        curl \
        file \
        gcc \
        git \
        libssl-dev \
        locales \
        pkg-config \
        python \
        unzip \
        wget \
    && \
    rm -rf /var/lib/apt/lists/*

# Set up locale
RUN locale-gen en_US.UTF-8
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# Install Rust
ENV RUST_TOOLCHAIN "nightly-2019-08-13"
ENV RUSTUP_HOME /usr/local/rustup
ENV CARGO_HOME /usr/local/cargo
ENV PATH /usr/local/cargo/bin:$PATH
# https://github.com/rust-lang-nursery/rustup.rs/issues/998
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain "$RUST_TOOLCHAIN" && \
    rm -rf ~/.rustup/toolchains/*/share/doc

# Install Prusti.
ADD . /tmp/prusti-dev
RUN cd /tmp/prusti-dev && \
    cargo build --release --manifest-path example_contracts_impl/Cargo.toml && \
    cargo build --release --manifest-path example_verification_tool/Cargo.toml && \
    mkdir -p /usr/local/lib/prusti && \
    cp example_contracts_impl/target/release/libexample_contracts_impl.so /usr/local/lib/prusti/ && \
	cp example_verification_tool/target/release/rustc-tool /usr/local/bin/rustc-tool

# Set up workdir.
ENV USER root
RUN cd / && \
    cargo new playground
WORKDIR /playground

ADD docker/entrypoint.sh /root/
ENTRYPOINT ["/root/entrypoint.sh"]

# Prepare env variables to run Prusti
RUN echo '#/bin/bash' > /usr/local/bin/prusti && \
    echo 'export RUST_CONTRACTS_LIB=/usr/local/lib/prusti/libexample_contracts_impl.so' >> /usr/local/bin/prusti && \
    echo '/usr/local/bin/rustc-tool "$@"' >> /usr/local/bin/prusti && \
    chmod 755 /usr/local/bin/prusti
ENV RUSTC_WRAPPER /usr/local/bin/prusti

# Reduce log level
ENV RUST_LOG warn

# Pre-build dependencies
ADD docker/Cargo.toml /playground/Cargo.toml
RUN cd /playground && \
    cargo build && \
    cargo build --release
