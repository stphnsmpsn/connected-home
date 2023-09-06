FROM rust:1.71.1-slim-bullseye
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    iputils-ping \
    libpq-dev \
    cmake \
    pkg-config \
    gcc \
    g++ \
    python3 \
    libssl-dev \
    protobuf-compiler \
    git
WORKDIR /app
RUN rustup component add rustfmt clippy
RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install mdbook mdbook-mermaid
CMD cargo build