cmake_minimum_required(VERSION 3.10)
project(aslang_cpp_ops VERSION 0.1.0)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable optimizations
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3")

# Check for SIMD support
include(CheckCXXCompilerFlag)
check_cxx_compiler_flag("-mavx2" COMPILER_SUPPORTS_AVX2)
if(COMPILER_SUPPORTS_AVX2)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -mavx2")
endif()

# Create shared library
add_library(aslang_cpp_ops SHARED
    src/simd_ops.cpp
)

# Set library properties
set_target_properties(aslang_cpp_ops PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
    PUBLIC_HEADER "include/simd_ops.h"
)

# Install targets
install(TARGETS aslang_cpp_ops
    LIBRARY DESTINATION lib
    PUBLIC_HEADER DESTINATION include
) 