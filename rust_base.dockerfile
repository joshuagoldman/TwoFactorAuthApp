FROM rust
RUN apt-get update \
&& apt-get -y install clang \
&& apt-get install \
&& rustup target add wasm32-unknown-unknown \
&& cargo install cargo-generate trunk \
&& apt-get install libpq-dev
