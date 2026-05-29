FROM rust:slim AS builder

WORKDIR /src
COPY src/main.rs .
RUN rustc -O main.rs -o /usr/local/bin/rust-primes

FROM debian:bookworm-slim

ENV LIMIT=100000

RUN useradd --create-home --shell /usr/sbin/nologin appuser
USER appuser

COPY --from=builder /usr/local/bin/rust-primes /usr/local/bin/rust-primes

CMD ["rust-primes"]
