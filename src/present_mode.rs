use std::fmt;

/// How frames are presented to the display.
///
/// Every backend supports `Fifo`; `Mailbox` and `Immediate`
/// are optional but widely available.
///
/// # Examples
///
/// ```rust
/// use wrfgx::PresentMode;
///
/// assert_eq!(PresentMode::default(), PresentMode::Fifo);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PresentMode {
    /// Vertical sync, always available. Frames wait for the
    /// next display refresh.
    #[default]
    Fifo,
    /// Triple-buffered, no tearing. Typically available.
    Mailbox,
    /// No sync, frames display as soon as ready. Tearing
    /// possible.
    Immediate,
}

impl fmt::Display for PresentMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Fifo => "Fifo",
            Self::Mailbox => "Mailbox",
            Self::Immediate => "Immediate",
        };
        write!(f, "{name}")
    }
}
