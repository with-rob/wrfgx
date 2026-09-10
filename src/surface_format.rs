use std::fmt;

use crate::texture::TextureFormat;

/// Image format for a swapchain surface.
///
/// Only formats available on every backend. Anything exotic stays
/// out.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{SurfaceFormat, TextureFormat};
///
/// let format = SurfaceFormat::B8G8R8A8Srgb;
/// assert_eq!(format.bytes_per_pixel(), 4);
/// assert_eq!(
///     format.to_texture_format(),
///     TextureFormat::B8G8R8A8Srgb
/// );
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

    /// The equivalent [`TextureFormat`].
    pub const fn to_texture_format(self) -> TextureFormat {
        match self {
            Self::B8G8R8A8Srgb => TextureFormat::B8G8R8A8Srgb,
            Self::B8G8R8A8Unorm => TextureFormat::B8G8R8A8Unorm,
            Self::R8G8B8A8Srgb => TextureFormat::Rgba8UnormSrgb,
            Self::R8G8B8A8Unorm => TextureFormat::Rgba8Unorm,
        }
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
