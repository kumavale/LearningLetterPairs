FROM rustlang/rust:nightly-slim as builder

RUN apt update -y \
    && apt install -y \
        build-essential \
        pkg-config \
        libssl-dev

RUN USER=root cargo new --bin app
WORKDIR /app

# Install dependencies first for cache
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo +nightly build --release
RUN rm -rf ./src

# Build my app
COPY ./src ./src
COPY ./templates ./templates
RUN touch -a -m ./src/main.rs
RUN cargo +nightly build --release

# Final image
FROM rustlang/rust:nightly-slim

COPY ./css ./css
COPY ./script ./script
COPY ./resources ./resources
COPY --from=builder /app/target/release/learning_letter_pairs .

CMD ["./learning_letter_pairs"]
