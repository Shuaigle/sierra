FROM rustlang/rust:nightly-bullseye as builder

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install --locked cargo-leptos

RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

# TODO optimize build speed and image size
COPY . .

ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
RUN cargo update
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/sierra /app/target/release/sierra
COPY --from=builder /app/target/site /app/target/site


ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_TAILWIND_VERSION=v3.4.3
ENV APP_ENVIRONMENT="production"

EXPOSE 3000

CMD ["./target/release/sierra"]
