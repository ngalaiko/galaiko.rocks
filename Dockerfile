FROM rust:1.75 as build

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl

ENV DEBIAN_FRONTEND=noninteractive
RUN \
	apt-get update && \
	apt-get -y --no-install-recommends install musl-tools && \
	rustup target add ${CARGO_BUILD_TARGET}

ENV PKG_CONFIG_ALLOW_CROSS=1

COPY ./ ./

RUN cargo build --package serve --release

RUN \
	mkdir -p /build && \
	cp target/${CARGO_BUILD_TARGET}/release/serve /build/ && \
	strip /build/serve

FROM scratch

EXPOSE 8080

COPY --from=build /build/ /

ENTRYPOINT ["/serve"]
CMD ["--address", "0.0.0.0:8080"]
