From rustlang/rust:nightly-slim
COPY ./ /
RUN apt update -y \
    && apt install -y \
        build-essential \
        pkg-config \
        libssl-dev \
    && cargo +nightly build
CMD ["cargo", "+nightly", "run"]
