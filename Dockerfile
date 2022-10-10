FROM rust:1.61-slim

WORKDIR /usr/src/api
COPY src/ src/
COPY Cargo.toml .

RUN cargo build --release

CMD ["./target/release/ner"]