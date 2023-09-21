use std::{
    ffi::CStr, fs::Permissions, marker::PhantomData, os::unix::prelude::PermissionsExt, ptr,
};

use libcdio_sys::{
    udf_dirent_free, udf_dirent_t, udf_get_file_length, udf_get_filename, udf_get_link_count,
    udf_get_posix_filemode, udf_is_dir, udf_opendir, udf_readdir, udf_t,
};

/// Represents an owned, non-null `udf_dirent_s*`.
pub struct Dirent<'udf> {
    /// This is an [Option] because we might sometimes move it out of
    /// this struct, so calling `drop()` is unnecessary.
    ptr: *const udf_dirent_t,
    _udf: PhantomData<&'udf udf_t>,
}

impl<'udf> Dirent<'udf> {
    pub fn new(ptr: *const udf_dirent_t) -> Option<Self> {
        if ptr == ptr::null_mut() {
            None
        } else {
            Some(Self {
                ptr,
                _udf: PhantomData,
            })
        }
    }

    pub unsafe fn new_unchecked(ptr: *const udf_dirent_t) -> Self {
        Self {
            ptr,
            _udf: PhantomData,
        }
    }

    /// Advances this pointer using `udf_readdir()`.
    pub fn next_sibling(mut self) -> Option<Dirent<'udf>> {
        let next = unsafe { udf_readdir(self.ptr as *mut _) };
        self.ptr = ptr::null();
        Dirent::new(next)
    }

    pub fn child(&self) -> Option<Dirent<'udf>> {
        let child = unsafe { udf_opendir(self.ptr) };
        Dirent::new(child)
    }

    pub fn file_name<'a>(&'a self) -> Option<&'a CStr> {
        unsafe {
            let p_name = udf_get_filename(self.ptr);
            if p_name == ptr::null_mut() {
                None
            } else {
                Some(CStr::from_ptr(p_name))
            }
        }
    }

    pub fn is_dir(&self) -> bool {
        unsafe { udf_is_dir(self.ptr) != 0 }
    }

    pub fn link_count(&self) -> Option<u16> {
        unsafe {
            match udf_get_link_count(self.ptr) {
                0 => None,
                l => Some(l),
            }
        }
    }

    pub fn file_length(&self) -> Option<u32> {
        unsafe {
            match udf_get_file_length(self.ptr) {
                2147483647 => None,
                l => Some(l as u32),
            }
        }
    }

    pub fn posix_filemode(&self) -> Permissions {
        let mode = unsafe { udf_get_posix_filemode(self.ptr) };
        Permissions::from_mode(mode)
    }
}

impl<'a> Drop for Dirent<'a> {
    fn drop(&mut self) {
        unsafe {
            udf_dirent_free(self.ptr as *mut _);
        }
    }
}
