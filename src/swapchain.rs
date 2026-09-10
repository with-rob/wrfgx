use crate::extent::Extent2D;

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
