FROM rust:1.83.0-slim
WORKDIR /workspace

RUN <<EOF
  rustup component add rustfmt
  rustup component add clippy
EOF

RUN <<EOF
  apt update
  apt install -y make git wget curl postgresql-client jq
EOF

RUN <<EOF
  curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

  cargo binstall --no-confirm cargo-watch
  cargo binstall --no-confirm sqlx-cli
EOF

RUN <<EOF
  arch=$(uname -m)
  if [ $arch = "x86_64" ]; then
    wget -O psqldef.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.17.26/psqldef_linux_amd64.tar.gz
    wget -O taskfile.tar.gz https://github.com/go-task/task/releases/download/v3.40.1/task_linux_amd64.tar.gz
  elif [ $arch = "aarch64" ]; then
    wget -O psqldef.tar.gz https://github.com/sqldef/sqldef/releases/download/v0.17.26/psqldef_linux_arm64.tar.gz
    wget -O taskfile.tar.gz https://github.com/go-task/task/releases/download/v3.40.1/task_linux_arm64.tar.gz
  else
    exit 1
  fi
  tar -C /usr/local/bin -xvf psqldef.tar.gz
  tar -C /usr/local/bin -xvf taskfile.tar.gz
  rm psqldef.tar.gz
  rm taskfile.tar.gz
EOF
