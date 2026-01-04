//! # Fast LOESS (Locally Estimated Scatterplot Smoothing)
//!
//! A production-ready, high-performance, multi-threaded LOESS implementation with comprehensive
//! features for robust nonparametric regression and trend estimation.
//!
//! ## What is LOESS?
//!
//! LOESS (Locally Estimated Scatterplot Smoothing) is a nonparametric regression
//! method that fits smooth curves through scatter plots. At each point, it fits
//! a weighted polynomial using nearby data points, with weights
//! decreasing smoothly with distance. This creates flexible, data-adaptive curves
//! without assuming a global functional form.
//!
//! **Key advantages:**
//! - No parametric assumptions about the underlying relationship
//! - Automatic adaptation to local data structure
//! - Robust to outliers (with robustness iterations enabled)
//! - Provides uncertainty estimates via confidence/prediction intervals
//! - Handles irregular sampling and missing regions gracefully
//! - Multi-threaded execution for high performance
//!
//! ## Quick Start
//!
//! ### Typical Use
//!
//! ```rust
//! use fastLoess::prelude::*;
//! use ndarray::Array1;
//!
//! let x = Array1::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
//! let y = Array1::from_vec(vec![2.0, 4.1, 5.9, 8.2, 9.8]);
//!
//! // Build the model with parallel execution (default)
//! let model = Loess::new()
//!     .fraction(0.5)      // Use 50% of data for each local fit
//!     .iterations(3)      // 3 robustness iterations
//!     .adapter(Batch)     // Parallel by default
//!     .build()?;
//!
//! // Fit the model to the data
//! let result = model.fit(&x, &y)?;
//!
//! println!("{}", result);
//! # Result::<(), LoessError>::Ok(())
//! ```
//!
//! ## Parallel Execution
//!
//! `fastLoess` provides high-performance parallel execution using [rayon](https://docs.rs/rayon).
//!
//! **Default behavior:**
//! - **Batch Adapter**: `parallel(true)` (multi-core smoothing)
//! - **Streaming Adapter**: `parallel(true)` (multi-core chunk processing)
//! - **Online Adapter**: `parallel(false)` (optimized for single-point latency)
//!
//! ```rust
//! use fastLoess::prelude::*;
//! # let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! # let y = vec![2.0, 4.1, 5.9, 8.2, 9.8];
//!
//! // Explicitly control parallelism
//! let model = Loess::new()
//!     .adapter(Batch)
//!     .parallel(true)  // Enable parallel execution
//!     .build()?;
//!
//! let result = model.fit(&x, &y)?;
//! # Result::<(), LoessError>::Ok(())
//! ```
//!
//! ## ndarray Integration
//!
//! `fastLoess` supports [ndarray](https://docs.rs/ndarray) natively, allowing for zero-copy
//! data passing and efficient numerical operations.
//!
//! ```rust
//! use fastLoess::prelude::*;
//! use ndarray::Array1;
//!
//! // Data as ndarray types
//! let x = Array1::from_vec((0..100).map(|i| i as f64 * 0.1).collect());
//! let y = Array1::from_elem(100, 1.0); // Replace with real data
//!
//! let model = Loess::new().adapter(Batch).build()?;
//!
//! // fit() accepts &Array1<f64>, &[f64], or Vec<f64>
//! let result = model.fit(&x, &y)?;
//!
//! // result.y is a Vec<f64>
//! let smoothed_values = result.y;
//! # Result::<(), LoessError>::Ok(())
//! ```

#![allow(non_snake_case)]
#![deny(missing_docs)]

// ============================================================================
// Internal Modules
// ============================================================================

/// Adapter layer for parallel execution modes.
pub mod adapters;

/// Input abstractions for flexible data types.
pub mod input;

/// Parallel execution engine.
pub mod engine;

/// Evaluation utilities (CV, intervals).
pub mod evaluation;

/// High-level API with parallel support.
pub mod api;

// ============================================================================
// Prelude
// ============================================================================

/// Standard fastLoess prelude.
///
/// This module is intended to be wildcard-imported for convenient access
/// to the most commonly used types:
///
/// ```
/// use fastLoess::prelude::*;
/// ```
pub mod prelude {
    // Re-export our parallel adapters
    pub use crate::api::{
        Adapter::{Batch, Online, Streaming},
        LoessError,
    };

    // Re-export the base types from loess-rs
    pub use loess_rs::prelude::{Average, TakeFirst, WeightedAverage};
    pub use loess_rs::prelude::{
        Bisquare, Biweight, Chebyshev, Constant, Cosine, Cubic, Direct, Epanechnikov, Euclidean,
        Full, Gaussian, Huber, Incremental, Interpolation, Linear, Loess, Manhattan, Minkowski,
        Normalized, Quadratic, Quartic, ReturnNone, ReturnOriginal, Talwar, Triangle, Tricube,
        Uniform, UseLocalMean, Weighted, MAD, MAR,
    };
    pub use loess_rs::prelude::{Extend, NoBoundary, Reflect, Zero};
    pub use loess_rs::prelude::{KFold, LoessResult, LOOCV};

    // Re-export ndarray for convenience
    #[cfg(feature = "cpu")]
    pub use ndarray::Array1;
}
