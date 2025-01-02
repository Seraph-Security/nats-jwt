FROM rust:1.79

RUN rustup override set nightly

# For running Taskfiles
RUN curl -sL https://taskfile.dev/install.sh | sh

# release-plz has a lot of dependencies and takes a while to build, hence the ci-image
RUN cargo install --locked --version 0.3.111 release-plz
