use std::{path::Path, os::unix::prelude::OsStrExt};

use libcdio_sys::{cdio_open, CdIo_t, driver_id_t_DRIVER_UNKNOWN, cdio_destroy};

pub struct CD {
    cdio: *mut CdIo_t
}

impl CD {
    pub fn open(path: impl AsRef<Path>) -> CD {
        let path = path.as_ref();
        let cdio = unsafe {
            cdio_open(path.as_os_str().as_bytes().as_ptr() as *const i8, driver_id_t_DRIVER_UNKNOWN)
        };
        
        Self { cdio }
    }
}

impl Drop for CD {
    fn drop(&mut self) {
        unsafe { cdio_destroy(self.cdio) }
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
