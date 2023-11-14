# Build the app

FROM rust AS builder

ENV BINARY accounts

WORKDIR /app
COPY ./accounts .

RUN cargo build --release

RUN mv /app/target/release/$BINARY /rust_compiled_binary


# Run the app

FROM rust:slim

COPY --from=builder /rust_compiled_binary /rust_compiled_binary

CMD ["/rust_compiled_binary"]