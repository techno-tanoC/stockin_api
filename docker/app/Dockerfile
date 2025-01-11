# Builder stage
FROM rust:1.83.0-slim AS builder
WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
RUN <<EOF
  mkdir -p src/bin/
  echo "" > src/lib.rs
  echo "fn main() {}" > src/main.rs
  echo "fn main() {}" > src/bin/api.rs
  echo "fn main() {}" > src/bin/seed.rs
  cargo build --release --locked
EOF

ENV SQLX_OFFLINE true
COPY . .
RUN <<EOF
  touch src/lib.rs
  touch src/main.rs
  touch src/bin/api.rs
  touch src/bin/seed.rs
  cargo build --release --locked
EOF

# App stage
FROM gcr.io/distroless/cc-debian12:debug
WORKDIR /workspace

COPY --from=builder /workspace/target/release/api ./

CMD ["./api"]
