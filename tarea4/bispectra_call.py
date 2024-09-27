import numpy as np
from bispectra_rust import calculate_bispectra
# before running script need to install module (run "maturin develop --release" in rust module)
# Random data batches
data_batches = [np.random.rand(1000, 1000) for _ in range(10)] 

results = calculate_bispectra(data_batches)

for result in results:
    print("Bispectra result shape:", result.shape)