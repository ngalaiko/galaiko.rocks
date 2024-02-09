FROM rust:1.74 as build

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl \
    DEBIAN_FRONTEND=noninteractive \
    PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    apt-get update \
	&& apt-get -y --no-install-recommends install musl-tools \
    && rustup install stable \
	&& rustup target add ${CARGO_BUILD_TARGET} \
    && cargo build --package serve --release \
	&& mkdir -p /build \
	&& cp /app/target/${CARGO_BUILD_TARGET}/release/serve /build/ \
	&& strip /build/serve


FROM scratch

EXPOSE 8080

COPY --from=build /build/ /

ENTRYPOINT ["/serve"]
CMD ["--address", "0.0.0.0:8080"]
