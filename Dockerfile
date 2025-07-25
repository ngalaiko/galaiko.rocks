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
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz" /tmp
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-aarch64.tar.xz" /tmp
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz" /tmp
ADD "https://github.com/P3TERX/GeoLite.mmdb/raw/download/GeoLite2-Country.mmdb" /usr/local/share/GeoIP/GeoLite2-Country.mmdb
RUN \
    sha256sum "/tmp/s6-overlay-noarch.tar.xz"; \
    echo "4b0c0907e6762814c31850e0e6c6762c385571d4656eb8725852b0b1586713b6  /tmp/s6-overlay-noarch.tar.xz" | sha256sum -c; \
    tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz; \
    \
    case "$(uname -m)" in \
        "x86_64") \
            sha256sum "/tmp/s6-overlay-x86_64.tar.xz"; \
            echo "868973e98210257bba725ff5b17aa092008c9a8e5174499e38ba611a8fc7e473  /tmp/s6-overlay-x86_64.tar.xz" | sha256sum -c; \
            tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz; \
            ;; \
        "aarch64") \
            sha256sum "/tmp/s6-overlay-aarch64.tar.xz"; \
            echo "868973e98210257bba725ff5b17aa092008c9a8e5174499e38ba611a8fc7e473  /tmp/s6-overlay-aarch64.tar.xz" | sha256sum -c; \
            tar -C / -Jxpf /tmp/s6-overlay-aarch64.tar.xz; \
            ;; \
        *) \
          echo "Cannot build, missing valid build platform." \
          exit 1; \
    esac; \
    rm -rf "/tmp/*"; \
    apk add --update --no-cache goaccess=1.9.4-r0 nginx=1.28.0-r3
COPY --from=build /app/build /var/www/nikita.galaiko.rocks
COPY etc /etc
COPY init-wrapper /
ENTRYPOINT ["/init-wrapper"]
