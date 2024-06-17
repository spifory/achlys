FROM rust:1.77-slim-bookworm

WORKDIR /achlys

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release --verbose && \
    cp ./target/release/achlys ./ && rm src/*.rs

CMD [ "./achlys" ]
