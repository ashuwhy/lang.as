// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use ndarray::Array2;
use rayon::prelude::*;

#[pyclass]
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
        let mut strides = vec![1; dims.len()];
        for i in (0..dims.len()-1).rev() {
            strides[i] = strides[i + 1] * dims[i + 1];
        }
        
        Ok(NDArray {
            data: vec![0.0; size],
            dims,
            strides,
        })
    }

    fn get(&self, indices: Vec<usize>) -> PyResult<f64> {
        if indices.len() != self.dims.len() {
            return Err(PyValueError::new_err("Wrong number of indices"));
        }
        
        for (idx, dim) in indices.iter().zip(self.dims.iter()) {
            if *idx >= *dim {
                return Err(PyValueError::new_err(format!("Index {} out of bounds for dimension {}", idx, dim)));
            }
        }
        
        let flat_idx = indices.iter()
            .zip(self.strides.iter())
            .map(|(&idx, &stride)| idx * stride)
            .sum();
            
        Ok(self.data[flat_idx])
    }

    fn set(&mut self, indices: Vec<usize>, value: f64) -> PyResult<()> {
        if indices.len() != self.dims.len() {
            return Err(PyValueError::new_err("Wrong number of indices"));
        }
        
        for (idx, dim) in indices.iter().zip(self.dims.iter()) {
            if *idx >= *dim {
                return Err(PyValueError::new_err(format!("Index {} out of bounds for dimension {}", idx, dim)));
            }
        }
        
        let flat_idx = indices.iter()
            .zip(self.strides.iter())
            .map(|(&idx, &stride)| idx * stride)
            .sum();
            
        self.data[flat_idx] = value;
        Ok(())
    }

    fn shape(&self) -> Vec<usize> {
        self.dims.clone()
    }
}

#[pymodule]
fn aslang_array_ops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NDArray>()?;
    m.add_function(wrap_pyfunction!(parallel_map, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_reduce, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_multiply, m)?)?;
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
    let sum: f64 = input.par_iter()
        .sum();
    Ok(sum)
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