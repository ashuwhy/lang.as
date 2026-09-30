// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

#include <immintrin.h>
#include "../include/simd_ops.h"

extern "C" {

void vector_add_f64(const double* a, const double* b, double* result, size_t size) {
    size_t i = 0;
    
    // Process 4 doubles at a time using AVX
    for (; i + 4 <= size; i += 4) {
        __m256d va = _mm256_loadu_pd(a + i);
        __m256d vb = _mm256_loadu_pd(b + i);
        __m256d vr = _mm256_add_pd(va, vb);
        _mm256_storeu_pd(result + i, vr);
    }
    
    // Handle remaining elements
    for (; i < size; i++) {
        result[i] = a[i] + b[i];
    }
}

void vector_multiply_f64(const double* a, const double* b, double* result, size_t size) {
    size_t i = 0;
    
    // Process 4 doubles at a time using AVX
    for (; i + 4 <= size; i += 4) {
        __m256d va = _mm256_loadu_pd(a + i);
        __m256d vb = _mm256_loadu_pd(b + i);
        __m256d vr = _mm256_mul_pd(va, vb);
        _mm256_storeu_pd(result + i, vr);
    }
    
    // Handle remaining elements
    for (; i < size; i++) {
        result[i] = a[i] * b[i];
    }
}

void vector_scale_f64(const double* input, double scale, double* result, size_t size) {
    size_t i = 0;
    __m256d vscale = _mm256_set1_pd(scale);
    
    // Process 4 doubles at a time using AVX
    for (; i + 4 <= size; i += 4) {
        __m256d va = _mm256_loadu_pd(input + i);
        __m256d vr = _mm256_mul_pd(va, vscale);
        _mm256_storeu_pd(result + i, vr);
    }
    
    // Handle remaining elements
    for (; i < size; i++) {
        result[i] = input[i] * scale;
    }
}

} 