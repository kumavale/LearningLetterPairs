FROM rust:latest
WORKDIR /app/frontend

RUN apt update \
    && apt install -y libssl-dev pkg-config \
    && cargo install trunk wasm-bindgen-cli \
    && rustup target add wasm32-unknown-unknown

CMD ["trunk", "serve", "--address", "0.0.0.0"]
