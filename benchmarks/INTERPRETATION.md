# Benchmark Interpretation (fastLoess)

## Summary

The `fastLoess` crate demonstrates consistent performance improvements over R's `stats::loess`. The latest benchmarks comparing **Serial** vs **Parallel (Rayon)** execution modes show that the parallel implementation correctly leverages multiple cores to provide additional speedups, particularly for computationally heavier tasks (high dimensions, larger datasets).

Overall, Rust implementations achieve **3x to 54x** speedups over R.

## Comparison: R vs Rust (Serial) vs Rust (Parallel)

The table below shows the execution time and speedup relative to R.

| Name                           |      R       |  Rust (Serial)  |  Rust (Parallel)  |
|--------------------------------|--------------|-----------------|-------------------|
| **Dimensions**                 |              |                 |                   |
| 1d_linear                      |    4.18ms    |  0.58ms (7.2x)  | **0.52ms (8.1x)** |
| 2d_linear                      |   13.24ms    |  2.03ms (6.5x)  | **1.31ms (10.1x)**|
| 3d_linear                      |   28.37ms    |  3.57ms (7.9x)  | **2.08ms (13.6x)**|
| **Pathological**               |              |                 |                   |
| clustered                      |   19.70ms    | 1.26ms (15.7x)  | **0.92ms (21.5x)**|
| constant_y                     |   13.61ms    | 1.00ms (13.6x)  | **0.78ms (17.5x)**|
| extreme_outliers               |   23.55ms    | 2.29ms (10.3x)  | **2.02ms (11.7x)**|
| high_noise                     |   34.96ms    | 1.76ms (19.9x)  | **1.25ms (28.0x)**|
| **Polynomial Degree**          |              |                 |                   |
| degree_constant                |    8.50ms    | 0.85ms (10.0x)  | **0.63ms (13.5x)**|
| degree_linear                  |   13.47ms    | 0.83ms (16.2x)  | **0.63ms (21.4x)**|
| degree_quadratic               |   19.07ms    | 0.82ms (23.3x)  | **0.64ms (29.7x)**|
| **Scalability**                |              |                 |                   |
| scale_1000                     |    1.09ms    |  0.26ms (4.3x)  | **0.30ms (3.7x)** |
| scale_5000                     |    8.63ms    |  1.19ms (7.2x)  | **1.06ms (8.2x)** |
| scale_10000                    |   28.68ms    | 2.76ms (10.4x)  | **1.98ms (14.5x)**|
| **Real-world Scenarios**       |              |                 |                   |
| financial_1000                 |    1.11ms    |  0.23ms (4.8x)  | 0.24ms (4.7x)     |
| financial_5000                 |    8.28ms    |  1.09ms (7.6x)  | **0.90ms (9.2x)** |
| genomic_5000                   |    8.27ms    |  1.24ms (6.7x)  | **1.11ms (7.5x)** |
| scientific_5000                |   11.23ms    |  1.65ms (6.8x)  | **1.11ms (10.1x)**|
| **Parameter Sensitivity**      |              |                 |                   |
| fraction_0.67                  |   44.96ms    | 0.83ms (54.0x)  | **0.83ms (54.1x)**|
| iterations_10                  |   23.31ms    | 2.13ms (10.9x)  | **1.97ms (11.8x)**|

*Note: "Rust (Parallel)" corresponds to the optimized CPU backend using Rayon.*

## Key Takeaways

1. **Parallel Wins on Load**: For computationally intensive tasks (e.g., `3d_linear`, `high_noise`, `scientific_5000`, `scale_10000`), the parallel backend provides significant additional speedup over the serial implementation (e.g., **13.6x vs 7.9x** for 3D data).
2. **Overhead on Small Data**: For very small or fast tasks (e.g., `scale_1000`, `financial_1000`), the serial implementation is comparable or slightly faster, indicating that thread management overhead is visible but minimal (often < 0.05ms difference).
3. **Consistent Superiority**: Both Rust implementations consistently outperform R, usually by an order of magnitude.

## Recommendation

* **Default to Parallel**: The overhead for small datasets is negligible (microseconds), while the gains for larger or more complex datasets are substantial (doubling the speedup factor in some cases).
* **Use Serial for Tiny Batches**: If processing millions of independent tiny datasets (< 1000 points) where calling `fit()` repeatedly, the serial backend might save thread pool overhead.
