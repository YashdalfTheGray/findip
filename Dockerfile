FROM clux/muslrust

WORKDIR /usr/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new findip --lib
RUN mkdir -p /usr/findip/src/bin && touch /usr/findip/src/bin/main.rs && echo "pub fn main() {}" > /usr/findip/src/bin/main.rs
WORKDIR /usr/findip
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm -rf /usr/findip/src
COPY src/ ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .
