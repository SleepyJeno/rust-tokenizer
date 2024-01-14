FROM rust:slim-buster

RUN mkdir -p /opt/rust_tokenizer
WORKDIR /opt/rust_tokenizer

COPY Cargo.* /opt/rust_tokenizer
COPY .env /opt/rust_tokenizer
COPY Dockerfile /opt/rust_tokenizer
COPY README.md /opt/rust_tokenizer
COPY src/ /opt/rust_tokenizer/src

RUN cargo build --release

ENTRYPOINT [ "./target/release/rust_tokenizer" ]