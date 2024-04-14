# バックエンド

## 開発環境
FROM rust:latest as develop
WORKDIR /app

COPY . .
RUN cargo install cargo-watch

## ビルド環境
FROM rust:latest as builder
WORKDIR /app

RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release

## デプロイ環境
FROM rust:1.77.2-slim
COPY --from=builder /app/target/release/utaha .
CMD ["/usr/local/bin/utaha"]
