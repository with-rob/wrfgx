use std::fmt;

/// What a buffer is used for.
///
/// Bit flags: combine with `|`. Backends reject combinations they
/// cannot express with [`GpuError::Unsupported`](crate::GpuError).
///
/// # Examples
///
/// ```rust
/// use wrfgx::BufferUsage;
///
/// let usage = BufferUsage::VERTEX | BufferUsage::COPY_DST;
/// assert!(usage.contains(BufferUsage::VERTEX));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BufferUsage(u32);

impl BufferUsage {
    /// Vertex buffer.
    pub const VERTEX: Self = Self(0b0000_0001);
    /// Index buffer.
    pub const INDEX: Self = Self(0b0000_0010);
    /// Uniform buffer.
    pub const UNIFORM: Self = Self(0b0000_0100);
    /// Storage buffer (compute only).
    pub const STORAGE: Self = Self(0b0000_1000);
    /// Copy source.
    pub const COPY_SRC: Self = Self(0b0001_0000);
    /// Copy destination.
    pub const COPY_DST: Self = Self(0b0010_0000);

    /// Whether `self` includes all bits of `other`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Raw bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl std::ops::BitOr for BufferUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl fmt::Display for BufferUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BufferUsage({:#08b})", self.0)
    }
}

/// Buffer creation parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{BufferDescriptor, BufferUsage};
///
/// let desc = BufferDescriptor {
///     size: 1024,
///     usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
///     mapped: false,
/// };
/// assert_eq!(desc.size, 1024);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferDescriptor {
    /// Size in bytes.
    pub size: u64,
    /// Allowed uses.
    pub usage: BufferUsage,
    /// Created persistently mapped (WebGL2 fallback).
    pub mapped: bool,
}

/// A GPU buffer, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Buffer(u64);

impl Buffer {
    /// Wraps a backend buffer id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend buffer id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}
