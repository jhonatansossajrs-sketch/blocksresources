FROM mcr.microsoft.com/vscode/devcontainers/rust:1

# Install additional tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Stellar CLI
RUN curl -sSf https://raw.githubusercontent.com/stellar/stellar-sdk/master/install.sh | sh

# Set environment variables
ENV PATH="/home/vscode/.stellar/bin:${PATH}"
