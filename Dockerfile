# building
FROM rust:1.85 as builder

WORKDIR /app

# accept the build argument
# ARG DATABASE_URL

# ENV DATABASE_URL=$DATABASE_URL

COPY . . 

RUN cargo build --release

# production
FROM ubuntu

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/chatguard .

CMD ["./chatguard"]