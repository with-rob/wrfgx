//! # wrfgx
//!
//! Low-level GPU primitives shared by every backend
//! (Vulkan, OpenGL, WebGPU).
//!
//! Plain structs only: no traits, no implementations, no
//! capabilities queries. Every backend honors the same
//! [`Limits`]; anything that cannot work on all GPUs stays out.

/// GPU error kinds.
pub mod error;
/// Surface and swapchain extents.
pub mod extent;
/// Fixed limits honored by every backend.
pub mod limits;
/// Swapchain creation parameters.
pub mod swapchain;

pub use error::GpuError;
pub use extent::Extent2D;
pub use limits::{LIMITS, Limits};
pub use swapchain::SwapchainConfig;
