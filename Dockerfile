FROM liuchong/rustup:nightly

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY src ./src
COPY target ./target
RUN cargo build

WORKDIR /code
