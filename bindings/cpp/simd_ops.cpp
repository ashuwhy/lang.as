#include <pybind11/pybind11.h>
#include <pybind11/numpy.h>
#include <immintrin.h>

namespace py = pybind11;

class SIMDOps {
public:
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
        for (; i + 4 <= buf1.size; i += 4) {
            __m256d a = _mm256_loadu_pd(ptr1 + i);
            __m256d b = _mm256_loadu_pd(ptr2 + i);
            __m256d c = _mm256_add_pd(a, b);
            _mm256_storeu_pd(ptr3 + i, c);
        }
        
        for (; i < buf1.size; i++) {
            ptr3[i] = ptr1[i] + ptr2[i];
        }
        
        return result;
    }
};

PYBIND11_MODULE(cpp_ops, m) {
    m.doc() = "SIMD operations for AS language"; 
    py::class_<SIMDOps>(m, "SIMDOps")
        .def_static("vector_add", &SIMDOps::vector_add);
} 