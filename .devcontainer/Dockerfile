# Choose Go version + base OS
# Default: Go 1.25 on Debian Bullseye (can be overridden in devcontainer.json)
ARG VARIANT="1.25-bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/go:1-${VARIANT}

# [Choice] Node.js version: none, lts/*, 20, 18, 16
# Default = none, can be overridden from devcontainer.json
ARG NODE_VERSION="none"
RUN if [ "${NODE_VERSION}" != "none" ]; then \
        su vscode -c "umask 0002 && . /usr/local/share/nvm/nvm.sh && nvm install ${NODE_VERSION} 2>&1"; \
    fi

# Install common system dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        build-essential \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain (cargo, rustc, etc.)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && chmod +x rustup.sh \
    && sh rustup.sh -y \
    && rm rustup.sh

# Install Bun runtime
RUN curl -fsSL https://bun.sh/install | bash

# [Optional] Uncomment to install global Go dependencies
# RUN go install <your-go-package>@latest

# [Optional] Uncomment to install global Node.js packages
# RUN su vscode -c "source /usr/local/share/nvm/nvm.sh && npm install -g <your-package>" 2>&1
