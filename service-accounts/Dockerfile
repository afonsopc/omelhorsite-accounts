FROM rust:latest

WORKDIR /
COPY . /app

RUN cargo install --path /app

RUN rm -rf /app

CMD ["service-accounts"]