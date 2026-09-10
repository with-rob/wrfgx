/// Fixed limits honored by every backend.
///
/// No capabilities queries: these values hold on Vulkan, OpenGL,
/// WebGPU, Metal, DirectX and WebGL2 alike. Backends clamp to them
/// instead of asking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Limits {
    /// Minimum swapchain image count.
    pub min_swapchain_images: u32,
    /// Maximum swapchain image count (`0` = no limit).
    pub max_swapchain_images: u32,
    /// Maximum 2D texture dimension in pixels.
    pub max_texture_2d: u32,
    /// Maximum 3D texture dimension in pixels (`0` = unsupported).
    pub max_texture_3d: u32,
    /// Maximum storage buffers per shader stage (`0` = unsupported).
    pub max_storage_buffers: u32,
    /// Maximum sampled textures per shader stage.
    pub max_sampled_textures: u32,
    /// Maximum vertex buffers per draw.
    pub max_vertex_buffers: u32,
    /// Maximum vertex attributes per draw.
    pub max_vertex_attributes: u32,
    /// Maximum color attachments per render pass.
    pub max_color_attachments: u32,
    /// Maximum uniform buffer size in bytes.
    pub max_uniform_buffer_size: u64,
    /// Maximum compute workgroup size (`0` = unsupported).
    pub max_compute_workgroup_size: u32,
    /// Maximum sampler anisotropy.
    pub max_anisotropy: u32,
}

/// The limits every backend honors.
///
/// # Examples
///
/// ```rust
/// use wrfgx::LIMITS;
///
/// assert!(LIMITS.min_swapchain_images >= 2);
/// assert!(LIMITS.max_texture_2d >= 4096);
/// ```
pub const LIMITS: Limits = Limits {
    min_swapchain_images: 2,
    max_swapchain_images: 0,
    max_texture_2d: 4096,
    max_texture_3d: 256,
    max_storage_buffers: 8,
    max_sampled_textures: 16,
    max_vertex_buffers: 8,
    max_vertex_attributes: 16,
    max_color_attachments: 8,
    max_uniform_buffer_size: 16384,
    max_compute_workgroup_size: 256,
    max_anisotropy: 16,
};
