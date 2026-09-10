use std::fmt;

/// Which shader stage a module runs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    /// Vertex stage.
    Vertex,
    /// Fragment stage.
    Fragment,
    /// Compute stage (requires the `compute` feature).
    Compute,
}

/// Which GPU language the shader bytes are written in.
///
/// Each backend consumes the format it understands; the user
/// compiles one Slang source to all formats at build time with the
/// `shaders` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderFormat {
    /// SPIR-V binary (Vulkan, DirectX).
    Spirv,
    /// Metal Shading Language source (Metal).
    Msl,
    /// HLSL source (DirectX via FXC/DXC).
    Hlsl,
    /// GLSL ES 3.0 source (OpenGL, WebGL2).
    GlslEs,
    /// WGSL source (WebGPU).
    Wgsl,
}

/// Shader source bytes in one [`ShaderFormat`].
///
/// # Examples
///
/// ```rust
/// use wrfgx::{ShaderFormat, ShaderSource, ShaderStage};
///
/// let source = ShaderSource {
///     stage: ShaderStage::Vertex,
///     format: ShaderFormat::Spirv,
///     code: vec![0x03, 0x02, 0x23, 0x07],
/// };
/// assert_eq!(source.stage, ShaderStage::Vertex);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderSource {
    /// Pipeline stage.
    pub stage: ShaderStage,
    /// Language of `code`.
    pub format: ShaderFormat,
    /// Shader bytes (binary or source text).
    pub code: Vec<u8>,
}

/// A compiled GPU shader, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderModule(u64);

impl ShaderModule {
    /// Wraps a backend shader id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend shader id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ShaderStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for ShaderFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
