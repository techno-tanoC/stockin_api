FROM rust:1.86.0-slim
WORKDIR /workspace

RUN <<EOF
  rustup component add rustfmt
  rustup component add clippy
EOF

RUN <<EOF
  apt update
  apt install -y make git wget curl sqlite3 jq
EOF

RUN <<EOF
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

  cargo binstall --no-confirm bacon@3.12.0
  cargo binstall --no-confirm sqlx-cli@0.7.4
EOF

RUN <<EOF
  arch=$(uname -m)
  if [ $arch = "x86_64" ]; then
    wget -O sqlite3def.tar.gz https://github.com/sqldef/sqldef/releases/download/v2.0.0/sqlite3def_linux_amd64.tar.gz
    wget -O litestream.tar.gz https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz
    wget -O taskfile.tar.gz https://github.com/go-task/task/releases/download/v3.44.0/task_linux_amd64.tar.gz
  elif [ $arch = "aarch64" ]; then
    wget -O sqlite3def.tar.gz https://github.com/sqldef/sqldef/releases/download/v2.0.0/sqlite3def_linux_arm64.tar.gz
    wget -O litestream.tar.gz https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-arm64.tar.gz
    wget -O taskfile.tar.gz https://github.com/go-task/task/releases/download/v3.44.0/task_linux_arm64.tar.gz
  else
    exit 1
  fi
  tar -C /usr/local/bin -xvf sqlite3def.tar.gz
  tar -C /usr/local/bin -xvf litestream.tar.gz
  tar -C /usr/local/bin -xvf taskfile.tar.gz
  rm sqlite3def.tar.gz
  rm litestream.tar.gz
  rm taskfile.tar.gz
EOF
