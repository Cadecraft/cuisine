from lukemathwalker/cargo-chef:latest-rust-1 as chef

# Based on <https://docs.railway.com/guides/axum#use-a-dockerfile>

# Create and change to the app directory
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies (caching Docker layer)
RUN cargo chef cook --release --recipe-path recipe.json

# Env
ARG HOST
ARG PORT
ARG KV_PATH
ARG ADMIN_AUTH

# Build application
COPY . ./
RUN cargo build --release

CMD ["./target/release/cuisine"]
