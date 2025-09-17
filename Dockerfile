FROM rust:1.89-bookworm AS builder

ENV VGA9X16_PUBLIC=1
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch --locked
RUN cargo build --release
RUN rm src/main.rs

COPY . .
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY . .
COPY --from=builder /app/target/release/vga9x16 /usr/local/bin/server

EXPOSE 8000/tcp
CMD ["server"]
