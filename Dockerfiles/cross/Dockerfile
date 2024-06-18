FROM rustembedded/cross:armv7-unknown-linux-gnueabihf
# FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1
# # Use the official balenalib base image for Raspberry Pi
# # FROM balenalib/rpi-raspbian:latest

# # Set environment variables for non-interactive installation
# # ENV DEBIAN_FRONTEND=noninteractive

# RUN apt-get update && apt-get install -y \
#     gcc-arm-linux-gnueabihf \
#     g++-arm-linux-gnueabihf \
#     libv4l-dev \
#     curl \
#     git \
#     build-essential && \
#     apt-get clean && \
#     rm -rf /var/lib/apt/lists/*

# # Install Rust
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH=/root/.cargo/bin:$PATH

# # Add the ARM target for Rust
# RUN rustup target add armv7-unknown-linux-gnueabihf
