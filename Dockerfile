# FROM rust:latest AS builder

# WORKDIR /app
# COPY . .
# RUN cargo build --release --bin prep
# RUN mv /app/target/release/prep prep

# CMD ["./prep"]

FROM rust:bullseye as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin prep

FROM debian:bullseye
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/src/web /srv/web
COPY --from=builder /app/target/release/prep /usr/local/bin/prep
CMD ["prep"]