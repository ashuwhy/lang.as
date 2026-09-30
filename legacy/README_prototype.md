# AS Lang

**A high-performance multi-language programming language.**

AS Lang combines the memory safety of Rust, the raw performance of C++ SIMD, and the ease of use of Python. It is designed to be embedded anywhere—from Python scripts to WebAssembly in the browser, to Go and Julia applications.

## 🚀 Features

- **Multi-Language Core**:
  - **Rust**: Memory-safe runtime and compiler.
  - **C++**: AVX2-accelerated SIMD vector operations.
- **Universal Embeddability**:
  - **Python**: Full extension module (`import aslang`).
  - **WebAssembly**: Run in browser/Node.js.
  - **Go**: CGO bindings via FFI.
  - **Julia**: Native `ccall` bindings.
- **Performance**:
  - Zero-cost abstractions.
  - Parallel array operations (via Rayon).
  - Efficient bytecode interpreter.

## 📂 Project Structure

```
.
├── src/
│   ├── core/           # Rust Core (Interpreter, Compiler, FFI)
│   └── runtime/        # Runtime engine
├── bindings/           # Language Bindings
│   ├── python/         # Python extension (setup.py)
│   ├── rust/           # Rust helper crates (array_ops)
│   ├── cpp/            # C++ SIMD operations
│   ├── wasm/           # WebAssembly interface
│   ├── go/             # Go bindings (cgo)
│   └── julia/          # Julia bindings
├── docs/               # Documentation
└── tests/              # Integration tests
```

## 🛠️ Installation

### Prerequisites

- **Rust Toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Python 3.8+**
- **CMake** (for C++ SIMD ops)
- *(Optional)* Go 1.20+, Julia 1.6+

### 1. Python Extension

Install `aslang` as a Python library:

```bash
pip install .
```

Usage:

```python
import aslang.core as aslang
print(aslang.run_code('print("Hello from Python!");'))
```

### 2. Standalone Shared Library (FFI)

Build the shared library (`libaslang.so` / `.dylib` / `.dll`) for use with Go, Julia, or C:

```bash
# Build core without Python dependencies
cargo build --release -p aslang --no-default-features
```

The library will be in `target/release/`.

## 🔗 Language Bindings

### Go

Run `aslang` from Go using CGO:

```bash
export DYLD_LIBRARY_PATH=$(pwd)/target/release:$DYLD_LIBRARY_PATH
go run bindings/go/aslang.go
```

### Julia

Run `aslang` from Julia:

```julia
# In bindings/julia/ASLang.jl
include("bindings/julia/ASLang.jl")
ASLang.execute("print(\"Hello from Julia!\")")
```

### WebAssembly

Build for the web:

```bash
wasm-pack build bindings/wasm --target web
```

## 📄 License

MIT License - Copyright (c) 2026 Ashutosh Sharma.
