FROM clux/muslrust:stable as builder

WORKDIR /usr/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new findip --lib
WORKDIR /usr/findip

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src

RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/build

FROM scratch
COPY --from=builder /usr/build/bin/findip .
COPY testfiles/stdout.yml ./findip-config.yml
USER 1000
CMD ["./findip", "--config-file-name", "findip-config.yml"]
