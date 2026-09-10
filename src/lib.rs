//! # wrfgx
//!
//! Low-level GPU vocabulary shared by every backend (Vulkan,
//! OpenGL, WebGPU, Metal, DirectX, WebGL2).
//!
//! Plain structs and enums only: no traits, no implementations, no
//! capabilities queries. Every backend honors the same
//! [`Limits`]; anything that cannot work on all GPUs stays out.
//!
//! Compute types ([`ComputePipelineDescriptor`], [`DispatchCommand`])
//! need the `compute` feature (on by default); WebGL2 has no
//! compute shaders.

/// Swapchain compositing mode.
pub mod alpha_mode;
/// GPU bind groups.
pub mod bind_group;
/// GPU buffers.
pub mod buffer;
/// GPU draw and state commands.
pub mod commands;
#[cfg(feature = "compute")]
/// Compute pipelines and dispatches.
pub mod compute;
/// GPU error kinds.
pub mod error;
/// Surface and swapchain extents.
pub mod extent;
/// Fixed limits honored by every backend.
pub mod limits;
/// Graphics pipelines.
pub mod pipeline;
/// How frames are presented to the display.
pub mod present_mode;
/// Render passes and attachments.
pub mod render_pass;
/// GPU samplers.
pub mod sampler;
/// GPU shaders.
pub mod shader;
/// Image formats for swapchain surfaces.
pub mod surface_format;
/// Swapchain creation parameters.
pub mod swapchain;
/// GPU textures.
pub mod texture;
/// Vertex input layouts.
pub mod vertex;

pub use alpha_mode::AlphaMode;
pub use bind_group::{
    BindGroup, BindGroupEntry, BindGroupEntryData, BindGroupLayout, BindGroupLayoutEntry,
    BindingType, BufferBinding,
};
pub use buffer::{Buffer, BufferDescriptor, BufferUsage};
pub use commands::{
    DrawCommand, DrawIndexedCommand, IndexFormat, RenderCommands, Scissor, Viewport,
};
#[cfg(feature = "compute")]
pub use compute::{ComputePipeline, ComputePipelineDescriptor, DispatchCommand};
pub use error::GpuError;
pub use extent::Extent2D;
pub use limits::{LIMITS, Limits};
pub use pipeline::{
    BlendFactor, BlendOp, BlendState, CompareOp, CullMode, DepthState, FrontFace, Pipeline,
    PipelineDescriptor, PrimitiveTopology, StencilFaceState, StencilOp, StencilState,
};
pub use present_mode::PresentMode;
pub use render_pass::{
    Attachment, AttachmentRef, ClearColor, ImageLayout, LoadOp, RenderPass, StoreOp,
};
pub use sampler::{AddressMode, FilterMode, Sampler, SamplerDescriptor};
pub use shader::{ShaderFormat, ShaderModule, ShaderSource, ShaderStage};
pub use surface_format::SurfaceFormat;
pub use swapchain::SwapchainConfig;
pub use texture::{
    Texture, TextureDescriptor, TextureFormat, TextureUsage, TextureView, TextureViewDescriptor,
};
pub use vertex::{VertexAttribute, VertexBufferLayout, VertexFormat};
