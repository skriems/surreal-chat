FROM rust:1.78 as base
RUN cargo install cargo-watch

WORKDIR /workspace

COPY . .
RUN cargo build --bin surreal-events
CMD ["app"]
EXPOSE 8000

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]
