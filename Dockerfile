FROM rust:1.77 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
	--mount=type=cache,target=/var/cache/apt,sharing=locked \
	rm -f /etc/apt/apt.conf.d/docker-clean \
	&& apt-get update \
	&& apt-get -y --no-install-recommends install musl-tools=1.2.3-1

ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl \
	PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/app/target \
	--mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	--mount=type=cache,target=/usr/local/rustup \
	rustup install stable \
	&& rustup target add ${CARGO_BUILD_TARGET} \
	&& cargo run --package convert --release \
	&& cargo build --package serve --release \
	&& mkdir -p /build \
	&& cp /app/target/${CARGO_BUILD_TARGET}/release/serve /build/ \
	&& strip /build/serve


FROM scratch

EXPOSE 8080

COPY --from=build /build/ /

ENTRYPOINT ["/serve"]
CMD ["--address", "0.0.0.0:8080"]
