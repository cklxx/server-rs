FROM rust:latest as builder

ENV PATH=/root/.cargo/bin:$PATH
WORKDIR /src

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install postgresql libc6 -y
COPY --from=builder /src/target/release/rust-starter .

EXPOSE 8080/tcp
EXPOSE 8080/udp

CMD ["./rust-starter"]
