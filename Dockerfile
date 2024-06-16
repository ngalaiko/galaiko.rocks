FROM python:3.11-alpine3.20 as build
WORKDIR /app
COPY requirements.txt requirements.txt
RUN apk add --update --no-cache \
    imagemagick imagemagick-dev libjpeg-turbo-dev libpng-dev libwebp-dev \
    pandoc \
    make \
    jq \
    yq \
    \
    && pip install --break-system-packages -r requirements.txt \
    \
    && wget --quiet "https://github.com/cooklang/cookcli/releases/download/v0.8.0/cook-x86_64-unknown-linux-musl.tar.gz" \
    && sha256sum "cook-x86_64-unknown-linux-musl.tar.gz" \
    && echo "4e1b95202d92b492027a5df2f78624679f93f368a9b5832e2ec94f518890f130  cook-x86_64-unknown-linux-musl.tar.gz" | sha256sum -c \
    && tar -xzf "cook-x86_64-unknown-linux-musl.tar.gz" -C /usr/bin
COPY . .
RUN make


FROM alpine:3.20
ARG S6_OVERLAY_VERSION=3.2.0.0
ARG REPROXY_VERSION=1.2.2
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz" /tmp
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-aarch64.tar.xz" /tmp
ADD "https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz" /tmp
ADD "https://github.com/umputun/reproxy/releases/download/v${REPROXY_VERSION}/reproxy_v${REPROXY_VERSION}_linux_arm64.tar.gz" /tmp
ADD "https://github.com/umputun/reproxy/releases/download/v${REPROXY_VERSION}/reproxy_v${REPROXY_VERSION}_linux_amd64.tar.gz" /tmp
ADD "https://github.com/P3TERX/GeoLite.mmdb/releases/download/2024.06.16/GeoLite2-Country.mmdb" /usr/local/share/GeoIP/GeoLite2-Country.mmdb
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
            \
            sha256sum "/tmp/reproxy_v${REPROXY_VERSION}_linux_amd64.tar.gz"; \
            echo "3a9c9d0d1dcb266967858eb0502473b461d5c8443f8f427e08a104a69a456931  /tmp/reproxy_v${REPROXY_VERSION}_linux_amd64.tar.gz" | sha256sum -c; \
            mkdir -p "/tmp/reproxy_v${REPROXY_VERSION}_linux_amd64"; \
            tar -C "/tmp/reproxy_v${REPROXY_VERSION}_linux_amd64" -xzf "/tmp/reproxy_v${REPROXY_VERSION}_linux_amd64.tar.gz"; \
            mv "/tmp/reproxy_v${REPROXY_VERSION}_linux_amd64/reproxy" /usr/bin/reproxy; \
            ;; \
        "aarch64") \
            sha256sum "/tmp/s6-overlay-aarch64.tar.xz"; \
            echo "868973e98210257bba725ff5b17aa092008c9a8e5174499e38ba611a8fc7e473  /tmp/s6-overlay-aarch64.tar.xz" | sha256sum -c; \
            tar -C / -Jxpf /tmp/s6-overlay-aarch64.tar.xz; \
            \
            sha256sum "/tmp/reproxy_v${REPROXY_VERSION}_linux_arm64.tar.gz"; \
            echo "3a9c9d0d1dcb266967858eb0502473b461d5c8443f8f427e08a104a69a456931  /tmp/reproxy_v${REPROXY_VERSION}_linux_arm64.tar.gz" | sha256sum -c; \
            mkdir -p "/tmp/reproxy_v${REPROXY_VERSION}_linux_arm64"; \
            tar -C "/tmp/reproxy_v${REPROXY_VERSION}_linux_arm64" -xzf "/tmp/reproxy_v${REPROXY_VERSION}_linux_arm64.tar.gz"; \
            mv "/tmp/reproxy_v${REPROXY_VERSION}_linux_arm64/reproxy" /usr/bin/reproxy; \
            ;; \
        *) \
          echo "Cannot build, missing valid build platform." \
          exit 1; \
    esac; \
    rm -rf "/tmp/*"; \
    apk add --update --no-cache goaccess;
COPY --from=build /app/build /var/www/galaiko.rocks
COPY etc /etc
COPY init-wrapper /
ENTRYPOINT ["/init-wrapper"]
