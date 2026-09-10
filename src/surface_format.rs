use std::fmt;

/// Image format for a swapchain surface.
///
/// Only formats available on every backend (Vulkan, OpenGL,
/// WebGPU). Anything exotic stays out.
///
/// # Examples
///
/// ```rust
/// use wrfgx::SurfaceFormat;
///
/// let format = SurfaceFormat::B8G8R8A8Srgb;
/// assert_eq!(format.bytes_per_pixel(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceFormat {
    /// 32-bit BGRA with sRGB nonlinear encoding.
    B8G8R8A8Srgb,
    /// 32-bit BGRA linear.
    B8G8R8A8Unorm,
    /// 32-bit RGBA with sRGB nonlinear encoding.
    R8G8B8A8Srgb,
    /// 32-bit RGBA linear.
    R8G8B8A8Unorm,
}

impl SurfaceFormat {
    /// Bytes per pixel for this format.
    pub const fn bytes_per_pixel(self) -> u32 {
        4
    }
}

impl fmt::Display for SurfaceFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::B8G8R8A8Srgb => "B8G8R8A8Srgb",
            Self::B8G8R8A8Unorm => "B8G8R8A8Unorm",
            Self::R8G8B8A8Srgb => "R8G8B8A8Srgb",
            Self::R8G8B8A8Unorm => "R8G8B8A8Unorm",
        };
        write!(f, "{name}")
    }
}
