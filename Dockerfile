# To run the ordinal explorer:
# docker run -d --name qord_server -e RUST_LOG=debug -e ORD_BITCOIN_RPC_USER=qtum -e ORD_BITCOIN_RPC_PASS=qtum -e CHAIN=testnet -e RPC_URL=<qtumd container>:3889  -p 8080:8080 qord

# Use Ubuntu as the base image
FROM ubuntu:20.04

# Set a non-interactive build to avoid prompts during the build process
ARG DEBIAN_FRONTEND=noninteractive

# Install common libraries and tools needed for a Rust application
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /usr/src/qord

# Copy the source code of qord into the container
COPY . .

# Build the qord binary with the release profile
RUN cargo build --release

# Expose the port that qord will listen on (default: 8080)
EXPOSE 8080

# Set the entrypoint to the qord binary
ENTRYPOINT ["/usr/src/qord/target/release/qord"]

# Set the default command to the `server` subcommand
CMD ["server"]
