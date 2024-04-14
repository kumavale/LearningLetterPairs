# フロントエンド

## 開発環境
FROM rust:latest
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

CMD ["trunk", "serve", "--address", "0.0.0.0"]
