FROM liuchong/rustup:nightly

RUN apt-get update && \
    apt-get install -y libpq-dev

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY Rocket.toml ./Rocket.toml
COPY src ./src
COPY templates ./templates
COPY .env ./.env

RUN cargo build
RUN cargo install diesel_cli --no-default-features --features postgres
