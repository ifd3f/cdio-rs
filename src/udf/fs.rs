use std::{
    fmt::Debug,
    io::Read,
    os::unix::prelude::OsStrExt,
    path::Path,
    ptr,
    sync::{Arc, RwLock},
};

use libcdio_sys::{udf_close, udf_fopen, udf_get_root, udf_open, udf_s};
use vfs::{error::VfsErrorKind, FileSystem, VfsError, VfsResult};

use super::dirent::{Dirent, UDF_BLOCKSIZE};

/// An opened UDF filesystem.
///
/// The underlying file pointer is freed on drop.
pub(crate) struct Udf {
    p_udf: *mut udf_s,
}

unsafe impl Send for Udf {}
unsafe impl Sync for Udf {}

impl Udf {
    pub fn open(path: impl AsRef<Path>) -> Udf {
        let path = path.as_ref();
        let p_udf = unsafe { udf_open(path.as_os_str().as_bytes().as_ptr() as *const i8) };
        if p_udf == ptr::null_mut() {
            todo!()
        }

        Self { p_udf }
    }

    fn root<'udf>(&'udf self) -> Option<Dirent<'udf>> {
        let dirent = unsafe { udf_get_root(self.p_udf, 1, 0) };
        Dirent::new(dirent)
    }

    fn fopen<'udf>(&'udf self, path: &str) -> Option<Dirent<'udf>> {
        self.root()?.fopen(path)
    }
}

impl Drop for Udf {
    fn drop(&mut self) {
        unsafe {
            udf_close(self.p_udf);
        }
    }
}

pub struct UdfFs {
    inner: Udf,
}

impl UdfFs {
    pub fn open(path: impl AsRef<Path>) -> UdfFs {
        let fs = Udf::open(path);
        Self { inner: fs.into() }
    }
}

impl FileSystem for UdfFs {
    fn read_dir(&self, path: &str) -> vfs::VfsResult<Box<dyn Iterator<Item = String> + Send>> {
        if let Some(mut dirent) = self.inner.fopen(path) {
            let mut children = vec![];
            loop {
                children.push(dirent.file_name().unwrap().to_string_lossy().to_string());
                match dirent.next_sibling() {
                    Some(s) => dirent = s,
                    None => break Ok(Box::new(children.into_iter())),
                }
            }
        } else {
            Err(VfsError::from(VfsErrorKind::FileNotFound))
        }
    }

    fn open_file(&self, path: &str) -> vfs::VfsResult<Box<dyn vfs::SeekAndRead + Send>> {
        todo!()
    }

    fn metadata(&self, path: &str) -> vfs::VfsResult<vfs::VfsMetadata> {
        todo!()
    }

    fn exists(&self, path: &str) -> vfs::VfsResult<bool> {
        Ok(self.inner.fopen(path).is_some())
    }

    fn create_dir(&self, _path: &str) -> vfs::VfsResult<()> {
        Err(VfsError::from(VfsErrorKind::NotSupported))
    }

    fn create_file(&self, _path: &str) -> vfs::VfsResult<Box<dyn std::io::Write + Send>> {
        Err(VfsError::from(VfsErrorKind::NotSupported))
    }

    fn append_file(&self, _path: &str) -> vfs::VfsResult<Box<dyn std::io::Write + Send>> {
        Err(VfsError::from(VfsErrorKind::NotSupported))
    }

    fn remove_file(&self, _path: &str) -> vfs::VfsResult<()> {
        Err(VfsError::from(VfsErrorKind::NotSupported))
    }

    fn remove_dir(&self, _path: &str) -> vfs::VfsResult<()> {
        Err(VfsError::from(VfsErrorKind::NotSupported))
    }
}

impl Debug for UdfFs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UdfFs").finish()
    }
}

pub struct UdfFile<'udf> {
    dirent: Dirent<'udf>,
    idx: usize,
    buf: Vec<u8>,
}

impl<'udf> UdfFile<'udf> {
    pub fn new(dirent: Dirent<'udf>) -> Self {
        Self {
            dirent,
            idx: 0,
            buf: vec![],
        }
    }
}

impl<'udf> Read for UdfFile<'udf> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end = self.idx + buf.len();
        let start_block = self.idx / UDF_BLOCKSIZE;
        let end_block = end / UDF_BLOCKSIZE;
        self.dirent.read_block(buf, block_count)
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
