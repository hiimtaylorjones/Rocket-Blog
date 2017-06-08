FROM liuchong/rustup:nightly

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY src ./src
COPY target ./target
RUN cargo build

WORKDIR /code
