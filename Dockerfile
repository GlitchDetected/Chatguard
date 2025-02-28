FROM rust:latest

WORKDIR /app

COPY Cargo.lock Cargo.toml ./

COPY . .

CMD ["cargo", "run"]