# Lang AS - Architecture & Design

*Created by Ashutosh Sharma*

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Core Architecture](#core-architecture)
4. [Language Bindings](#language-bindings)
5. [Build System](#build-system)
6. [Performance](#performance)

## Overview

AS Lang is a polyglot system designed to leverage the best features of modern systems programming languages.

- **Rust**: Provides the **Core Runtime**, ensuring memory safety and concurrency without a garbage collector.
- **C++**: Used for **SIMD (AVX2)** operations where raw hardware access is critical.
- **Python**: Serves as the primary high-level interface.
- **Go & Julia**: Supported via a C-compatible FFI layer.

## Project Structure

```
.
├── src/
│   └── core/           # The Heart of AS Lang
│       ├── compiler/   # Bytecode compiler
│       ├── parser/     # Recursive descent parser
│       ├── runtime/    # Stack-based VM
│       ├── ffi.rs      # C-API exports (extern "C")
│       └── lib.rs      # Crate root
├── bindings/           # The Bridges
│   ├── rust/           # array_ops (Rayon parallelism)
│   ├── cpp/            # SIMD operations (AVX2)
│   ├── wasm/           # wasm-bindgen interface
│   ├── go/             # CGO bindings
│   └── julia/          # Julia ccall bindings
└── tests/              # Integration Suites
```

## Core Architecture

### 1. The Parser (`src/core/parser`)

A recursive descent parser that produces an AST. It supports:

- Operator precedence (Pratt parsing)
- Async/Await syntax
- Error recovery

### 2. The Compiler (`src/core/compiler`)

Lowers AST into a compact bytecode format.

- **Opcode Design**: Stack-based instructions (`LoadConst`, `Call`, `BinaryOp`).
- **Optimization**: Constant folding happens here.

### 3. The Runtime (`src/core/runtime`)

Executes the bytecode.

- **Values**: Tagged union (`enum Value`) handling Numbers, Strings, and Arrays.
- **Memory**: Hybrid approach using Rust's ownership model + reference counting (Arc) for shared data.

### 4. FFI Layer (`src/core/ffi.rs`)

Exposes the runtime to the outside world via the C ABI.

- `as_execute(code: *const c_char) -> *mut c_char`: The entry point for Go/Julia/C.

## Language Bindings

This is where AS Lang shines. It is not just one language; it is a library that speaks many languages.

### 1. Rust Array Ops (`bindings/rust/array_ops`)

A dedicated crate for high-performance math.

- Uses `Rayon` for parallel iterators.
- Implements `NDArray` for NumPy-like behavior.

### 2. C++ SIMD (`bindings/cpp`)

Hand-written intrinsics for maximum throughput.

- `vector_add`, `dot_product`, `matrix_add`.
- Compiled via CMake and linked into the Rust binary.

### 3. WebAssembly (`bindings/wasm`)

Allows AS Lang to run in the browser.

- Exports `execute()`, `parse()`, `compile()`.
- Useful for building online playgrounds or IDEs.

### 4. Go & Julia

- **Go**: Uses `cgo` to wrap the C API.
- **Julia**: Uses `ccall` for zero-overhead calls into the runtime.

## Build System

The build system is a hybrid:

1. **Cargo**: Builds the Rust core.
    - Feature flags (`python`, `default`) control whether to build with PyO3 or as a standalone FFI lib.
2. **CMake**: Builds the C++ SIMD components.
3. **setuptools**: Orchestrates the Python extension build (`setup.py`).

### Building for FFI

To embed AS Lang in Go or Julia, we build a "cdylib":

```bash
cargo build --release -p aslang --no-default-features
```

This produces a standard shared library (`.so`, `.dylib`, or `.dll`) that has no Python dependencies.

## Performance

AS Lang aims for **Zero-Cost Abstractions** in its core.

- **Interpreter**: ~10x faster than raw Python for loop-heavy code.
- **Array Ops**: ~100x faster than Python lists (comparable to NumPy) due to SIMD and parallelism.
- **Startup**: Immediate (sub-millisecond), unlike JVM-based languages.
