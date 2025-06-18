FROM ubuntu:22.04

# Update package lists and install dependencies in one layer
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    parallel \
    zip unzip \
    libssl-dev \
    libclang-dev \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    libcurl4-openssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set PATH for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify Rust installation
RUN . ~/.cargo/env && rustc --version


# Copy your Rust project directory
WORKDIR /app
# Set the entrypoint
# Copy your project here (replace with your actual copy command)
COPY . .

# Build your Rust project (replace with your actual build command)
RUN cargo build --release

ENV PATH="${PATH}:/app/target/release"
# Final image

ENV SHELL=/bin/bash

CMD ["/bin/bash"]
