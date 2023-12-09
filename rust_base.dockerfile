FROM rust:latest
RUN apt-get update
RUN apt-get -y install clang
RUN apt-get install
RUn rustup target add wasm32-unknown-unknown 
RUN cargo install cargo-generate trunk 
RUN apt-get install libpq-dev
