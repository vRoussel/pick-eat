FROM --platform=$BUILDPLATFORM rust:1.68 as builder

RUN apt update && apt upgrade -y
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/arm64") \
        echo aarch64-unknown-linux-gnu > /rust_target.txt ; \
        apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross; \
        rustup target add aarch64-unknown-linux-gnu; \
        ;; \
    "linux/amd64") \
        echo x86_64-unknown-linux-gnu > /rust_target.txt ; \
        ;; \
    *) exit 1;; \
esac

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

WORKDIR /usr/src/pickeat/
COPY src/ src/
COPY Cargo.lock Cargo.toml .
COPY sqlx-data.json .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=./target \
    cargo install --path . --target $(cat /rust_target.txt)


FROM --platform=$TARGETPLATFORM scratch AS export
COPY --from=builder /usr/local/cargo/bin/pickeat /
