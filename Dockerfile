FROM rust:1.75-slim AS builder

# TODO: Check the working directory path
WORKDIR /app 

# Install dependencies for postgres and romving cache of apt packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache the dependencies
# Better alternative to use `cargo install cargo-chef`? COPY Cargo.toml Cargo.lock ./ RUN cargo chef prepare --recipe-path recipe.json 

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main()' > src/main.rs && cargo build --release && rm -rf src

# Copy the source code
COPY . .
RUN cargo build --release

# Stage 2:Minimal runtime 
# Debinan slim image is used as the base image to reduce the image size by removing unnecessary packages like docs, man pages, etc
FROM debian:bookwrom-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq1 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# TODO: Check the working directory path
WORKDIR /app

COPY --from=builder /app/target/release/rust-chat-server .

EXPOSE 8080

ENTRYPOINT [ "./rust-chat-server" ]