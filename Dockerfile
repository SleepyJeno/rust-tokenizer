FROM rust:slim-buster

COPY . ./rust_tokenizer
WORKDIR ./rust_tokenizer

RUN cargo build --release

ENTRYPOINT [ "./target/release/rust_tokenizer" ]