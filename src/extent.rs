use std::fmt;

/// A surface size in physical pixels.
///
/// # Examples
///
/// ```rust
/// use wrfgx::Extent2D;
///
/// let extent = Extent2D::new(800, 600);
/// assert_eq!(extent.width, 800);
/// assert_eq!(extent.to_string(), "800x600");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent2D {
    /// Width in physical pixels.
    pub width: u32,
    /// Height in physical pixels.
    pub height: u32,
}

impl Extent2D {
    /// Creates a new extent.
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns `true` if either dimension is zero (no drawable area).
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl fmt::Display for Extent2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
