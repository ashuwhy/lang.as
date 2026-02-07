#!/bin/bash
# Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

set -e  # Exit on error

echo "Installing AS Lang..."

# Check and install Rust if needed
install_rust() {
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

# Check for required tools and install if possible
check_command() {
    if ! command -v $1 &> /dev/null; then
        case $1 in
            python3)
                echo "Error: Python3 is required but not installed."
                echo "Please install Python3 and try again."
                exit 1
                ;;
            pip3)
                echo "Error: pip3 is required but not installed."
                echo "Please install pip3 and try again."
                exit 1
                ;;
            rustc|cargo)
                install_rust
                ;;
            cmake)
                if command -v apt-get &> /dev/null; then
                    echo "Installing cmake..."
                    sudo apt-get update && sudo apt-get install -y cmake
                elif command -v brew &> /dev/null; then
                    echo "Installing cmake..."
                    brew install cmake
                else
                    echo "Error: cmake is required but not installed."
                    echo "Please install cmake and try again."
                    exit 1
                fi
                ;;
            go)
                if command -v apt-get &> /dev/null; then
                    echo "Installing Go..."
                    sudo apt-get update && sudo apt-get install -y golang
                elif command -v brew &> /dev/null; then
                    echo "Installing Go..."
                    brew install go
                else
                    echo "Error: Go is required but not installed."
                    echo "Please install Go and try again."
                    exit 1
                fi
                ;;
            julia)
                if command -v apt-get &> /dev/null; then
                    echo "Installing Julia..."
                    sudo apt-get update && sudo apt-get install -y julia
                elif command -v brew &> /dev/null; then
                    echo "Installing Julia..."
                    brew install julia
                else
                    echo "Error: Julia is required but not installed."
                    echo "Please install Julia and try again."
                    exit 1
                fi
                ;;
            *)
                echo "Error: $1 is required but not installed."
                echo "Please install $1 and try again."
                exit 1
                ;;
        esac
    fi
}

# Check for curl (needed for Rust installation)
check_command curl

# Check all required tools
check_command python3
check_command pip3
check_command rustc
check_command cargo
check_command cmake
check_command go
check_command julia

# Create directories
echo "Creating directory structure..."
mkdir -p src/bindings/{rust,cpp,go,julia,wasm}/src
mkdir -p src/core
mkdir -p src/runtime

# Create virtual environment
echo "Creating Python virtual environment..."
python3 -m venv .venv
source .venv/bin/activate || {
    echo "Error: Failed to activate virtual environment"
    exit 1
}

# Install Python dependencies
echo "Installing Python dependencies..."
pip install --upgrade pip
pip install -r requirements.txt || {
    echo "Error: Failed to install Python dependencies"
    exit 1
}
pip install setuptools-rust wheel

# Build Rust components
echo "Building Rust components..."
# First update rust
rustup update || {
    echo "Warning: Failed to update Rust, continuing with current version"
}

# Clean any previous builds
cargo clean

# Build with detailed error output
RUST_BACKTRACE=1 cargo build --release -vv || {
    echo "Error: Failed to build Rust components"
    echo "Checking individual components..."
    
    # Try building components separately
    (cd src/bindings/rust/array_ops && cargo build --release) || {
        echo "Error: Failed to build array_ops"
        exit 1
    }
    
    (cd src/bindings/wasm && cargo build --release) || {
        echo "Error: Failed to build wasm"
        exit 1
    }
    
    echo "Error: Failed to build main project despite components building successfully"
    exit 1
}

# Build C++ components
echo "Building C++ components..."
(cd src/bindings/cpp && {
    cmake . && make
}) || {
    echo "Warning: Failed to build C++ components"
}

# Build Go components
echo "Building Go components..."
(cd src/bindings/go && {
    go mod init aslang/go_ops
    go build -buildmode=c-shared -o libgo_ops.so
}) || {
    echo "Warning: Failed to build Go components"
}

# Build Julia components
echo "Building Julia components..."
(cd src/bindings/julia && {
    julia -e 'using Pkg; Pkg.activate("."); Pkg.instantiate()'
}) || {
    echo "Warning: Failed to build Julia components"
}

# Install the package
echo "Installing AS Lang..."
pip install -e . || {
    echo "Error: Failed to install AS Lang package"
    exit 1
}

# Create command line link
echo "Creating command line link..."
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
ln -sf "$(pwd)/target/release/aslang" "$INSTALL_DIR/aslang" || {
    echo "Warning: Failed to create command line link"
}

echo "Installation complete!"
echo "Please add $INSTALL_DIR to your PATH if not already added."
echo "You can now use 'aslang' command to run AS Lang programs."

# Suggest adding PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "To add the installation directory to your PATH, run:"
    echo "echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.bashrc"
    echo "source ~/.bashrc"
fi 
