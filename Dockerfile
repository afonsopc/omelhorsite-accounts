# First stage: build the dependencies
FROM rust AS dependency-builder

WORKDIR /app
COPY ./accounts/Cargo.toml ./

## Create a dummy main.rs to build dependencies
RUN mkdir ./src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > ./src/main.rs

RUN cargo build --release

# Second stage: build the actual application
FROM rust AS application-builder

WORKDIR /app

COPY ./accounts .

COPY --from=dependency-builder /app/target ./target
COPY --from=dependency-builder /app/Cargo.toml ./Cargo.toml

RUN SQLX_OFFLINE=true cargo build --release

RUN mv /app/target/release/accounts /compiled_binary


# Third stage: create the final image
FROM rust:slim

COPY --from=application-builder /compiled_binary /compiled_binary

CMD ["/compiled_binary"]