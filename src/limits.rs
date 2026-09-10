/// Fixed limits honored by every backend.
///
/// No capabilities queries: these values hold on Vulkan, OpenGL
/// and WebGPU alike. Backends clamp to them instead of asking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Limits {
    /// Minimum swapchain image count.
    pub min_swapchain_images: u32,
    /// Maximum swapchain image count (`0` = no limit).
    pub max_swapchain_images: u32,
}

/// The limits every backend honors.
///
/// # Examples
///
/// ```rust
/// use wrfgx::LIMITS;
///
/// assert!(LIMITS.min_swapchain_images >= 2);
/// ```
pub const LIMITS: Limits = Limits {
    min_swapchain_images: 2,
    max_swapchain_images: 0,
};
