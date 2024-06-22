FROM rust:1.78 as base

# install dependencies for building rdkafka
RUN apt-get update && apt-get install -y \
    make \
    cmake \
    # pthreads
    libc6-dev \
    # zlib for rdkafka's libz feature
    zlib1g-dev \
    # for rdkafka's ssl feature
    libssl-dev \
    # for rdkafka's gssapi feature
    libsasl2-dev \
    # for rdkafka's zstd-pkg-config feature
    libzstd-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

WORKDIR /workspace

COPY . .
RUN cargo build --bin warp-chat
CMD ["cargo", "watch", "-x", "run", "--bin", "warp-chat"]
EXPOSE 3000

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]
