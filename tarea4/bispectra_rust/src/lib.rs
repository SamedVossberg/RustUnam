use pyo3::prelude::*;
use numpy::{PyArrayDyn};
use numpy::ToPyArray; 
use ndarray::{ArrayD, Array2, Axis};
use ndarray::parallel::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

// Define the Python function -> run "maturin develop --release" before running python script to access the module!
#[pyfunction]
fn calculate_bispectra(
    _py: Python<'_>, 
    data_batches: Vec<&PyArrayDyn<f64>>,
) -> PyResult<Vec<PyObject>> { // TODO Check understand why whis
    let data_arrays: Vec<ArrayD<f64>> = data_batches
        .iter()
        .map(|batch| {
            batch.readonly().as_array().to_owned()
        })
        .collect();

    // Process data in parallel using Rayon -> this makes it waaaay faster
    let results: Vec<ArrayD<f64>> = data_arrays
        .par_iter()
        .map(|array| {
            bispectra_calculation(array) // -> actual call here
        })
        .collect();

    // Convert the results back to PyObject (as NumPy arrays) 
    let py_results: Vec<PyObject> = results
        .into_iter()
        .map(|result| {
            // Acquire the GIL to convert result to PyArray 
            Python::with_gil(|py| result.to_pyarray(py).into_py(py))
        })
        .collect();

    Ok(py_results)
}

// Implement the actual bispectra calculation
fn bispectra_calculation(data: &ArrayD<f64>) -> ArrayD<f64> {
    assert_eq!(data.ndim(), 2, "Input data must be a 2D array");

    let shape = data.shape();
    let num_signals = shape[0];
    let signal_length = shape[1];

    // Convert data to Complex<f64> -> still have to review this again
    let mut data_complex = Array2::<Complex<f64>>::zeros((num_signals, signal_length));
    data_complex
        .axis_iter_mut(Axis(0))
        .into_par_iter()
        .zip(data.axis_iter(Axis(0)))
        .for_each(|(mut row_complex, row_real)| {
            row_complex
                .iter_mut()
                .zip(row_real)
                .for_each(|(elem_complex, &elem_real)| {
                    *elem_complex = Complex::new(elem_real, 0.0);
                });
        });

    // Perform FFT on each signal -> way through power density or not might need to check
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(signal_length);

    data_complex
        .axis_iter_mut(Axis(0))
        .into_par_iter()
        .for_each(|mut signal| {
            fft.process(signal.as_slice_mut().unwrap());
        });

    // Compute bispectrum
    let N = signal_length;
    let half_N = N / 2;

    // Use parallel iterator over f1 and f2
    let bispectrum = Array2::from_shape_fn((half_N, half_N), |(f1, f2)| {
        let f3 = (f1 + f2) % N;
        let sum: f64 = (0..num_signals)
            .into_par_iter()
            .map(|s| {
                let X_f1 = data_complex[[s, f1]];
                let X_f2 = data_complex[[s, f2]];
                let X_f3_conj = data_complex[[s, f3]].conj();
                let triple_product = X_f1 * X_f2 * X_f3_conj;
                triple_product.norm()
            })
            .sum();
        sum / num_signals as f64
    });

    bispectrum.into_dyn()
}

#[pymodule]
fn bispectra_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_bispectra, m)?)?;
    Ok(())
}
