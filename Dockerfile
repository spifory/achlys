FROM rust:1.77-slim-bookworm

WORKDIR /steam-currently-playing

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release --verbose && \
    cp ./target/release/steam-currently-playing ./ && rm src/*.rs

CMD [ "./steam-currently-playing" ]
