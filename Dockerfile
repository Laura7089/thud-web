# Get a nightly version of rust to build with
FROM rust:stretch AS builder
RUN rustup default nightly

RUN apt update && \
    apt install -y --no-install-recommends \
    perl

COPY . /build
WORKDIR /build
RUN cargo build --release --locked --verbose

FROM alpine AS runner
ENV ROCKET_PORT = 37542
ENV THUD_SAVES_DIR = "/data"

RUN adduser -s /bin/false -SH thud && \
        mkdir /app /data && \
        chown -R thud:thud /app /data

USER thud

COPY --from=builder /build/target/release/thud-web /app

EXPOSE 37542
VOLUME /data
WORKDIR /app
CMD ["./thud-web"]
