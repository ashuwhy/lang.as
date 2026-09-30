#include <pybind11/pybind11.h>
#include <pybind11/numpy.h>
#include <immintrin.h>
#include <cmath>

namespace py = pybind11;

class SIMDOps {
public:
    // Vector addition using AVX2
    static py::array_t<double> vector_add(py::array_t<double> input1, py::array_t<double> input2) {
        auto buf1 = input1.request();
        auto buf2 = input2.request();
        
        if (buf1.size != buf2.size)
            throw std::runtime_error("Input shapes must match");
            
        auto result = py::array_t<double>(buf1.size);
        auto buf3 = result.request();
        
        double *ptr1 = (double *) buf1.ptr,
               *ptr2 = (double *) buf2.ptr,
               *ptr3 = (double *) buf3.ptr;
               
        size_t i = 0;
        for (; i + 4 <= static_cast<size_t>(buf1.size); i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            __m256d c = _mm256_add_pd(a, b);
            _mm256_storeu_pd(ptr3 + i, c);
        }
        
        for (; i < static_cast<size_t>(buf1.size); i++) {
            ptr3[i] = ptr1[i] + ptr2[i];
        }
        
        return result;
    }

    // Vector subtraction using AVX2
    static py::array_t<double> vector_sub(py::array_t<double> input1, py::array_t<double> input2) {
        auto buf1 = input1.request();
        auto buf2 = input2.request();
        
        if (buf1.size != buf2.size)
            throw std::runtime_error("Input shapes must match");
            
        auto result = py::array_t<double>(buf1.size);
        auto buf3 = result.request();
        
        double *ptr1 = (double *) buf1.ptr,
               *ptr2 = (double *) buf2.ptr,
               *ptr3 = (double *) buf3.ptr;
               
        size_t i = 0;
        for (; i + 4 <= static_cast<size_t>(buf1.size); i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            __m256d c = _mm256_sub_pd(a, b);
            _mm256_storeu_pd(ptr3 + i, c);
        }
        
        for (; i < static_cast<size_t>(buf1.size); i++) {
            ptr3[i] = ptr1[i] - ptr2[i];
        }
        
        return result;
    }

    // Vector multiplication using AVX2
    static py::array_t<double> vector_mul(py::array_t<double> input1, py::array_t<double> input2) {
        auto buf1 = input1.request();
        auto buf2 = input2.request();
        
        if (buf1.size != buf2.size)
            throw std::runtime_error("Input shapes must match");
            
        auto result = py::array_t<double>(buf1.size);
        auto buf3 = result.request();
        
        double *ptr1 = (double *) buf1.ptr,
               *ptr2 = (double *) buf2.ptr,
               *ptr3 = (double *) buf3.ptr;
               
        size_t i = 0;
        for (; i + 4 <= static_cast<size_t>(buf1.size); i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            __m256d c = _mm256_mul_pd(a, b);
            _mm256_storeu_pd(ptr3 + i, c);
        }
        
        for (; i < static_cast<size_t>(buf1.size); i++) {
            ptr3[i] = ptr1[i] * ptr2[i];
        }
        
        return result;
    }

    // Vector division using AVX2
    static py::array_t<double> vector_div(py::array_t<double> input1, py::array_t<double> input2) {
        auto buf1 = input1.request();
        auto buf2 = input2.request();
        
        if (buf1.size != buf2.size)
            throw std::runtime_error("Input shapes must match");
            
        auto result = py::array_t<double>(buf1.size);
        auto buf3 = result.request();
        
        double *ptr1 = (double *) buf1.ptr,
               *ptr2 = (double *) buf2.ptr,
               *ptr3 = (double *) buf3.ptr;
               
        size_t i = 0;
        for (; i + 4 <= static_cast<size_t>(buf1.size); i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            __m256d c = _mm256_div_pd(a, b);
            _mm256_storeu_pd(ptr3 + i, c);
        }
        
        for (; i < static_cast<size_t>(buf1.size); i++) {
            ptr3[i] = ptr1[i] / ptr2[i];
        }
        
        return result;
    }

    // Dot product using AVX2
    static double dot_product(py::array_t<double> input1, py::array_t<double> input2) {
        auto buf1 = input1.request();
        auto buf2 = input2.request();
        
        if (buf1.size != buf2.size)
            throw std::runtime_error("Input shapes must match");
        
        double *ptr1 = (double *) buf1.ptr,
               *ptr2 = (double *) buf2.ptr;
        
        __m256d sum = _mm256_setzero_pd();
        size_t i = 0;
        
        for (; i + 4 <= static_cast<size_t>(buf1.size); i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            sum = _mm256_add_pd(sum, _mm256_mul_pd(a, b));
        }
        
        // Horizontal sum of the 4 doubles in sum
        double result[4];
        _mm256_storeu_pd(result, sum);
        double total = result[0] + result[1] + result[2] + result[3];
        
        // Handle remaining elements
        for (; i < static_cast<size_t>(buf1.size); i++) {
            total += ptr1[i] * ptr2[i];
        }
        
        return total;
    }

    // Matrix addition (2D arrays flattened)
    static py::array_t<double> matrix_add(py::array_t<double> input1, py::array_t<double> input2) {
        // Simply delegate to vector_add since matrices are stored contiguously
        return vector_add(input1, input2);
    }
};

PYBIND11_MODULE(cpp_ops, m) {
    m.doc() = "SIMD operations for AS language using AVX2";
    py::class_<SIMDOps>(m, "SIMDOps")
        .def_static("vector_add", &SIMDOps::vector_add, "Element-wise vector addition")
        .def_static("vector_sub", &SIMDOps::vector_sub, "Element-wise vector subtraction")
        .def_static("vector_mul", &SIMDOps::vector_mul, "Element-wise vector multiplication")
        .def_static("vector_div", &SIMDOps::vector_div, "Element-wise vector division")
        .def_static("dot_product", &SIMDOps::dot_product, "Dot product of two vectors")
        .def_static("matrix_add", &SIMDOps::matrix_add, "Element-wise matrix addition");
}