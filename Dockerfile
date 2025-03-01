FROM rust:1.85 as builder

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
COPY --from=builder /chatguard/target/x86_64-unknown-linux-musl/release/chatguard /chatguard
CMD ["./chatguard"]