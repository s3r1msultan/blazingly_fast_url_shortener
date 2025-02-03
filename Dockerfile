FROM rust:1.82 AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

COPY Cargo.toml ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN cargo build --release --verbose || true && rm -rf src

COPY . .

RUN cargo build --release --verbose

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rxstme .

COPY migrations /app/migrations

COPY static /app/static

EXPOSE 8080

ENV DATABASE_URL=sqlite:/app/url_shortener.db
ENV RUST_LOG=info

CMD ["./rxstme"]
