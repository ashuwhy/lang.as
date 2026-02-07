// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use ndarray::Array2;
use rayon::prelude::*;

#[pyclass]
#[derive(Clone)]
struct NDArray {
    data: Vec<f64>,
    dims: Vec<usize>,
    strides: Vec<usize>,
}

#[pymethods]
impl NDArray {
    #[new]
    fn new(dims: Vec<usize>) -> PyResult<Self> {
        if dims.is_empty() {
            return Err(PyValueError::new_err("Dimensions cannot be empty"));
        }
        
        let size: usize = dims.iter().product();
        let strides = Self::compute_strides(&dims);
        
        Ok(NDArray {
            data: vec![0.0; size],
            dims,
            strides,
        })
    }

    /// Create an NDArray from a flat vector and shape
    #[staticmethod]
    fn from_vec(data: Vec<f64>, dims: Vec<usize>) -> PyResult<Self> {
        let expected_size: usize = dims.iter().product();
        if data.len() != expected_size {
            return Err(PyValueError::new_err(
                format!("Data length {} doesn't match shape {:?} (expected {})", 
                    data.len(), dims, expected_size)
            ));
        }
        
        let strides = Self::compute_strides(&dims);
        Ok(NDArray { data, dims, strides })
    }

    /// Convert to a flat vector
    fn to_vec(&self) -> Vec<f64> {
        self.data.clone()
    }

    fn get(&self, indices: Vec<usize>) -> PyResult<f64> {
        let flat_idx = self.compute_flat_index(&indices)?;
        Ok(self.data[flat_idx])
    }

    fn set(&mut self, indices: Vec<usize>, value: f64) -> PyResult<()> {
        let flat_idx = self.compute_flat_index(&indices)?;
        self.data[flat_idx] = value;
        Ok(())
    }

    fn shape(&self) -> Vec<usize> {
        self.dims.clone()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    /// Element-wise addition
    fn add(&self, other: &NDArray) -> PyResult<NDArray> {
        if self.dims != other.dims {
            return Err(PyValueError::new_err("Shapes must match for addition"));
        }
        
        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a + b)
            .collect();
        
        Ok(NDArray {
            data: result,
            dims: self.dims.clone(),
            strides: self.strides.clone(),
        })
    }

    /// Element-wise multiplication
    fn mul(&self, other: &NDArray) -> PyResult<NDArray> {
        if self.dims != other.dims {
            return Err(PyValueError::new_err("Shapes must match for multiplication"));
        }
        
        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a * b)
            .collect();
        
        Ok(NDArray {
            data: result,
            dims: self.dims.clone(),
            strides: self.strides.clone(),
        })
    }

    /// Scalar multiplication
    fn scale(&self, factor: f64) -> NDArray {
        let result: Vec<f64> = self.data.par_iter()
            .map(|&x| x * factor)
            .collect();
        
        NDArray {
            data: result,
            dims: self.dims.clone(),
            strides: self.strides.clone(),
        }
    }

    /// Sum all elements
    fn sum(&self) -> f64 {
        self.data.par_iter().sum()
    }

    /// Mean of all elements
    fn mean(&self) -> f64 {
        self.sum() / self.data.len() as f64
    }
}

impl NDArray {
    fn compute_strides(dims: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; dims.len()];
        for i in (0..dims.len()-1).rev() {
            strides[i] = strides[i + 1] * dims[i + 1];
        }
        strides
    }

    fn compute_flat_index(&self, indices: &[usize]) -> PyResult<usize> {
        if indices.len() != self.dims.len() {
            return Err(PyValueError::new_err("Wrong number of indices"));
        }
        
        for (idx, dim) in indices.iter().zip(self.dims.iter()) {
            if *idx >= *dim {
                return Err(PyValueError::new_err(
                    format!("Index {} out of bounds for dimension {}", idx, dim)
                ));
            }
        }
        
        Ok(indices.iter()
            .zip(self.strides.iter())
            .map(|(&idx, &stride)| idx * stride)
            .sum())
    }
}

#[pymodule]
fn aslang_array_ops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NDArray>()?;
    m.add_function(wrap_pyfunction!(parallel_map, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_reduce, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(elementwise_add, m)?)?;
    m.add_function(wrap_pyfunction!(elementwise_mul, m)?)?;
    Ok(())
}

#[pyfunction]
fn parallel_map(input: Vec<f64>, scale: f64) -> PyResult<Vec<f64>> {
    let result: Vec<f64> = input.par_iter()
        .map(|&x| x * scale)
        .collect();
    Ok(result)
}

#[pyfunction]
fn parallel_reduce(input: Vec<f64>) -> PyResult<f64> {
    let sum: f64 = input.par_iter().sum();
    Ok(sum)
}

#[pyfunction]
fn elementwise_add(a: Vec<f64>, b: Vec<f64>) -> PyResult<Vec<f64>> {
    if a.len() != b.len() {
        return Err(PyValueError::new_err("Vectors must have same length"));
    }
    let result: Vec<f64> = a.par_iter()
        .zip(b.par_iter())
        .map(|(&x, &y)| x + y)
        .collect();
    Ok(result)
}

#[pyfunction]
fn elementwise_mul(a: Vec<f64>, b: Vec<f64>) -> PyResult<Vec<f64>> {
    if a.len() != b.len() {
        return Err(PyValueError::new_err("Vectors must have same length"));
    }
    let result: Vec<f64> = a.par_iter()
        .zip(b.par_iter())
        .map(|(&x, &y)| x * y)
        .collect();
    Ok(result)
}

#[pyfunction]
fn matrix_multiply(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
        return Err(PyValueError::new_err(
            "Invalid matrix dimensions for multiplication",
        ));
    }

    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let a_array = Array2::from_shape_vec((rows_a, cols_a), a.into_iter().flatten().collect())
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let b_array = Array2::from_shape_vec((cols_a, cols_b), b.into_iter().flatten().collect())
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let result = a_array.dot(&b_array);
    
    Ok(result.outer_iter()
        .map(|row| row.to_vec())
        .collect())
}