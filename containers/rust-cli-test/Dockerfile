FROM rust:1.97.1-alpine3.23@sha256:c4a364ddbf684fe038e6fa6a4f25b30c8dc85247423e0e660676ece0d17be4a2

RUN apk add --no-cache build-base git perl
RUN rustup component add clippy

WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/arcantry-core/Cargo.toml crates/arcantry-core/Cargo.toml
COPY crates/arcantry-cli/Cargo.toml crates/arcantry-cli/Cargo.toml
COPY xtask/Cargo.toml xtask/Cargo.toml
RUN mkdir -p crates/arcantry-core/src crates/arcantry-cli/src xtask/src \
    && touch crates/arcantry-core/src/lib.rs crates/arcantry-cli/src/lib.rs \
      crates/arcantry-cli/src/main.rs xtask/src/main.rs \
    && cargo fetch --locked
COPY . .

CMD ["tail", "-f", "/dev/null"]
