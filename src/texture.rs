use std::fmt;

/// Texture image format.
///
/// Only formats expressible on every backend (Vulkan, OpenGL,
/// WebGPU, Metal, DirectX, WebGL2). Compressed formats (BC, ASTC,
/// ETC2, PVRTC) stay out: they vary completely per platform.
///
/// # Examples
///
/// ```rust
/// use wrfgx::TextureFormat;
///
/// assert_eq!(TextureFormat::Rgba8Unorm.bytes_per_pixel(), 4);
/// assert!(TextureFormat::Rgba8UnormSrgb.is_srgb());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFormat {
    /// 8-bit RGBA linear.
    Rgba8Unorm,
    /// 8-bit RGBA with sRGB nonlinear encoding.
    Rgba8UnormSrgb,
    /// 8-bit RGBA unsigned integer.
    Rgba8Uint,
    /// 16-bit float RGBA.
    Rgba16Float,
    /// 32-bit float RGBA.
    Rgba32Float,
    /// 8-bit red linear.
    R8Unorm,
    /// 16-bit float red.
    R16Float,
    /// 32-bit float red.
    R32Float,
    /// 8-bit red-green linear.
    Rg8Unorm,
    /// Shared-exponent float RGB.
    Rgb9E5Ufloat,
    /// 10-bit RGB + 2-bit alpha.
    Rgb10A2Unorm,
    /// 32-bit BGRA linear.
    B8G8R8A8Unorm,
    /// 32-bit BGRA with sRGB nonlinear encoding.
    B8G8R8A8Srgb,
    /// 16-bit depth.
    D16Unorm,
    /// 32-bit float depth.
    D32Float,
    /// 24-bit depth + 8-bit stencil.
    D24UnormS8Uint,
}

impl TextureFormat {
    /// Bytes per pixel (per texel block) for this format.
    pub const fn bytes_per_pixel(self) -> u32 {
        match self {
            Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Uint
            | Self::Rg8Unorm
            | Self::Rgb9E5Ufloat
            | Self::Rgb10A2Unorm
            | Self::B8G8R8A8Unorm
            | Self::B8G8R8A8Srgb => 4,
            Self::Rgba16Float | Self::D16Unorm => 8,
            Self::Rgba32Float | Self::D32Float | Self::D24UnormS8Uint => 16,
            Self::R8Unorm => 1,
            Self::R16Float => 2,
            Self::R32Float => 4,
        }
    }

    /// Whether this format uses sRGB nonlinear encoding.
    pub const fn is_srgb(self) -> bool {
        matches!(self, Self::Rgba8UnormSrgb | Self::B8G8R8A8Srgb)
    }

    /// Whether this format holds depth or stencil data.
    pub const fn is_depth(self) -> bool {
        matches!(self, Self::D16Unorm | Self::D32Float | Self::D24UnormS8Uint)
    }
}

impl fmt::Display for TextureFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// What a texture is used for.
///
/// Bit flags: combine with `|`. Backends reject combinations they
/// cannot express with [`GpuError::Unsupported`](crate::GpuError).
///
/// # Examples
///
/// ```rust
/// use wrfgx::TextureUsage;
///
/// let usage = TextureUsage::SAMPLED | TextureUsage::RENDER;
/// assert!(usage.contains(TextureUsage::SAMPLED));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TextureUsage(u32);

impl TextureUsage {
    /// Sampled (read) from shaders.
    pub const SAMPLED: Self = Self(0b0000_0001);
    /// Read and written from shaders (compute only).
    pub const STORAGE: Self = Self(0b0000_0010);
    /// Render attachment target.
    pub const RENDER: Self = Self(0b0000_0100);
    /// Copy source.
    pub const COPY_SRC: Self = Self(0b0000_1000);
    /// Copy destination.
    pub const COPY_DST: Self = Self(0b0001_0000);

    /// Whether `self` includes all bits of `other`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Raw bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl std::ops::BitOr for TextureUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Texture creation parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{TextureDescriptor, TextureFormat, TextureUsage};
///
/// let desc = TextureDescriptor {
///     format: TextureFormat::Rgba8Unorm,
///     width: 512,
///     height: 512,
///     depth: 1,
///     mip_levels: 1,
///     array_layers: 1,
///     samples: 1,
///     usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
/// };
/// assert_eq!(desc.width, 512);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureDescriptor {
    /// Image format.
    pub format: TextureFormat,
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// Depth in texels (`1` for 2D).
    pub depth: u32,
    /// Mipmap level count.
    pub mip_levels: u32,
    /// Array layer count (`1` when unused).
    pub array_layers: u32,
    /// Sample count (`1` = no MSAA).
    pub samples: u32,
    /// Allowed uses.
    pub usage: TextureUsage,
}

/// A GPU texture, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Texture(u64);

impl Texture {
    /// Wraps a backend texture id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend texture id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Which mips and layers of a [`Texture`] a view exposes.
///
/// # Examples
///
/// ```rust
/// use wrfgx::TextureViewDescriptor;
///
/// let desc = TextureViewDescriptor {
///     mip_range: 0..1,
///     layer_range: 0..1,
/// };
/// assert_eq!(desc.mip_range, 0..1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureViewDescriptor {
    /// Mipmap levels exposed.
    pub mip_range: std::ops::Range<u32>,
    /// Array layers exposed.
    pub layer_range: std::ops::Range<u32>,
}

/// A view into a [`Texture`], created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureView(u64);

impl TextureView {
    /// Wraps a backend view id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend view id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}
