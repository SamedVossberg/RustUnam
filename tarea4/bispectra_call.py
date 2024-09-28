import bispectra_rust
import numpy as np
from time import time

# random test data
data_batches = [np.random.rand(10, 1024) for _ in range(5)]

start_time = time()
results = bispectra_rust.calculate_bispectra(data_batches)
end_time = time()

print(results)
print(f"Rust bispectra calculation total time: {end_time - start_time:.2f} seconds")


import numpy as np
from time import time

def bispectra_calculation_python(data):
    num_signals, signal_length = data.shape
    data_fft = np.fft.fft(data, axis=1)
    N = signal_length
    half_N = N // 2
    bispectrum = np.zeros((half_N, half_N))
    for f1 in range(half_N):
        for f2 in range(half_N):
            f3 = (f1 + f2) % N
            triple_products = data_fft[:, f1] * data_fft[:, f2] * np.conj(data_fft[:, f3])
            bispectrum[f1, f2] = np.mean(np.abs(triple_products))
    return bispectrum

# Benchmark Python implementation
start_time = time()
results_python = [bispectra_calculation_python(data) for data in data_batches]
end_time = time()
print(f"Python bispectra calculation total time: {end_time - start_time:.2f} seconds")
