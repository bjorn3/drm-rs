//!
//! # DumbBuffer
//!
//! Memory-supported, slow, but easy & cross-platform buffer implementation
//!

use buffer;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// Slow, but generic `Buffer` implementation
pub struct DumbBuffer {
    pub(crate) size: (u32, u32),
    pub(crate) length: usize,
    pub(crate) format: buffer::format::PixelFormat,
    pub(crate) pitch: u32,
    pub(crate) handle: buffer::Handle,
}

/// Mapping of a dumbbuffer
pub struct DumbMapping<'a> {
    pub(crate) _phantom: core::marker::PhantomData<&'a ()>,
    pub(crate) map: &'a mut [u8],
}

impl<'a> AsMut<[u8]> for DumbMapping<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.map
    }
}

impl<'a> Drop for DumbMapping<'a> {
    fn drop(&mut self) {
        use nix::sys::mman;

        unsafe {
            mman::munmap(self.map.as_mut_ptr() as *mut _, self.map.len()).expect("Unmap failed");
        }
    }
}

impl buffer::Buffer for DumbBuffer {
    fn size(&self) -> (u32, u32) {
        self.size
    }
    fn format(&self) -> buffer::format::PixelFormat {
        self.format
    }
    fn pitch(&self) -> u32 {
        self.pitch
    }
    fn handle(&self) -> buffer::Handle {
        self.handle
    }
}
