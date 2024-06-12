FROM rust:latest AS rust-build

RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates

WORKDIR /usr/src/google-sheets-crud
COPY ./ ./
RUN cargo build --release




FROM debian:bookworm-slim

WORKDIR /google-sheets-crud

COPY --from=rust-build /usr/src/google-sheets-crud/target/release/google-sheets-crud /usr/local/bin/google-sheets-crud

RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates

CMD ["/usr/local/bin/google-sheets-crud"]