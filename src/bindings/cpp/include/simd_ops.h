// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

#ifndef ASLANG_SIMD_OPS_H
#define ASLANG_SIMD_OPS_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Add two vectors using SIMD instructions
 * @param a First input vector
 * @param b Second input vector
 * @param result Output vector
 * @param size Size of vectors
 */
void vector_add_f64(const double* a, const double* b, double* result, size_t size);

/**
 * Multiply two vectors using SIMD instructions
 * @param a First input vector
 * @param b Second input vector
 * @param result Output vector
 * @param size Size of vectors
 */
void vector_multiply_f64(const double* a, const double* b, double* result, size_t size);

/**
 * Scale a vector by a constant using SIMD instructions
 * @param input Input vector
 * @param scale Scale factor
 * @param result Output vector
 * @param size Size of vectors
 */
void vector_scale_f64(const double* input, double scale, double* result, size_t size);

#ifdef __cplusplus
}
#endif

#endif // ASLANG_SIMD_OPS_H