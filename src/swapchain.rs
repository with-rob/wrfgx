use crate::alpha_mode::AlphaMode;
use crate::error::GpuError;
use crate::extent::Extent2D;
use crate::limits::LIMITS;
use crate::present_mode::PresentMode;
use crate::surface_format::SurfaceFormat;
use crate::texture::TextureUsage;

/// Swapchain creation parameters.
///
/// `format` and `present_mode` are concrete enums shared by every
/// backend, so the same config works on Vulkan, OpenGL, WebGPU,
/// Metal, DirectX and WebGL2.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{AlphaMode, Extent2D, PresentMode, SurfaceFormat, SwapchainConfig, TextureUsage};
///
/// let config = SwapchainConfig::new(
///     Extent2D::new(800, 600),
///     SurfaceFormat::B8G8R8A8Srgb,
///     PresentMode::Fifo,
///     3,
/// )
/// .unwrap();
/// assert_eq!(config.image_count, 3);
/// assert_eq!(config.usage, TextureUsage::RENDER);
/// assert_eq!(config.alpha_mode, AlphaMode::Opaque);
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
    /// How swapchain images are used.
    pub usage: TextureUsage,
    /// Window compositing mode.
    pub alpha_mode: AlphaMode,
}

impl SwapchainConfig {
    /// Creates a validated swapchain configuration.
    ///
    /// Rejects an empty extent (`SurfaceLost`): every backend needs
    /// a real size. Clamps `image_count` into [`LIMITS`] — the same
    /// bounds everywhere, so backends never ask.
    ///
    /// `usage` defaults to `RENDER`, `alpha_mode` to `Opaque`;
    /// change them with [`with_usage`](Self::with_usage) and
    /// [`with_alpha_mode`](Self::with_alpha_mode).
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
            usage: TextureUsage::RENDER,
            alpha_mode: AlphaMode::Opaque,
        })
    }

    /// Returns a copy with a different extent, re-validated.
    pub fn with_extent(self, extent: Extent2D) -> Result<Self, GpuError> {
        Self::new(extent, self.format, self.present_mode, self.image_count)
    }

    /// Returns a copy with different image usage.
    pub fn with_usage(mut self, usage: TextureUsage) -> Self {
        self.usage = usage;
        self
    }

    /// Returns a copy with a different compositing mode.
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }
}
