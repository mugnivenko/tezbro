FROM rust:1.64
COPY ./migrations .
RUN cargo install sqlx-cli --locked
COPY ./tests.env ./.env
CMD sqlx migrate run
