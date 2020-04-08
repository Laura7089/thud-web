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
RUN mkdir /app
COPY --from=builder /build/target/release/thud-web /app

RUN adduser -s /bin/false -SH thud
USER thud

ENV ROCKET_PORT = 37542
ENV THUD_SAVES_DIR = "/data"
EXPOSE 37542
WORKDIR /app
CMD ["./thud-web"]
