
FROM rust:1.60 as build

RUN USER=root cargo new --bin rust_clipstash
WORKDIR /holy

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --bin httpd --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/rust_clipstash*
RUN cargo build --bin httpd --release

FROM debian:buster-slim
COPY --from=build /rust_clipstash/target/release/rust_clipstash .

CMD ["./holy"]