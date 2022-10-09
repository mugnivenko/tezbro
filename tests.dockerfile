FROM rust:1.64-slim-buster as build
COPY . .
COPY ./tests.env ./.env
RUN cargo test --no-run

FROM debian:buster-slim
COPY --from=build ./target/debug/deps ./target/debug/
COPY ./tests.env ./.env
CMD run-parts ./target/debug/deps
