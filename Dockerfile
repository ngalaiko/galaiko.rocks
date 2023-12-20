FROM rust:1.74-slim-bookworm as builder
WORKDIR /build
COPY . .
RUN cargo build \
    --package serve \
    --release

FROM debian:bookworm-slim
COPY --from=builder /build/target/release/serve /serve
CMD ["/serve", "--address", "0.0.0.0:8080"]
