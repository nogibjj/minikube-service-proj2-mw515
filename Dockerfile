# build stage
FROM rust:latest as builder
ENV APP musicreco
WORKDIR /usr/src/$APP
COPY . .
RUN cargo build --release

# runtime stage
FROM debian:buster-slim
ENV APP musicreco
COPY --from=builder /usr/src/$APP/target/release/minikubemusicreco /usr/local/bin/$App
CMD ["musicreco"]
