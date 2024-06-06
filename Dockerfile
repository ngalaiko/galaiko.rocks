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

FROM ghcr.io/umputun/reproxy:v1.2.2
COPY --from=build /app/build /www
EXPOSE 8080
USER app
ENTRYPOINT ["/srv/reproxy", "--assets.location=/www", "--listen=0.0.0.0:8080"]
