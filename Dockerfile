FROM rust:1.85 as builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./

RUN cargo build --release

FROM scratch

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/chatguard .

CMD ["./chatguard"]