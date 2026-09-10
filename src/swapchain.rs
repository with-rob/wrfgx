use crate::error::GpuError;
use crate::extent::Extent2D;
use crate::limits::LIMITS;

/// Swapchain creation parameters.
///
/// Backend-defined numeric codes: each backend maps `format` and
/// `present_mode` to its native values (`VkFormat` /
/// `VkPresentModeKHR` on Vulkan, internal format / swap interval
/// on OpenGL). `0` means "backend default".
///
/// # Examples
///
/// ```rust
/// use wrfgx::{Extent2D, SwapchainConfig};
///
/// let config = SwapchainConfig {
///     extent: Extent2D::new(800, 600),
///     format: 0,
///     present_mode: 0,
///     image_count: 3,
/// };
/// assert_eq!(config.image_count, 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapchainConfig {
    /// Swapchain size in physical pixels.
    pub extent: Extent2D,
    /// Backend-defined image format code.
    pub format: u32,
    /// Backend-defined present mode code.
    pub present_mode: u32,
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
    /// use wrfgx::{Extent2D, GpuError, SwapchainConfig};
    ///
    /// let config = SwapchainConfig::new(Extent2D::new(800, 600), 0, 0, 1)?;
    /// assert_eq!(config.image_count, 2);
    ///
    /// let empty = SwapchainConfig::new(Extent2D::new(0, 0), 0, 0, 3);
    /// assert_eq!(empty, Err(GpuError::SurfaceLost));
    /// # Ok::<(), GpuError>(())
    /// ```
    pub fn new(
        extent: Extent2D,
        format: u32,
        present_mode: u32,
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
}
