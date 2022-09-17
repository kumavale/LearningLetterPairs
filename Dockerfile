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
COPY ./css ./css
COPY ./templates ./templates
RUN touch -a -m ./src/main.rs
RUN cargo +nightly build --release

# Final image
FROM rustlang/rust:nightly-slim

COPY ./css ./css
COPY ./resources ./resources
COPY --from=builder /app/target/release/LearningLetterPairs .

CMD ["./LearningLetterPairs"]
