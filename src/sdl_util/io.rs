use std::ffi::c_void;
use std::io::{Read, Seek, SeekFrom, ErrorKind};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{self, NonNull};
use std::slice;

use sdl3_sys::iostream::*;

use super::{AsSdlExt, sdl_assert, sdl_panic};

/// An SDL IO stream for a [`Read`] type.
///
/// The stream will be closed when dropped. SDL may attempt to close the stream,
/// but it will do nothing.
pub struct SdlIoStream<'a, T>(NonNull<SDL_IOStream>, PhantomData<&'a mut T>);

impl<'a, T: Read> SdlIoStream<'a, T> {

	/// Opens a new SDL IO stream that allows reading.
	pub fn new_read(bytes: &'a mut T) -> Self {
		// Creates IO stream
		let interface = SDL_IOStreamInterface {
			version: size_of::<SDL_IOStreamInterface>() as u32,
			size:    Some(iface_size_fail),
			seek:    Some(iface_seek_fail),
			read:    Some(iface_read::<T>),
			write:   Some(iface_write_fail_readonly),
			flush:   Some(iface_flush_fail_readonly),
			close:   Some(iface_close_noop),
		};
		let stream_ptr = unsafe { SDL_OpenIO(&interface, ptr::from_mut(bytes) as *mut c_void) };
		let Some(stream_non_null) = NonNull::new(stream_ptr) else { sdl_panic!() };
		// Returns
		Self(stream_non_null, PhantomData)
	}

}

impl<'a, T: Read + Seek> SdlIoStream<'a, T> {

	/// Opens a new SDL IO stream that allows reading and seeking.
	pub fn new_read_seek(bytes: &'a mut T) -> Self {
		// Creates IO stream
		let interface = SDL_IOStreamInterface {
			version: size_of::<SDL_IOStreamInterface>() as u32,
			size:    Some(iface_size_fail),
			seek:    Some(iface_seek::<T>),
			read:    Some(iface_read::<T>),
			write:   Some(iface_write_fail_readonly),
			flush:   Some(iface_flush_fail_readonly),
			close:   Some(iface_close_noop),
		};
		let stream_ptr = unsafe { SDL_OpenIO(&interface, ptr::from_mut(bytes) as *mut c_void) };
		let Some(stream_non_null) = NonNull::new(stream_ptr) else { sdl_panic!() };
		// Returns
		Self(stream_non_null, PhantomData)
	}

}

impl<'a, T> AsSdlExt for SdlIoStream<'a, T> {

	type Sdl = *mut SDL_IOStream;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl<'a, T> Drop for SdlIoStream<'a, T> {

	fn drop(&mut self) {
		sdl_assert!(unsafe { SDL_CloseIO(self.as_sdl()) });
	}

}

/// An [`SDL_IOStreamInterface::size`] callback.
///
/// Always returns -1.
extern "C" fn iface_size_fail(_: *mut c_void) -> i64 {
	-1
}

/// An [`SDL_IOStreamInterface::seek`] callback.
///
/// Seeks to `offset` relative to `whence`. Returns the final offset in the data
/// stream, or `-1` on error.
extern "C" fn iface_seek<T: Seek>(userdata: *mut c_void, offset: i64, whence: SDL_IOWhence) -> i64 {
	// SAFETY: `userdata` is created from a mutable reference in
	// `SdlIoStream::new_read()` or `SdlIoStream::new_read_seek()`
	let bytes = unsafe { (userdata as *mut T).as_mut_unchecked() };
	let pos = match whence {
		SDL_IO_SEEK_SET => SeekFrom::Start(offset as u64),
		SDL_IO_SEEK_CUR => SeekFrom::Current(offset),
		SDL_IO_SEEK_END => SeekFrom::End(offset),
		_               => panic!("Unknown `SDL_IOWhence` variant: {}", whence.0),
	};
	bytes.seek(pos).map_or(-1, |i| i64::try_from(i).expect("Seek position should be convertible to `i64`"))
}

/// An [`SDL_IOStreamInterface::seek`] callback.
///
/// Always returns -1.
extern "C" fn iface_seek_fail(_: *mut c_void, _: i64, _: SDL_IOWhence) -> i64 {
	-1
}

/// An [`SDL_IOStreamInterface::read`] callback.
///
/// Reads up to `size` bytes from the data stream to the buffer pointed at by
/// `ptr`. On an incomplete read, sets `status` to an `SDL_IOStatus` value.
/// Returns the number of bytes read.
extern "C" fn iface_read<T: Read>(userdata: *mut c_void, ptr: *mut c_void, size: usize, status_ptr: *mut SDL_IOStatus) -> usize {
	// Converts arguments
	// SAFETY: `userdata` is created from a mutable reference in
	// `SdlIoStream::new_read()` or `SdlIoStream::new_read_seek()`
	let bytes = unsafe { (userdata as *mut T).as_mut_unchecked() };
	let buf = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, size) };
	// Reads data into buffer
	let bytes_read;
	let status;
	match bytes.read(buf) {
		Ok(n) => {
			bytes_read = n;
			status = if bytes_read == 0 && buf.len() != 0 { SDL_IO_STATUS_EOF } else { SDL_IO_STATUS_READY };
		}
		Err(err) => {
			bytes_read = 0;
			status = if err.kind() == ErrorKind::Interrupted { SDL_IO_STATUS_READY } else { SDL_IO_STATUS_ERROR };
		}
	}
	// Writes status
	unsafe { status_ptr.write(status); }
	// Returns number of bytes read
	bytes_read
}

/// An [`SDL_IOStreamInterface::write`] callback.
///
/// Always sets status to `SDL_IO_STATUS_READONLY` and returns 0.
extern "C" fn iface_write_fail_readonly(_userdata: *mut c_void, _ptr: *const c_void, _size: usize, status: *mut SDL_IOStatus) -> usize {
	unsafe { status.write(SDL_IO_STATUS_READONLY); }
	0
}

/// An [`SDL_IOStreamInterface::flush`] callback.
///
/// Always sets status to `SDL_IO_STATUS_READONLY` and returns false.
extern "C" fn iface_flush_fail_readonly(_userdata: *mut c_void, status: *mut SDL_IOStatus) -> bool {
	unsafe { status.write(SDL_IO_STATUS_READONLY); }
	false
}

/// An [`SDL_IOStreamInterface::close`] callback.
///
/// Does nothing. Always returns true.
extern "C" fn iface_close_noop(_userdata: *mut c_void) -> bool {
	true
}