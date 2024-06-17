# Use the official balenalib base image for Raspberry Pi
FROM balenalib/rpi-raspbian:latest

# Set environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive
ENV VCPKG_FORCE_SYSTEM_BINARIES=1

# Update and install required packages
RUN apt-get update && \
    apt-get install -y \
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabihf \
    cmake \
    pkg-config \
    curl \
    git \
    build-essential \
    llvm \
    clang \
    libclang-dev \
    libopencv-dev \
    curl zip unzip tar \
    ninja-build && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Verify llvm-config is installed
RUN which llvm-config
RUN llvm-config --version

# Set environment variables
RUN export LLVM_CONFIG_PATH=$(which llvm-config)
RUN export LIBCLANG_PATH=$(llvm-config --libdir)

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# Add the ARM target for Rust
RUN rustup target add armv7-unknown-linux-gnueabihf

# # Install vcpkg
# RUN git clone https://github.com/microsoft/vcpkg.git /opt/vcpkg && \
#     /opt/vcpkg/bootstrap-vcpkg.sh -useSystemBinaries

# Set environment variables for clang-sys and vcpkg
# ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config
# ENV LIBCLANG_PATH=/usr/lib/llvm-10/lib
# ENV VCPKG_ROOT=/opt/vcpkg
# ENV PATH=$VCPKG_ROOT:$PATH

# Integrate vcpkg
# RUN /opt/vcpkg/vcpkg integrate install

# Create and set the working directory
WORKDIR /workspace

# Copy the project files into the container
COPY . .

# Build the project for the Raspberry Pi 2
RUN cargo build --target=armv7-unknown-linux-gnueabihf --release -v

# Entry point to the container
CMD ["bash"]
