use std::{os::unix::prelude::OsStrExt, path::Path, ptr};

use libcdio_sys::{udf_close, udf_get_root, udf_open, udf_s};

use super::dirent::Dirent;

/// An opened UDF filesystem.
///
/// The underlying file pointer is freed on drop.
pub struct UdfFs {
    p_udf: *mut udf_s,
}

impl UdfFs {
    pub fn open(path: impl AsRef<Path>) -> UdfFs {
        let path = path.as_ref();
        let p_udf = unsafe { udf_open(path.as_os_str().as_bytes().as_ptr() as *const i8) };
        if p_udf == ptr::null_mut() {
            todo!()
        }

        Self { p_udf }
    }

    fn root<'a>(&'a self) -> Option<Dirent<'a>> {
        let dirent = unsafe { udf_get_root(self.p_udf, 1, 0) };
        Dirent::new(dirent)
    }
}

impl Drop for UdfFs {
    fn drop(&mut self) {
        unsafe {
            udf_close(self.p_udf);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::UdfFs;

    const EXPECTED_NAME: &str = "FéжΘvrier";

    #[test]
    fn open_iso() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/udf102.iso");
        let fs = UdfFs::open(&d);
    }
}
