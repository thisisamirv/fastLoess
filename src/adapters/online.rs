//! Online adapter for incremental LOESS smoothing.
//!
//! ## Purpose
//!
//! This module provides the online (incremental) execution adapter for LOESS
//! smoothing. It wraps the loess-rs OnlineLoess with parallel execution support.
//!
//! ## Design notes
//!
//! * **Storage**: Uses a fixed-size circular buffer for the sliding window.
//! * **Processing**: Performs smoothing on the current window for each new point.
//! * **Parallelism**: Optional parallel execution (defaults to false for latency).

// Feature-gated imports
#[cfg(feature = "cpu")]
use crate::engine::executor::{smooth_pass_parallel, vertex_pass_parallel};
#[cfg(feature = "cpu")]
use crate::evaluation::cv::cv_pass_parallel;
#[cfg(feature = "cpu")]
use crate::evaluation::intervals::interval_pass_parallel;

// External dependencies
use num_traits::Float;
use std::fmt::Debug;
use std::result::Result;

// Export dependencies from loess-rs crate
use loess_rs::internals::adapters::online::{OnlineLoessBuilder, OnlineOutput, UpdateMode};
use loess_rs::internals::algorithms::regression::SolverLinalg;
use loess_rs::internals::algorithms::regression::ZeroWeightFallback;
use loess_rs::internals::algorithms::robustness::RobustnessMethod;
use loess_rs::internals::math::boundary::BoundaryPolicy;
use loess_rs::internals::math::distance::DistanceLinalg;
use loess_rs::internals::math::kernel::WeightFunction;
use loess_rs::internals::math::linalg::FloatLinalg;
use loess_rs::internals::math::scaling::ScalingMethod;
use loess_rs::internals::primitives::backend::Backend;
use loess_rs::internals::primitives::errors::LoessError;

// ============================================================================
// Extended Online LOESS Builder
// ============================================================================

/// Builder for online LOESS processor with parallel support.
#[derive(Debug, Clone)]
pub struct ParallelOnlineLoessBuilder<T: FloatLinalg + DistanceLinalg + SolverLinalg> {
    /// Base builder from the loess-rs crate
    pub base: OnlineLoessBuilder<T>,
}

impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Debug + Send + Sync> Default
    for ParallelOnlineLoessBuilder<T>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Debug + Send + Sync>
    ParallelOnlineLoessBuilder<T>
{
    /// Create a new online LOESS builder with default parameters.
    fn new() -> Self {
        let mut base = OnlineLoessBuilder::default();
        // Default to false for online (latency-sensitive)
        base.parallel = Some(false);
        Self { base }
    }

    /// Set parallel execution mode.
    pub fn parallel(mut self, parallel: bool) -> Self {
        self.base.parallel = Some(parallel);
        self
    }

    /// Set the execution backend.
    pub fn backend(mut self, backend: Backend) -> Self {
        self.base.backend = Some(backend);
        self
    }

    // ========================================================================
    // Shared Setters
    // ========================================================================

    /// Set the smoothing fraction (span).
    pub fn fraction(mut self, fraction: T) -> Self {
        self.base.fraction = fraction;
        self
    }

    /// Set the number of robustness iterations.
    pub fn iterations(mut self, iterations: usize) -> Self {
        self.base.iterations = iterations;
        self
    }

    /// Set the kernel weight function.
    pub fn weight_function(mut self, wf: WeightFunction) -> Self {
        self.base.weight_function = wf;
        self
    }

    /// Set the robustness method for outlier handling.
    pub fn robustness_method(mut self, method: RobustnessMethod) -> Self {
        self.base.robustness_method = method;
        self
    }

    /// Set the residual scaling method (MAR/MAD).
    pub fn scaling_method(mut self, method: ScalingMethod) -> Self {
        self.base.scaling_method = method;
        self
    }

    /// Set the zero-weight fallback policy.
    pub fn zero_weight_fallback(mut self, fallback: ZeroWeightFallback) -> Self {
        self.base.zero_weight_fallback = fallback;
        self
    }

    /// Set the boundary handling policy.
    pub fn boundary_policy(mut self, policy: BoundaryPolicy) -> Self {
        self.base.boundary_policy = policy;
        self
    }

    /// Enable auto-convergence for robustness iterations.
    pub fn auto_converge(mut self, tolerance: T) -> Self {
        self.base.auto_convergence = Some(tolerance);
        self
    }

    /// Enable returning residuals in the output.
    pub fn compute_residuals(mut self, enabled: bool) -> Self {
        self.base.compute_residuals = enabled;
        self
    }

    /// Enable returning robustness weights in the result.
    pub fn return_robustness_weights(mut self, enabled: bool) -> Self {
        self.base.return_robustness_weights = enabled;
        self
    }

    // ========================================================================
    // Online-Specific Setters
    // ========================================================================

    /// Set window capacity (maximum number of points to retain).
    pub fn window_capacity(mut self, capacity: usize) -> Self {
        self.base.window_capacity = capacity;
        self
    }

    /// Set minimum points before smoothing starts.
    pub fn min_points(mut self, min: usize) -> Self {
        self.base.min_points = min;
        self
    }

    /// Set the update mode for incremental processing.
    pub fn update_mode(mut self, mode: UpdateMode) -> Self {
        self.base.update_mode = mode;
        self
    }

    // ========================================================================
    // Build Method
    // ========================================================================

    /// Build the online processor.
    pub fn build(self) -> Result<ParallelOnlineLoess<T>, LoessError> {
        // Check for deferred errors from adapter conversion
        if let Some(ref err) = self.base.deferred_error {
            return Err(err.clone());
        }

        // Configure parallel callbacks before building
        let mut builder = self.base;

        #[cfg(feature = "cpu")]
        {
            if builder.parallel.unwrap_or(false) {
                builder.custom_smooth_pass = Some(smooth_pass_parallel);
                builder.custom_cv_pass = Some(cv_pass_parallel);
                builder.custom_interval_pass = Some(interval_pass_parallel);
                builder.custom_vertex_pass = Some(vertex_pass_parallel);
            }
        }

        let processor = builder.build()?;
        Ok(ParallelOnlineLoess { processor })
    }
}

// ============================================================================
// Extended Online LOESS Processor
// ============================================================================

/// Online LOESS processor with parallel support.
pub struct ParallelOnlineLoess<T: FloatLinalg + DistanceLinalg + SolverLinalg> {
    processor: loess_rs::internals::adapters::online::OnlineLoess<T>,
}

impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Float + Debug + Send + Sync + 'static>
    ParallelOnlineLoess<T>
{
    /// Add a new point and get its smoothed value.
    pub fn add_point(&mut self, x: &[T], y: T) -> Result<Option<OnlineOutput<T>>, LoessError> {
        self.processor.add_point(x, y)
    }

    /// Get the current window size.
    pub fn window_size(&self) -> usize {
        self.processor.window_size()
    }

    /// Clear the window.
    pub fn reset(&mut self) {
        self.processor.reset();
    }
}
