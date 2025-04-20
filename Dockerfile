FROM ubuntu:22.04 AS builder

WORKDIR /usr/src/pileup_region
COPY . .

# Install dependencies for rust-htslib
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    zlib1g-dev \
    libcurl4-openssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build release binary
RUN cargo build --release

# Create a smaller runtime image
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    zlib1g \
    libcurl4 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /usr/src/pileup_region/target/release/pileup_region /usr/local/bin/

# Set the entrypoint
ENTRYPOINT ["pileup_region"]
