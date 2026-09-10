use std::fmt;

/// Why a GPU operation failed.
///
/// Same kinds on every backend (Vulkan, OpenGL, WebGPU).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuError {
    /// GPU or requested API not available on this platform.
    Unsupported,
    /// Surface destroyed (resize to zero, minimize).
    SurfaceLost,
    /// Swapchain no longer matches the surface, recreate it.
    OutOfDate,
    /// Anything else.
    Unknown,
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => write!(f, "GPU not supported"),
            Self::SurfaceLost => write!(f, "surface lost"),
            Self::OutOfDate => write!(f, "swapchain out of date"),
            Self::Unknown => write!(f, "GPU error"),
        }
    }
}

impl std::error::Error for GpuError {}
