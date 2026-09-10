//! # wrfgx
//!
//! Low-level GPU primitives shared by every backend
//! (Vulkan, OpenGL, WebGPU).
//!
//! Plain structs and enums only: no traits, no implementations, no
//! capabilities queries. Every backend honors the same
//! [`Limits`]; anything that cannot work on all GPUs stays out.

/// GPU error kinds.
pub mod error;
/// Surface and swapchain extents.
pub mod extent;
/// Fixed limits honored by every backend.
pub mod limits;
/// How frames are presented to the display.
pub mod present_mode;
/// Image formats for swapchain surfaces.
pub mod surface_format;
/// Swapchain creation parameters.
pub mod swapchain;

pub use error::GpuError;
pub use extent::Extent2D;
pub use limits::{LIMITS, Limits};
pub use present_mode::PresentMode;
pub use surface_format::SurfaceFormat;
pub use swapchain::SwapchainConfig;
