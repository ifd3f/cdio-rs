use std::{path::Path, os::unix::prelude::{OsStrExt, PermissionsExt}, ptr, marker::PhantomData, fs::Permissions};

use libcdio_sys::{cdio_open, CdIo_t, driver_id_t_DRIVER_UNKNOWN, cdio_destroy, cdio_guess_cd_type, udf_open, udf_opendir, udf_s, udf_close, udf_get_root, udf_get_volume_id, udf_readdir, udf_dirent_s, udf_is_dir, udf_dirent_free, udf_get_link_count, udf_get_file_length, udf_get_posix_filemode};

/// An opened UDF filesystem.
/// 
/// The underlying file pointer is freed on drop.
pub struct UdfFs {
    p_udf: *mut udf_s
}

pub struct UdfDirs<'a> {
    dirent: *mut udf_dirent_s,
    udf: &'a UdfFs
}

pub enum UdfDirType {
    File,
    Directory
}

impl UdfFs {
    pub fn open(path: impl AsRef<Path>) -> UdfFs {
        let path = path.as_ref();
        let p_udf = unsafe {
            let path = path.as_os_str().as_bytes().as_ptr() as *const i8;
            udf_open(path)
        };
        if p_udf == ptr::null_mut() {
            todo!()
        }
        
        Self { p_udf }
    }

    pub fn root<'a>(&'a self) -> UdfDirs<'a> {
        unsafe {
            let dirent = udf_get_root(self.p_udf, 1, 0) ;
            if dirent == ptr::null_mut() {
                todo!()
            }
            UdfDirs::new(dirent, self)
        }
    }
}

impl<'a> UdfDirs<'a> {
    fn new(dirent: *mut udf_dirent_s, udf: &'a UdfFs) -> Self { 
        Self { dirent, udf }
    }

    pub fn file_type(&self) -> UdfDirType {
        unsafe {
            if udf_is_dir(self.dirent) == 0 {
                UdfDirType::File
            } else {
                UdfDirType::Directory
            }
        }
    }

    pub fn link_count(&self) -> u16 {
        unsafe {
            match udf_get_link_count(self.dirent) {
                0 => todo!(),
                l => l,
            }
        }
    }

    pub fn file_length(&self) -> u32 {
        unsafe {
            match udf_get_file_length(self.dirent) {
                2147483647 => todo!(),
                l => l as u32,
            }
        }
    }

    pub fn posix_filemode(&self) -> Permissions {
        unsafe {
            let mode = udf_get_posix_filemode(self.dirent); 
            Permissions::from_mode(mode)
        }
    }
}

impl<'a> Iterator for UdfDirs<'a> {
    type Item = UdfDirs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let dirent = udf_readdir(self.dirent);
            if dirent == ptr::null_mut() {
                None
            } else {
                Some(UdfDirs { dirent, udf: self.udf })
            }
        }
    }
}

impl Drop for UdfFs {
    fn drop(&mut self) {
        unsafe { udf_close(self.p_udf); }
    }
}

impl<'a> Drop for UdfDirs<'a> {
    fn drop(&mut self) {
        unsafe { udf_dirent_free(self.dirent); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
