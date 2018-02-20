## -----------------------------------------------------------------------------
## Build
## -----------------------------------------------------------------------------
FROM rust:latest as build-stage

WORKDIR "/build"
COPY . .
RUN cargo build --release

## -----------------------------------------------------------------------------
## Package
## -----------------------------------------------------------------------------
FROM ubuntu:18.04

ENV DEBIAN_FRONTEND noninteractive

RUN set -xe \
    && apt-get update \
    && apt-get -y --no-install-recommends install \
        libssl1.1 \
        libpq-dev

COPY --from=build-stage "/build/target/release/signals" "/app/"
ENTRYPOINT ["/app/signals"]