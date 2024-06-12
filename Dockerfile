FROM rust:latest AS rust-build

RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates

WORKDIR /usr/src/google-sheets-curd
COPY ./ ./
RUN cargo build --release




FROM debian:bookworm-slim

WORKDIR /google-sheets-curd

COPY --from=rust-build /usr/src/google-sheets-curd/target/release/google-sheets-curd /usr/local/bin/google-sheets-curd

RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates

CMD ["/usr/local/bin/google-sheets-curd"]