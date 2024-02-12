# Builder stage
FROM rust:1.76.0-slim AS builder
WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
RUN <<EOF
  mkdir -p src/bin/
  echo "fn main() {}" > src/main.rs
  echo "fn main() {}" > src/bin/api.rs
  echo "fn main() {}" > src/bin/seed.rs
  cargo build --release --locked
EOF

ENV SQLX_OFFLINE true
COPY . .
RUN <<EOF
  touch src/main.rs
  touch src/bin/api.rs
  touch src/bin/seed.rs
  cargo build --release --locked
EOF

# App stage
FROM debian:12-slim
WORKDIR /workspace

RUN <<EOF
  apt update
  apt install -y wget

  wget -O sqlite3def.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.16.15/sqlite3def_linux_amd64.tar.gz
  wget -O litestream.tar.gz https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz
  tar -C /usr/local/bin -xvf sqlite3def.tar.gz
  tar -C /usr/local/bin -xvf litestream.tar.gz
  rm sqlite3def.tar.gz
  rm litestream.tar.gz

  apt purge -y wget
EOF

COPY schema.sql docker/app/litestream.yaml docker/app/entrypoint.sh ./
COPY --from=builder /workspace/target/release/api .

ENTRYPOINT ["./entrypoint.sh"]
