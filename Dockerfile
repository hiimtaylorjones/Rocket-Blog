FROM liuchong/rustup:nightly

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY src ./src
RUN cargo build

WORKDIR /code
