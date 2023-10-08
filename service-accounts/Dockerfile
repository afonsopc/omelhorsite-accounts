FROM rust:latest

WORKDIR /usr/src/service-accounts
COPY . .

RUN cargo install --path .

WORKDIR /
RUN rm -rf /usr/src/service-accounts

CMD ["service-accounts"]