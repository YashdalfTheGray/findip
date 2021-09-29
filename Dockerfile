FROM clux/muslrust:stable as builder

WORKDIR /usr/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new findip --lib
WORKDIR /usr/findip

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src

RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/build

FROM scratch
ARG CONFIG_FILE=testfiles/stdout.yml
COPY --from=builder /usr/build/bin/findip .
COPY $CONFIG_FILE ./findip-config.yml
USER 1000
ENTRYPOINT ["./findip", "--config-file-name", "findip-config.yml"]