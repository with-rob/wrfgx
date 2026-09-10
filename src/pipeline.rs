use std::fmt;

use crate::shader::ShaderModule;
use crate::texture::TextureFormat;
use crate::vertex::VertexBufferLayout;

/// How vertices connect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PrimitiveTopology {
    /// Separate triangles.
    #[default]
    TriangleList,
    /// Triangle strip.
    TriangleStrip,
    /// Separate lines.
    LineList,
    /// Line strip.
    LineStrip,
    /// Points.
    PointList,
}

/// Which faces to cull.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CullMode {
    /// Cull nothing.
    #[default]
    None,
    /// Cull front faces.
    Front,
    /// Cull back faces.
    Back,
}

/// Which winding is front-facing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FrontFace {
    /// Counter-clockwise winding.
    #[default]
    CounterClockwise,
    /// Clockwise winding.
    Clockwise,
}

/// Depth/stencil comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareOp {
    /// Never passes.
    Never,
    /// Passes when less.
    Less,
    /// Passes when equal.
    Equal,
    /// Passes when less or equal.
    LessOrEqual,
    /// Passes when greater.
    Greater,
    /// Passes when not equal.
    NotEqual,
    /// Passes when greater or equal.
    GreaterOrEqual,
    /// Always passes.
    Always,
}

/// Stencil operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StencilOp {
    /// Keep current value.
    #[default]
    Keep,
    /// Set to zero.
    Zero,
    /// Set to reference value.
    Replace,
    /// Increment, clamped.
    IncrementClamp,
    /// Decrement, clamped.
    DecrementClamp,
    /// Bitwise invert.
    Invert,
    /// Increment, wrapped.
    IncrementWrap,
    /// Decrement, wrapped.
    DecrementWrap,
}

/// Blend factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    /// Zero.
    Zero,
    /// One.
    One,
    /// Source alpha.
    SrcAlpha,
    /// One minus source alpha.
    OneMinusSrcAlpha,
    /// Destination alpha.
    DstAlpha,
    /// One minus destination alpha.
    OneMinusDstAlpha,
    /// Source color.
    SrcColor,
    /// One minus source color.
    OneMinusSrcColor,
}

/// Blend operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlendOp {
    /// Add source and destination.
    #[default]
    Add,
    /// Subtract destination from source.
    Subtract,
    /// Subtract source from destination.
    ReverseSubtract,
    /// Minimum of source and destination.
    Min,
    /// Maximum of source and destination.
    Max,
}

/// Color blending for one attachment.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{BlendFactor, BlendOp, BlendState};
///
/// let blend = BlendState {
///     enabled: true,
///     src_color: BlendFactor::SrcAlpha,
///     dst_color: BlendFactor::OneMinusSrcAlpha,
///     color_op: BlendOp::Add,
///     src_alpha: BlendFactor::One,
///     dst_alpha: BlendFactor::OneMinusSrcAlpha,
///     alpha_op: BlendOp::Add,
/// };
/// assert!(blend.enabled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendState {
    /// Whether blending applies.
    pub enabled: bool,
    /// Color source factor.
    pub src_color: BlendFactor,
    /// Color destination factor.
    pub dst_color: BlendFactor,
    /// Color operation.
    pub color_op: BlendOp,
    /// Alpha source factor.
    pub src_alpha: BlendFactor,
    /// Alpha destination factor.
    pub dst_alpha: BlendFactor,
    /// Alpha operation.
    pub alpha_op: BlendOp,
}

/// Depth testing and writing.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{CompareOp, DepthState};
///
/// let depth = DepthState {
///     test: Some(CompareOp::Less),
///     write: true,
/// };
/// assert!(depth.write);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DepthState {
    /// Comparison (`None` = no depth test).
    pub test: Option<CompareOp>,
    /// Whether passing fragments write depth.
    pub write: bool,
}

/// Stencil testing for one face.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{CompareOp, StencilFaceState, StencilOp};
///
/// let face = StencilFaceState {
///     test: CompareOp::Always,
///     fail_op: StencilOp::Keep,
///     depth_fail_op: StencilOp::Keep,
///     pass_op: StencilOp::Replace,
///     read_mask: 0xFF,
///     write_mask: 0xFF,
/// };
/// assert_eq!(face.read_mask, 0xFF);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilFaceState {
    /// Comparison.
    pub test: CompareOp,
    /// Op when stencil test fails.
    pub fail_op: StencilOp,
    /// Op when stencil passes but depth fails.
    pub depth_fail_op: StencilOp,
    /// Op when both pass.
    pub pass_op: StencilOp,
    /// Bits read during test.
    pub read_mask: u32,
    /// Bits written on update.
    pub write_mask: u32,
}

/// Stencil testing for both faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilState {
    /// Front faces.
    pub front: StencilFaceState,
    /// Back faces.
    pub back: StencilFaceState,
}

/// Graphics pipeline creation parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{CullMode, FrontFace, PipelineDescriptor, PrimitiveTopology, ShaderModule};
///
/// let desc = PipelineDescriptor {
///     vertex_shader: ShaderModule::from_raw(1),
///     fragment_shader: Some(ShaderModule::from_raw(2)),
///     vertex_buffers: vec![],
///     color_formats: vec![wrfgx::TextureFormat::Rgba8Unorm],
///     depth_format: None,
///     topology: PrimitiveTopology::TriangleList,
///     cull: CullMode::Back,
///     front_face: FrontFace::CounterClockwise,
///     depth: None,
///     stencil: None,
///     blend: None,
///     samples: 1,
/// };
/// assert_eq!(desc.samples, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineDescriptor {
    /// Vertex shader.
    pub vertex_shader: ShaderModule,
    /// Fragment shader (`None` = vertex-only pass).
    pub fragment_shader: Option<ShaderModule>,
    /// Vertex buffer layouts.
    pub vertex_buffers: Vec<VertexBufferLayout>,
    /// Color attachment formats.
    pub color_formats: Vec<TextureFormat>,
    /// Depth/stencil format (`None` = no depth).
    pub depth_format: Option<TextureFormat>,
    /// Primitive topology.
    pub topology: PrimitiveTopology,
    /// Face culling.
    pub cull: CullMode,
    /// Front face winding.
    pub front_face: FrontFace,
    /// Depth test and write (`None` = off).
    pub depth: Option<DepthState>,
    /// Stencil test (`None` = off).
    pub stencil: Option<StencilState>,
    /// Blending (`None` = opaque).
    pub blend: Option<BlendState>,
    /// Sample count (`1` = no MSAA).
    pub samples: u32,
}

/// A compiled graphics pipeline, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pipeline(u64);

impl Pipeline {
    /// Wraps a backend pipeline id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend pipeline id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PrimitiveTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for CullMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
