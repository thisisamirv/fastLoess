use approx::assert_relative_eq;
use fastLoess::prelude::*;

#[test]
fn test_parallel_interpolation_mode() {
    let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
    let y: Vec<f64> = x.iter().map(|&v| (v * 0.1).sin()).collect();

    // 1. Fit with parallel interpolation
    let result_parallel = Loess::new()
        .fraction(0.3)
        .surface_mode(Interpolation)
        .adapter(Batch)
        .parallel(true)
        .build()
        .unwrap()
        .fit(&x, &y)
        .unwrap();

    // 2. Fit with sequential interpolation
    let result_sequential = Loess::new()
        .fraction(0.3)
        .surface_mode(Interpolation)
        .adapter(Batch)
        .parallel(false)
        .build()
        .unwrap()
        .fit(&x, &y)
        .unwrap();

    // Verify results are consistent
    assert_eq!(result_parallel.y.len(), result_sequential.y.len());
    for (p, s) in result_parallel.y.iter().zip(result_sequential.y.iter()) {
        assert_relative_eq!(p, s, epsilon = 1e-10);
    }
}

#[test]
fn test_parallel_interpolation_multi_dim() {
    // 2D data: x1 + x2
    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            x.push(i as f64);
            x.push(j as f64);
            y.push((i + j) as f64);
        }
    }

    let result_parallel = Loess::new()
        .dimensions(2)
        .fraction(0.3)
        .surface_mode(Interpolation)
        .adapter(Batch)
        .parallel(true)
        .build()
        .unwrap()
        .fit(&x, &y)
        .unwrap();

    let result_sequential = Loess::new()
        .dimensions(2)
        .fraction(0.3)
        .surface_mode(Interpolation)
        .adapter(Batch)
        .parallel(false)
        .build()
        .unwrap()
        .fit(&x, &y)
        .unwrap();

    assert_eq!(result_parallel.y.len(), result_sequential.y.len());
    for (p, s) in result_parallel.y.iter().zip(result_sequential.y.iter()) {
        assert_relative_eq!(p, s, epsilon = 1e-10);
    }
}
