use std::fmt;

/// Texture filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FilterMode {
    /// Nearest texel.
    #[default]
    Nearest,
    /// Linear interpolation.
    Linear,
}

/// How texture coordinates outside `[0, 1]` wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AddressMode {
    /// Repeat the texture.
    #[default]
    Repeat,
    /// Mirrored repeat.
    MirroredRepeat,
    /// Clamp to the edge texel.
    ClampToEdge,
}

/// Sampler creation parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{AddressMode, FilterMode, SamplerDescriptor};
///
/// let desc = SamplerDescriptor {
///     min_filter: FilterMode::Linear,
///     mag_filter: FilterMode::Linear,
///     mipmap_filter: FilterMode::Linear,
///     address_u: AddressMode::Repeat,
///     address_v: AddressMode::Repeat,
///     address_w: AddressMode::ClampToEdge,
///     max_anisotropy: None,
/// };
/// assert_eq!(desc.min_filter, FilterMode::Linear);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerDescriptor {
    /// Minification filter.
    pub min_filter: FilterMode,
    /// Magnification filter.
    pub mag_filter: FilterMode,
    /// Mipmap filter.
    pub mipmap_filter: FilterMode,
    /// U wrap mode.
    pub address_u: AddressMode,
    /// V wrap mode.
    pub address_v: AddressMode,
    /// W wrap mode.
    pub address_w: AddressMode,
    /// Anisotropy level (`None` = off).
    pub max_anisotropy: Option<u32>,
}

/// A GPU sampler, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sampler(u64);

impl Sampler {
    /// Wraps a backend sampler id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend sampler id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for FilterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
