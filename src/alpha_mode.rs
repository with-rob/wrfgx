use std::fmt;

/// How the swapchain alpha channel composites with the window.
///
/// # Examples
///
/// ```rust
/// use wrfgx::AlphaMode;
///
/// assert_eq!(AlphaMode::default(), AlphaMode::Opaque);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AlphaMode {
    /// Alpha ignored, surface is opaque.
    #[default]
    Opaque,
    /// Alpha pre-multiplied into color channels.
    PreMultiplied,
}

impl fmt::Display for AlphaMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Opaque => "Opaque",
            Self::PreMultiplied => "PreMultiplied",
        };
        write!(f, "{name}")
    }
}
