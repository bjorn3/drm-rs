//!
//! Error types
//!

use nix::errno::Errno;
use nix::Error as NixError;

/// Errors from system calls will always be in the form  NixError::Sys(errno).
///
/// This helper function unwraps a NixError into an Errno in places we know
/// other types of errors can not occur.
pub(crate) fn unwrap_errno(err: NixError) -> Errno {
    match err {
        NixError::Sys(errno) => errno,
        _ => unreachable!(),
    }
}

/// A general system error that can be returned by any DRM command.
///
/// Receiving this error likely indicates a bug in either the program, this
/// crate, or the underlying operating system.
#[derive(Debug, Fail)]
pub enum SystemError {
    /// A command was attempted using an invalid file descriptor.
    #[fail(display = "invalid file descriptor")]
    InvalidFileDescriptor,
    /// Provided memory area is inaccessible.
    ///
    /// Receiving this error indicates a bug in this crate.
    #[fail(display = "invalid memory access")]
    MemoryFault,
    /// One or more arguments used are invalid.
    ///
    /// This can be due to the system not supporting a feature or value.
    #[fail(display = "invalid argument")]
    InvalidArgument,
    /// A command was attempted using a non-DRM device.
    #[fail(display = "invalid file type")]
    InvalidFileType,
    /// Permission denied.
    #[fail(display = "permission denied")]
    PermissionDenied,
    /// Unknown system error.
    #[fail(display = "unknown system error: {}", errno)]
    Unknown {
        /// Unknown nix::Errno returned by the system call.
        errno: Errno,
    },
}

impl From<Errno> for SystemError {
    fn from(errno: Errno) -> SystemError {
        match errno {
            Errno::EBADF => SystemError::InvalidFileDescriptor,
            Errno::EFAULT => SystemError::MemoryFault,
            Errno::EINVAL => SystemError::InvalidArgument,
            Errno::ENOTTY => SystemError::InvalidFileDescriptor,
            Errno::EACCES => SystemError::PermissionDenied,
            _ => SystemError::Unknown { errno: errno },
        }
    }
}
