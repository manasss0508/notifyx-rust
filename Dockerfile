FROM rust:1.89 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin api
RUN cargo build --release --bin worker_mail


FROM debian:bookworm-slim AS api

WORKDIR /app
COPY --from=builder /app/target/release/api /app/api

EXPOSE 3000

CMD ["/app/api"]


FROM debian:bookworm-slim AS worker

WORKDIR /app
COPY --from=builder /app/target/release/worker_mail /app/worker_mail

CMD ["/app/worker_mail"]