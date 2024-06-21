FROM rust:1.78.0-slim
WORKDIR /workspace

RUN <<EOF
  rustup component add rustfmt
  rustup component add clippy
EOF

RUN <<EOF
  apt update
  apt install -y make git wget curl sqlite3
EOF

RUN <<EOF
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

  cargo binstall --no-confirm cargo-watch
  cargo binstall --no-confirm sqlx-cli
EOF

RUN <<EOF
  arch=$(uname -m)
  if [ $arch = "x86_64" ]; then
    wget -O sqlite3def.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.16.15/sqlite3def_linux_amd64.tar.gz
    wget -O litestream.tar.gz https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz
  elif [ $arch = "aarch64" ]; then
    wget -O sqlite3def.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.16.15/sqlite3def_linux_arm64.tar.gz
    wget -O litestream.tar.gz https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-arm64.tar.gz
  else
    exit 1
  fi
  tar -C /usr/local/bin -xvf sqlite3def.tar.gz
  tar -C /usr/local/bin -xvf litestream.tar.gz
  rm sqlite3def.tar.gz
  rm litestream.tar.gz
EOF
