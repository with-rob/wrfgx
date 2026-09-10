use std::fmt;

use crate::texture::TextureFormat;

/// What happens to an attachment when a pass starts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadOp {
    /// Clear to a color.
    Clear(ClearColor),
    /// Keep previous contents.
    Load,
    /// Previous contents undefined.
    DontCare,
}

/// What happens to an attachment when a pass ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StoreOp {
    /// Keep rendered contents.
    #[default]
    Store,
    /// Contents undefined after the pass.
    DontCare,
}

/// Clear color value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearColor {
    /// Floating-point RGBA.
    Float([f32; 4]),
    /// Signed integer RGBA.
    Int([i32; 4]),
    /// Unsigned integer RGBA.
    Uint([u32; 4]),
}

/// Expected image layout inside a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageLayout {
    /// Undefined, backend picks.
    #[default]
    Undefined,
    /// Color attachment.
    ColorAttachment,
    /// Depth/stencil attachment.
    DepthStencilAttachment,
    /// Ready to present.
    PresentSrc,
    /// Sampled from shaders.
    ShaderReadOnly,
}

/// One render pass attachment.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{Attachment, ClearColor, LoadOp, StoreOp, TextureFormat};
///
/// let attachment = Attachment {
///     format: TextureFormat::B8G8R8A8Srgb,
///     samples: 1,
///     load: LoadOp::Clear(ClearColor::Float([0.0, 0.0, 0.0, 1.0])),
///     store: StoreOp::Store,
/// };
/// assert_eq!(attachment.samples, 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attachment {
    /// Image format.
    pub format: TextureFormat,
    /// Sample count (`1` = no MSAA).
    pub samples: u32,
    /// Start-of-pass behavior.
    pub load: LoadOp,
    /// End-of-pass behavior.
    pub store: StoreOp,
}

/// Reference to one attachment from a subpass description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttachmentRef {
    /// Attachment index.
    pub attachment: u32,
    /// Layout during the pass.
    pub layout: ImageLayout,
}

/// A render pass: which attachments to draw into and how.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{Attachment, ClearColor, LoadOp, RenderPass, StoreOp, TextureFormat};
///
/// let pass = RenderPass {
///     color_attachments: vec![Attachment {
///         format: TextureFormat::Rgba8Unorm,
///         samples: 1,
///         load: LoadOp::Clear(ClearColor::Float([0.1, 0.1, 0.2, 1.0])),
///         store: StoreOp::Store,
///     }],
///     depth_stencil: None,
/// };
/// assert_eq!(pass.color_attachments.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RenderPass {
    /// Color targets in draw order.
    pub color_attachments: Vec<Attachment>,
    /// Depth/stencil target (`None` = no depth).
    pub depth_stencil: Option<Attachment>,
}

impl fmt::Display for StoreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for ImageLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
