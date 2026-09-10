use std::fmt;

/// Vertex attribute data format.
///
/// Only formats expressible on every backend. Integer formats stay
/// out: WebGL2 cannot express them.
///
/// # Examples
///
/// ```rust
/// use wrfgx::VertexFormat;
///
/// assert_eq!(VertexFormat::Float32x3.size(), 12);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexFormat {
    /// One 32-bit float.
    Float32,
    /// Two 32-bit floats.
    Float32x2,
    /// Three 32-bit floats.
    Float32x3,
    /// Four 32-bit floats.
    Float32x4,
    /// Four normalized unsigned bytes.
    Unorm8x4,
}

impl VertexFormat {
    /// Size in bytes.
    pub const fn size(self) -> u32 {
        match self {
            Self::Float32 => 4,
            Self::Float32x2 => 8,
            Self::Float32x3 => 12,
            Self::Float32x4 | Self::Unorm8x4 => 16,
        }
    }
}

impl fmt::Display for VertexFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// One vertex attribute inside a vertex buffer.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{VertexAttribute, VertexFormat};
///
/// let attr = VertexAttribute {
///     format: VertexFormat::Float32x3,
///     offset: 0,
///     shader_location: 0,
/// };
/// assert_eq!(attr.format.size(), 12);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexAttribute {
    /// Data format.
    pub format: VertexFormat,
    /// Byte offset inside the vertex.
    pub offset: u32,
    /// Shader location.
    pub shader_location: u32,
}

/// Layout of one vertex buffer.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{VertexAttribute, VertexBufferLayout, VertexFormat};
///
/// let layout = VertexBufferLayout {
///     stride: 12,
///     instanced: false,
///     attributes: vec![VertexAttribute {
///         format: VertexFormat::Float32x3,
///         offset: 0,
///         shader_location: 0,
///     }],
/// };
/// assert!(!layout.instanced);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexBufferLayout {
    /// Byte stride between vertices.
    pub stride: u32,
    /// Steps per instance instead of per vertex.
    pub instanced: bool,
    /// Attributes in this buffer.
    pub attributes: Vec<VertexAttribute>,
}
