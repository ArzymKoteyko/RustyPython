use pyo3::prelude::*;
use std::time::{Duration, Instant};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn primes_from_range(start:usize, finish:usize) -> PyResult<Vec<i32>> {
    
    let now = Instant::now();
    let mut A = vec![0i32; finish + 1];
    
    for prime_number in 2..((((finish+1) as f64).sqrt()) as usize + 1) {
        if A[prime_number] == 0 {
            let mut mod_number = prime_number*2;
            while mod_number < finish + 1 {
                A[mod_number] += 1;
                mod_number += prime_number;
            } 
        }
    }
        
    A[0] = 1;
    A[1] = 1;
    let mut Res = vec![0i32; 0];
    for number in start..finish+1 {
        if A[number] == 0 {
            Res.push(number as i32);
        }
    }
    Res.push(now.elapsed().as_millis() as i32);
    Ok((Res))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(primes_from_range, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}