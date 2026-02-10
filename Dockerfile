FROM rust:1.93-trixie AS builder

ENV VGA9X16_PUBLIC=1
WORKDIR /build

COPY Cargo.toml Cargo.lock .
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch --locked
RUN cargo build --release && rm -f src/main.rs

COPY ./src .
RUN touch src/main.rs
RUN cargo build --release

FROM debian:trixie-slim AS runner

WORKDIR /app
COPY ./assets ./Rocket.toml .
COPY --from=builder /build/target/release/vga9x16 /usr/local/bin/server

EXPOSE 8000/tcp
CMD ["server"]
