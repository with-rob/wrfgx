use std::fmt;

/// Why a GPU operation failed.
///
/// Same kinds on every backend (Vulkan, OpenGL, WebGPU, Metal,
/// DirectX, WebGL2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuError {
    /// GPU or requested API not available on this platform.
    Unsupported,
    /// Surface destroyed (resize to zero, minimize).
    SurfaceLost,
    /// Swapchain no longer matches the surface, recreate it.
    OutOfDate,
    /// GPU reset or driver error.
    DeviceLost,
    /// Video memory exhausted.
    OutOfMemory,
    /// Backend lacks a required feature (for example compute on
    /// WebGL2).
    FeatureMissing,
    /// No backend supports the requested surface format.
    FormatNotSupported,
    /// Anything else.
    Unknown,
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Unsupported => "GPU not supported",
            Self::SurfaceLost => "surface lost",
            Self::OutOfDate => "swapchain out of date",
            Self::DeviceLost => "device lost",
            Self::OutOfMemory => "out of video memory",
            Self::FeatureMissing => "GPU feature missing",
            Self::FormatNotSupported => "surface format not supported",
            Self::Unknown => "GPU error",
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for GpuError {}
