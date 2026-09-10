//! Compute types, available with the `compute` feature.
//!
//! WebGL2 has no compute shaders, so anything here stays behind
//! this feature. Desktop and WebGPU backends enable it by default.

use crate::bind_group::BindGroupLayout;
use crate::shader::ShaderModule;

/// Compute pipeline creation parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::{BindGroupLayout, ComputePipelineDescriptor, ShaderModule};
///
/// let desc = ComputePipelineDescriptor {
///     shader: ShaderModule::from_raw(1),
///     layout: BindGroupLayout::from_raw(2),
/// };
/// assert_eq!(desc.shader.raw(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputePipelineDescriptor {
    /// Compute shader.
    pub shader: ShaderModule,
    /// Bind group layout.
    pub layout: BindGroupLayout,
}

/// A compiled compute pipeline, created by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputePipeline(u64);

impl ComputePipeline {
    /// Wraps a backend pipeline id.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// The backend pipeline id.
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Compute dispatch parameters.
///
/// # Examples
///
/// ```rust
/// use wrfgx::DispatchCommand;
///
/// let dispatch = DispatchCommand {
///     groups_x: 8,
///     groups_y: 8,
///     groups_z: 1,
/// };
/// assert_eq!(dispatch.groups_x, 8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DispatchCommand {
    /// Workgroups on X.
    pub groups_x: u32,
    /// Workgroups on Y.
    pub groups_y: u32,
    /// Workgroups on Z.
    pub groups_z: u32,
}
