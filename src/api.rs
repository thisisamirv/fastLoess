//! High-level API for LOESS smoothing with parallel execution support.
//!
//! ## Purpose
//!
//! This module provides the primary user-facing entry point for LOESS with
//! heavy-duty parallel execution capabilities. It extends the `loess-rs` API
//! with adapters that utilize all available CPU cores or GPU hardware.
//!
//! ## Design notes
//!
//! * **Fluent Integration**: Re-uses the base `loess-rs` builder pattern.
//! * **Parallel-First**: Defaults to parallel execution where beneficial.
//! * **Transparent**: Marker types (Batch, Streaming, Online) select the parallel builders.
//!
//! ## Key concepts
//!
//! * **Parallel Support**: Uses `rayon` (CPU) or `wgpu` (GPU) for acceleration.
//! * **Extended Adapters**: Wraps core adapters with parallel implementation logic.
//! * **Feature-Gated**: Parallelism is configurable via crate features.
//!
//! ### Configuration Flow
//!
//! 1. Create a [`LoessBuilder`] via `Loess::new()`.
//! 2. Chain configuration methods (`.fraction()`, `.iterations()`, etc.).
//! 3. Select an adapter via `.adapter(Batch)` to get a parallel execution builder.

// Feature-gated imports
#[cfg(feature = "cpu")]
use crate::adapters::batch::ParallelBatchLoessBuilder;
#[cfg(feature = "cpu")]
use crate::adapters::online::ParallelOnlineLoessBuilder;
#[cfg(feature = "cpu")]
use crate::adapters::streaming::ParallelStreamingLoessBuilder;

// Import base marker types for delegation
use loess_rs::internals::api::Batch as BaseBatch;
use loess_rs::internals::api::Online as BaseOnline;
use loess_rs::internals::api::Streaming as BaseStreaming;

// Import the base adapter types
use loess_rs::internals::algorithms::regression::SolverLinalg;
use loess_rs::internals::api::LoessAdapter;
use loess_rs::internals::api::LoessBuilder;
use loess_rs::internals::math::distance::DistanceLinalg;
use loess_rs::internals::math::linalg::FloatLinalg;

use std::fmt::Debug;

// Publicly re-exported types
pub use loess_rs::internals::adapters::online::UpdateMode;
pub use loess_rs::internals::adapters::streaming::MergeStrategy;
pub use loess_rs::internals::algorithms::regression::ZeroWeightFallback;
pub use loess_rs::internals::algorithms::robustness::RobustnessMethod;
pub use loess_rs::internals::engine::output::LoessResult;
pub use loess_rs::internals::evaluation::cv::{KFold, LOOCV};
pub use loess_rs::internals::math::boundary::BoundaryPolicy;
pub use loess_rs::internals::math::kernel::WeightFunction;
pub use loess_rs::internals::math::scaling::ScalingMethod;
pub use loess_rs::internals::primitives::backend::Backend;
pub use loess_rs::internals::primitives::errors::LoessError;

// ============================================================================
// Adapter Module
// ============================================================================

/// Adapter selection namespace.
#[allow(non_snake_case)]
pub mod Adapter {
    pub use super::{Batch, Online, Streaming};
}

// ============================================================================
// Adapter Marker Types
// ============================================================================

/// Marker for parallel in-memory batch processing.
#[derive(Debug, Clone, Copy)]
pub struct Batch;

#[cfg(feature = "cpu")]
impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Debug + Send + Sync + 'static> LoessAdapter<T>
    for Batch
{
    type Output = ParallelBatchLoessBuilder<T>;

    fn convert(builder: LoessBuilder<T>) -> Self::Output {
        // Determine parallel mode: user choice OR default to true for fastLoess Batch
        let parallel = builder.parallel.unwrap_or(true);

        // Delegate to base implementation to create base builder
        let mut base = <BaseBatch as LoessAdapter<T>>::convert(builder);
        base.parallel = Some(parallel);

        // Wrap with extension fields
        ParallelBatchLoessBuilder { base }
    }
}

/// Marker for parallel chunked streaming processing.
#[derive(Debug, Clone, Copy)]
pub struct Streaming;

#[cfg(feature = "cpu")]
impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Debug + Send + Sync + 'static> LoessAdapter<T>
    for Streaming
{
    type Output = ParallelStreamingLoessBuilder<T>;

    fn convert(builder: LoessBuilder<T>) -> Self::Output {
        // Determine parallel mode: user choice OR default to true for fastLoess Streaming
        let parallel = builder.parallel.unwrap_or(true);

        // Delegate to base implementation to create base builder
        let mut base = <BaseStreaming as LoessAdapter<T>>::convert(builder);
        base.parallel = Some(parallel);

        // Wrap with extension fields
        ParallelStreamingLoessBuilder { base }
    }
}

/// Marker for incremental online processing with parallel support.
#[derive(Debug, Clone, Copy)]
pub struct Online;

#[cfg(feature = "cpu")]
impl<T: FloatLinalg + DistanceLinalg + SolverLinalg + Debug + Send + Sync + 'static> LoessAdapter<T>
    for Online
{
    type Output = ParallelOnlineLoessBuilder<T>;

    fn convert(builder: LoessBuilder<T>) -> Self::Output {
        // Determine parallel mode: user choice OR default to false for fastLoess Online
        let parallel = builder.parallel.unwrap_or(false);

        // Delegate to base implementation to create base builder
        let mut base = <BaseOnline as LoessAdapter<T>>::convert(builder);
        base.parallel = Some(parallel);

        // Wrap with extension fields
        ParallelOnlineLoessBuilder { base }
    }
}
