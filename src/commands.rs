use std::fmt;

use crate::bind_group::BindGroup;
use crate::buffer::Buffer;
use crate::pipeline::Pipeline;

/// Draw region in pixels.
///
/// # Examples
///
/// ```rust
/// use wrfgx::Viewport;
///
/// let viewport = Viewport {
///     x: 0.0,
///     y: 0.0,
///     width: 800.0,
///     height: 600.0,
///     min_depth: 0.0,
///     max_depth: 1.0,
/// };
/// assert_eq!(viewport.width, 800.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32,
    /// Minimum depth.
    pub min_depth: f32,
    /// Maximum depth.
    pub max_depth: f32,
}

/// Scissor rectangle in pixels.
///
/// # Examples
///
/// ```rust
/// use wrfgx::Scissor;
///
/// let scissor = Scissor {
///     x: 0,
///     y: 0,
///     width: 800,
///     height: 600,
/// };
/// assert_eq!(scissor.width, 800);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scissor {
    /// Left edge.
    pub x: i32,
    /// Top edge.
    pub y: i32,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// Index buffer element size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexFormat {
    /// 16-bit indices.
    Uint16,
    /// 32-bit indices.
    Uint32,
}

/// Non-indexed draw parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::DrawCommand;
///
/// let draw = DrawCommand {
///     vertex_count: 3,
///     instance_count: 1,
///     first_vertex: 0,
///     first_instance: 0,
/// };
/// assert_eq!(draw.vertex_count, 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawCommand {
    /// Vertices to draw.
    pub vertex_count: u32,
    /// Instances to draw.
    pub instance_count: u32,
    /// First vertex index.
    pub first_vertex: u32,
    /// First instance index.
    pub first_instance: u32,
}

/// Indexed draw parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::DrawIndexedCommand;
///
/// let draw = DrawIndexedCommand {
///     index_count: 3,
///     instance_count: 1,
///     first_index: 0,
///     first_instance: 0,
/// };
/// assert_eq!(draw.index_count, 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawIndexedCommand {
    /// Indices to draw.
    pub index_count: u32,
    /// Instances to draw.
    pub instance_count: u32,
    /// First index.
    pub first_index: u32,
    /// First instance index.
    pub first_instance: u32,
}

/// One GPU command inside a render pass.
///
/// Backends translate each variant to their native call
/// (`vkCmdDraw`, `glDraw*`, encoder methods, ...).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderCommands {
    /// Bind a graphics pipeline.
    SetPipeline(Pipeline),
    /// Set the viewport.
    SetViewport(Viewport),
    /// Set the scissor rectangle.
    SetScissor(Scissor),
    /// Bind a vertex buffer to a slot.
    SetVertexBuffer {
        /// Slot index.
        slot: u32,
        /// Source buffer.
        buffer: Buffer,
        /// Byte offset.
        offset: u64,
    },
    /// Bind the index buffer.
    SetIndexBuffer {
        /// Source buffer.
        buffer: Buffer,
        /// Byte offset.
        offset: u64,
        /// Element size.
        index_format: IndexFormat,
    },
    /// Bind a bind group to an index.
    SetBindGroup {
        /// Group index.
        index: u32,
        /// Bound group.
        group: BindGroup,
    },
    /// Non-indexed draw.
    Draw(DrawCommand),
    /// Indexed draw.
    DrawIndexed(DrawIndexedCommand),
}

impl fmt::Display for IndexFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
