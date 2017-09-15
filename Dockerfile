FROM liuchong/rustup:nightly

RUN apt-get update && \
    apt-get install -y libpq-dev

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY src ./src

RUN cargo build

WORKDIR /code
