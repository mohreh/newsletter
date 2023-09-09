FROM rust:alpine as builder
WORKDIR /app

RUN apk add --no-cache build-base openssl-dev

COPY . .
RUN cargo build --release --bin newsletter --target x86_64-unknown-linux-musl

FROM alpine:latest AS runtime
WORKDIR /app

RUN apk add --no-cache openssl
RUN addgroup -S app && adduser -S app -G app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/newsletter newsletter
COPY configuration configuration

RUN chown -R app:app /app
USER app

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter"]
