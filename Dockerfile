FROM rust:slim-buster

COPY . ./rust_tokenizer
WORKDIR ./rust_tokenizer

RUN cargo build --release

ENV DB_USERNAME=postgres
ENV DB_PASSWORD=password 
ENV DB_URL='127.0.0.1' 
ENV DB_NAME=tokenizer_db
ENV DB_PORT=5443

ENTRYPOINT [ "./target/release/rust_tokenizer" ]