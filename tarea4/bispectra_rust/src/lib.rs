use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};
use ndarray::{ArrayD};
// run maturin develop --release before running python script
#[pyfunction]
fn calculate_bispectra(data_batches: Vec<&PyArrayDyn<f64>>) -> PyResult<Vec<PyObject>> {
    // Process each batch inside the GIL scope
    Python::with_gil(|py| {
        let results: Vec<PyObject> = data_batches
            .iter()
            .map(|batch| {
                // Convert PyArrayDyn to ndarray ArrayD
                let array = batch.readonly().as_array().to_owned();

                // Perform bispectra calculation (placeholder)
                let result = bispectra_calculation(&array);

                // Convert the result back to PyObject (as NumPy array)
                result.into_pyarray(py).into()  // Use into_pyarray as before
            })
            .collect();

        Ok(results)
    })
}

// Placeholder for the actual bispectra calculation
fn bispectra_calculation(data: &ArrayD<f64>) -> ArrayD<f64> {
    data.mapv(|x| x * 2.0) // Example operation
}

#[pymodule]
fn bispectra_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_bispectra, m)?)?;
    Ok(())
}
