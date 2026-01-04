# Benchmark Results

## Summary

The Rust `loess-rs` crate demonstrates consistent performance improvements over R's `loess` across all tested scenarios. Median speedups range from **3.0x to 9.8x** across different categories, with peak speedups reaching over **21x** in specific configurations. No regressions were observed.

## Category Comparison

| Category               | Matched | Median Speedup | Mean Speedup |
|------------------------|---------|----------------|--------------|
| **Polynomial Degrees** | 2       | **9.80x**      | 9.80x        |
| **Iterations**         | 6       | **8.15x**      | 8.63x        |
| **Pathological**       | 4       | **7.84x**      | 8.41x        |
| **Fraction**           | 6       | **6.96x**      | 9.24x        |
| **Dimensions**         | 3       | **4.76x**      | 4.99x        |
| **Scalability**        | 2       | **3.78x**      | 3.78x        |
| **Financial**          | 3       | **3.41x**      | 3.63x        |
| **Genomic**            | 2       | **2.99x**      | 2.99x        |
| **Scientific**         | 3       | **2.91x**      | 3.17x        |

## Top 10 Rust Wins

| Benchmark        | Rust   | R       | Speedup    |
|------------------|--------|---------|------------|
| fraction_0.67    | 0.98ms | 21.63ms | **21.97x** |
| fraction_0.5     | 1.07ms | 12.85ms | **11.98x** |
| degree_quadratic | 0.68ms | 7.86ms  | **11.52x** |
| iterations_1     | 0.75ms | 8.44ms  | **11.20x** |
| high_noise       | 1.50ms | 15.86ms | **10.58x** |
| iterations_2     | 0.95ms | 8.95ms  | **9.46x**  |
| iterations_5     | 1.55ms | 12.73ms | **8.19x**  |
| iterations_3     | 1.20ms | 9.73ms  | **8.11x**  |
| degree_linear    | 0.73ms | 5.86ms  | **8.08x**  |
| clustered        | 1.00ms | 7.86ms  | **7.87x**  |

## Regressions

**None identified.** R was not faster than Rust in any of the matched benchmarks.

## Detailed Results

### Dimensions

| Name      | Rust   | R       | Speedup |
|-----------|--------|---------|---------|
| 1d_linear | 0.50ms | 2.25ms  | 4.49x   |
| 2d_linear | 1.40ms | 6.66ms  | 4.76x   |
| 3d_linear | 2.30ms | 13.17ms | 5.73x   |

### Financial

| Name           | Rust   | R       | Speedup |
|----------------|--------|---------|---------|
| financial_1000 | 0.27ms | 0.91ms  | 3.41x   |
| financial_500  | 0.19ms | 0.57ms  | 3.00x   |
| financial_5000 | 0.98ms | 4.39ms  | 4.48x   |

### Fraction

| Name          | Rust   | R       | Speedup |
|---------------|--------|---------|---------|
| fraction_0.05 | 1.20ms | 4.42ms  | 3.67x   |
| fraction_0.1  | 1.11ms | 4.29ms  | 3.88x   |
| fraction_0.2  | 0.95ms | 5.97ms  | 6.28x   |
| fraction_0.3  | 1.14ms | 8.69ms  | 7.64x   |
| fraction_0.5  | 1.07ms | 12.85ms | 11.98x  |
| fraction_0.67 | 0.98ms | 21.63ms | 21.97x  |

### Genomic

| Name         | Rust   | R      | Speedup |
|--------------|--------|--------|---------|
| genomic_1000 | 0.37ms | 0.80ms | 2.19x   |
| genomic_5000 | 1.12ms | 4.23ms | 3.79x   |

### Iterations

| Name          | Rust   | R       | Speedup |
|---------------|--------|---------|---------|
| iterations_0  | 0.76ms | 5.69ms  | 7.48x   |
| iterations_1  | 0.75ms | 8.44ms  | 11.20x  |
| iterations_10 | 2.07ms | 15.15ms | 7.33x   |
| iterations_2  | 0.95ms | 8.95ms  | 9.46x   |
| iterations_3  | 1.20ms | 9.73ms  | 8.11x   |
| iterations_5  | 1.55ms | 12.73ms | 8.19x   |

### Pathological

| Name             | Rust   | R       | Speedup |
|------------------|--------|---------|---------|
| clustered        | 1.00ms | 7.86ms  | 7.87x   |
| constant_y       | 0.80ms | 5.88ms  | 7.38x   |
| extreme_outliers | 1.94ms | 15.11ms | 7.80x   |
| high_noise       | 1.50ms | 15.86ms | 10.58x  |

### Polynomial Degrees

| Name             | Rust   | R      | Speedup |
|------------------|--------|--------|---------|
| degree_linear    | 0.73ms | 5.86ms | 8.08x   |
| degree_quadratic | 0.68ms | 7.86ms | 11.52x  |

### Scalability

| Name       | Rust   | R      | Speedup |
|------------|--------|--------|---------|
| scale_1000 | 0.32ms | 1.00ms | 3.12x   |
| scale_5000 | 1.11ms | 4.91ms | 4.44x   |

### Scientific

| Name            | Rust   | R      | Speedup |
|-----------------|--------|--------|---------|
| scientific_1000 | 0.39ms | 1.12ms | 2.91x   |
| scientific_500  | 0.24ms | 0.60ms | 2.54x   |
| scientific_5000 | 1.34ms | 5.45ms | 4.06x   |

## Notes

- Both use identical scenarios
- Rust crate: `loess-rs` + `fastLoess`
- R: `loess`
- Test date: 2026-01-04
