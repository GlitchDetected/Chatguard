FROM rust:1.85 as builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./

RUN mkdir src && echo "fn main {}" > src/main.rs
RUN cargo build --release

FROM scratch
COPY ./src ./src
RUN cargo build --release

CMD ./target/release/chatguard