FROM rust:1.74 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN cargo build --release && rm -rf src

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rxstme .

COPY url_shortener.db /app/url_shortener.db

COPY migrations /app/migrations

COPY static /app/static

COPY .env /app/.env

EXPOSE 8080

ENV DATABASE_URL=sqlite:/app/url_shortener.db
ENV RUST_LOG=info

CMD ["./rxstme"]
