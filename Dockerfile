FROM liuchong/rustup:nightly

RUN apt-get update && \
    apt-get install -y libpq-dev

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY src ./src
COPY .env ./.env

RUN rustup update && cargo update
RUN cargo install diesel_cli --no-default-features --features postgres
