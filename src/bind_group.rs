use std::fmt;

use crate::buffer::Buffer;
use crate::sampler::Sampler;
use crate::shader::ShaderStage;
use crate::texture::TextureView;

/// What a bind group slot holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingType {
    /// Uniform buffer.
    UniformBuffer,
    /// Storage buffer (compute only).
    StorageBuffer,
    /// Sampler.
    Sampler,
    /// Sampled texture view.
    SampledTexture,
    /// Storage texture view (compute only).
    StorageTexture,
}

/// One slot in a [`BindGroupLayout`].
///
/// # Examples
///
/// ```rust
/// use wrfgx::{BindGroupLayoutEntry, BindingType, ShaderStage};
///
/// let entry = BindGroupLayoutEntry {
///     binding: 0,
///     visibility: ShaderStage::Vertex,
///     ty: BindingType::UniformBuffer,
/// };
/// assert_eq!(entry.binding, 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutEntry {
    /// Slot index.
    pub binding: u32,
    /// Stages that read this slot.
    pub visibility: ShaderStage,
    /// Slot kind.
    pub ty: BindingType,
}

/// A bind group layout, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroupLayout(u64);

impl BindGroupLayout {
    /// Wraps a backend layout id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend layout id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Which buffer a bind group entry points at.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{Buffer, BufferBinding};
///
/// let binding = BufferBinding {
///     buffer: Buffer::from_raw(1),
///     offset: 0,
///     size: None,
/// };
/// assert_eq!(binding.offset, 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferBinding {
    /// Source buffer.
    pub buffer: Buffer,
    /// Byte offset.
    pub offset: u64,
    /// Byte size (`None` = rest of buffer).
    pub size: Option<u64>,
}

/// The resource bound to one slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindGroupEntryData {
    /// A buffer range.
    Buffer(BufferBinding),
    /// A sampler.
    Sampler(Sampler),
    /// A texture view.
    Texture(TextureView),
}

/// One bound slot in a [`BindGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroupEntry {
    /// Slot index.
    pub binding: u32,
    /// Bound resource.
    pub data: BindGroupEntryData,
}

/// A bind group, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindGroup(u64);

impl BindGroup {
    /// Wraps a backend group id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend group id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for BindingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
