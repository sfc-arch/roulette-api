FROM rust:1.67 as builder
WORKDIR /usr/src/roulette-api
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/roulette-api /usr/local/bin/roulette-api
CMD ["roulette-api"]
