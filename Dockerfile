FROM rust:latest as builder

ENV PATH=/root/.cargo/bin:$PATH
WORKDIR /src

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install postgresql -y
COPY --from=builder /src/target/release/rust-starter .

EXPOSE 3000/tcp
EXPOSE 3000/udp

CMD ["./rust-starter"]
