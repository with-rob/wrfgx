use crate::error::GpuError;
use crate::extent::Extent2D;
use crate::limits::LIMITS;
use crate::present_mode::PresentMode;
use crate::surface_format::SurfaceFormat;

/// Swapchain creation parameters.
///
/// `format` and `present_mode` are concrete enums shared by every
/// backend, so the same config works on Vulkan, OpenGL and WebGPU.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{Extent2D, PresentMode, SurfaceFormat, SwapchainConfig};
///
/// let config = SwapchainConfig {
///     extent: Extent2D::new(800, 600),
///     format: SurfaceFormat::B8G8R8A8Srgb,
///     present_mode: PresentMode::Fifo,
///     image_count: 3,
/// };
/// assert_eq!(config.image_count, 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapchainConfig {
    /// Swapchain size in physical pixels.
    pub extent: Extent2D,
    /// Image format shared by every backend.
    pub format: SurfaceFormat,
    /// Present mode shared by every backend.
    pub present_mode: PresentMode,
    /// Number of swapchain images.
    pub image_count: u32,
}

impl SwapchainConfig {
    /// Creates a validated swapchain configuration.
    ///
    /// Rejects an empty extent (`SurfaceLost`): every backend needs
    /// a real size. Clamps `image_count` into [`LIMITS`] — the same
    /// bounds on Vulkan, OpenGL and WebGPU, so backends never ask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wrfgx::{Extent2D, GpuError, PresentMode, SurfaceFormat, SwapchainConfig};
    ///
    /// let config = SwapchainConfig::new(
    ///     Extent2D::new(800, 600),
    ///     SurfaceFormat::B8G8R8A8Srgb,
    ///     PresentMode::Fifo,
    ///     1,
    /// )?;
    /// assert_eq!(config.image_count, 2);
    ///
    /// let empty = SwapchainConfig::new(
    ///     Extent2D::new(0, 0),
    ///     SurfaceFormat::B8G8R8A8Srgb,
    ///     PresentMode::Fifo,
    ///     3,
    /// );
    /// assert_eq!(empty, Err(GpuError::SurfaceLost));
    /// # Ok::<(), GpuError>(())
    /// ```
    pub fn new(
        extent: Extent2D,
        format: SurfaceFormat,
        present_mode: PresentMode,
        image_count: u32,
    ) -> Result<Self, GpuError> {
        if extent.is_empty() {
            return Err(GpuError::SurfaceLost);
        }
        let image_count = if LIMITS.max_swapchain_images == 0 {
            image_count.max(LIMITS.min_swapchain_images)
        } else {
            image_count.clamp(LIMITS.min_swapchain_images, LIMITS.max_swapchain_images)
        };
        Ok(Self {
            extent,
            format,
            present_mode,
            image_count,
        })
    }

    /// Returns a copy with a different extent, re-validated.
    pub fn with_extent(self, extent: Extent2D) -> Result<Self, GpuError> {
        Self::new(extent, self.format, self.present_mode, self.image_count)
    }
}
