# Build Stage
FROM rust:latest AS builder
WORKDIR /build
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

RUN cargo install --target x86_64-unknown-linux-musl --path .

COPY config/sample_config.json ./config.json

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/btrader /app/btrader
COPY --from=builder /build/config.json /app/config.json
# FROM alpine:latest
# FROM busybox:latest
# # https://stackoverflow.com/questions/66963068/docker-alpine-executable-binary-not-found-even-if-in-path
# RUN apk add gcompat

# ENV GLIBC_REPO=https://github.com/sgerrand/alpine-pkg-glibc
# ENV GLIBC_VERSION=2.30-r0

# RUN set -ex && \
# 	apk --update add libstdc++ curl ca-certificates && \
# 	for pkg in glibc-${GLIBC_VERSION} glibc-bin-${GLIBC_VERSION}; \
# 	do curl -sSL ${GLIBC_REPO}/releases/download/${GLIBC_VERSION}/${pkg}.apk -o /tmp/${pkg}.apk; done && \
# 	apk add --allow-untrusted /tmp/*.apk && \
# 	rm -v /tmp/*.apk && \
# 	/usr/glibc-compat/sbin/ldconfig /lib /usr/glibc-compat/lib

# COPY --from=builder /build/target/release/btrader /app/btrader
# COPY --from=builder /build/config.json /app/config.json
# FROM rust:latest AS final
CMD ["/app/btrader", "/app/config.json"]
