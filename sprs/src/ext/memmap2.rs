mod buf_mut;
mod val_mut;

pub use buf_mut::*;
pub use val_mut::*;

/// Docs from https://man7.org/linux/man-pages/man2/mmap.2.html
pub enum Mode {
    /// `MAP_SHARED` flag
    ///
    /// Share this mapping.  Updates to the mapping are visible to other processes mapping
    /// the same region, and (in the case of file-backed mappings) are carried through
    /// to the underlying file
    Shared,
    /// `MAP_PRIVATE` flag
    ///
    /// Create a private copy-on-write mapping.  Updates to the mapping are not visible
    /// to other processes mapping the same file, and are not carried through
    /// to the underlying file
    Private,
}
