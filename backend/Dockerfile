FROM rust:latest
WORKDIR /app/backend

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/target \
    cargo build --release

CMD ["/app/backend/target/release/backend"]
