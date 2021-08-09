FROM rust:1.54.0

WORKDIR /app
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./ ./
RUN cargo build --release
RUN cargo install cargo-edit
RUN cargo install sccache
RUN cargo install diesel_cli --no-default-features --features mysql
ENV RUSTC_WRAPPER /usr/local/cargo/bin/sccache