FROM rust:latest as builder

RUN cargo new --bin api-new
WORKDIR ./api-new
COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /api-new/target/release/api-new ./api-new
RUN apt update && apt install libpq-dev -y
CMD ["./api-new"]