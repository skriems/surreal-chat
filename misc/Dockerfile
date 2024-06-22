FROM rust:bookworm as chef

RUN cargo install cargo-chef

WORKDIR app
FROM chef as planner 

COPY . .

RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

#Build deps (Cached)

RUN cargo chef cook --release --recipe-path recipe.json
#Build application
COPY . .

RUN cargo build --release

FROM debian:bookworm as runtime

WORKDIR /app/

COPY --from=builder /app/target/release/YOUR_BINARY

CMD ["/app/YOUR_BINARY"]
