FROM rust:1.62.0 as rust

RUN rustup component add clippy rustfmt
WORKDIR /app
COPY Cargo.toml Cargo.lock .rustfmt.toml ./
COPY src ./src
RUN cargo build --release
RUN cargo clippy
RUN cargo test
RUN cargo fmt -- --check

FROM debian:buster-slim
WORKDIR /app
COPY --from=rust /app/target/release/uniqtoo ./

CMD '/app/uniqtoo'
