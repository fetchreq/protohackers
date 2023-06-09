FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build the project dependencies, not the application
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build the project
RUN cargo build --release --bin smoke_test

FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean Up
    && apt-get autoremove \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/smoke_test smoke_test
COPY configuration configuration 
ENV APP_ENV prod
ENTRYPOINT [ "./smoke_test" ]

