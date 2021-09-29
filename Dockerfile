FROM clux/muslrust:stable as base

RUN cargo install cargo-chef
WORKDIR /usr/app

FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path findip-recipe.json

FROM base as builder
COPY --from=planner /usr/app/findip-recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/build

FROM scratch
COPY --from=builder /usr/build/bin/findip .
COPY testfiles/stdout.yml ./findip-config.yml
USER 1000
CMD ["./findip", "--config-file-name", "findip-config.yml"]
