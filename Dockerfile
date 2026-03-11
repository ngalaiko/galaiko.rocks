FROM ghcr.io/astral-sh/uv:python3.13-alpine as build
WORKDIR /app
RUN apk add --update --no-cache \
    imagemagick imagemagick-dev libjpeg-turbo-dev libpng-dev libwebp-dev \
    pandoc \
    make \
    jq \
    yq
RUN wget --quiet "https://github.com/cooklang/cookcli/releases/download/v0.10.0/cook-x86_64-unknown-linux-musl.tar.gz" \
    && sha256sum "cook-x86_64-unknown-linux-musl.tar.gz" \
    && echo "cbea3306f7a24ea2d065f6daf98c232f246c45b9f775d4b5def3989a85329a64  cook-x86_64-unknown-linux-musl.tar.gz" | sha256sum -c \
    && tar -xzf "cook-x86_64-unknown-linux-musl.tar.gz" -C /usr/bin
COPY . .
RUN make -j$(nproc)

FROM alpine:3.22
ARG S6_OVERLAY_VERSION=3.2.1.0
ADD "https://github.com/P3TERX/GeoLite.mmdb/raw/download/GeoLite2-Country.mmdb" /usr/local/share/GeoIP/GeoLite2-Country.mmdb
ARG TARGETARCH
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz" /tmp
RUN S6_ARCH=$(case "${TARGETARCH}" in "amd64") echo "x86_64" ;; "arm64") echo "aarch64" ;; *) echo "${TARGETARCH}" ;; esac) && \
	wget -O /tmp/s6-overlay-arch.tar.xz "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-${S6_ARCH}.tar.xz" && \
	tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz && \
	tar -C / -Jxpf /tmp/s6-overlay-arch.tar.xz && \
	rm -rf /tmp/*
COPY init-wrapper /
COPY etc /etc
COPY --from=build /app/build /var/www/nikita.galaiko.rocks
ENTRYPOINT ["/init-wrapper"]
