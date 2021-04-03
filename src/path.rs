use crate::ffi;
use crate::Error;
use std::path::Path;
use std::result::Result;

const C_STRING_INTERIOR_NULL_ERROR_MESSAGE: &'static str =
    "Cannot convert path with interior null values";

#[cfg(target_os = "windows")]
mod pathutil {
    use crate::ffi;
    //use std::ffi::OsString;
    //use std::os::windows::ffi::OsStrExt;
    //use std::os::windows::ffi::OsStringExt;
    use std::path::Path;
    use std::path::PathBuf;
    use widestring::U16CStr;
    use widestring::U16CString;

    pub type NfdCStr = U16CStr;
    pub type NfdCString = U16CString;
    pub type NfdPathBuf = PathBuf;

    pub fn wrap_path(path: *mut ffi::nfdnchar_t) -> NfdPathBuf {
        //let ret = OsString::from_wide(unsafe { NfdCStr::from_ptr_str(path) }.as_slice());
        let ret = unsafe { NfdCStr::from_ptr_str(path) }.to_os_string();
        unsafe { ffi::NFD_FreePathN(path) };
        ret.into()
    }

    pub fn unwrap_path(path: &Path) -> Result<NfdCString, super::Error> {
        NfdCString::from_os_str(path.as_os_str())
            .map_err(|_| super::C_STRING_INTERIOR_NULL_ERROR_MESSAGE)
    }

    pub fn str_to_native(s: &str) -> Result<NfdCString, super::Error> {
        NfdCString::from_str(s).map_err(|_| super::C_STRING_INTERIOR_NULL_ERROR_MESSAGE)
    }
}

#[cfg(not(target_os = "windows"))]
mod pathutil {
    use crate::ffi;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ffi::OsString;
    use std::os::unix::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::path::Path;
    use std::path::PathBuf;

    pub type NfdCStr = CStr;
    pub type NfdCString = CString;
    pub struct NfdPathBuf {
        path: *mut ffi::nfdnchar_t,
    }
    impl Drop for NfdPathBuf {
        fn drop(&mut self) {
            unsafe { ffi::NFD_FreePathN(path) };
        }
    }
    impl NfdPathBuf {
        pub fn as_path(&self) -> &Path {
            OsString::from_bytes(unsafe { NfdCStr::from_ptr(self.path) }.to_bytes())
        }
    }

    pub fn wrap_path(path: *mut ffi::nfdnchar_t) -> NfdPathBuf {
        NfdPathBuf { path }
    }

    pub fn unwrap_path(path: &Path) -> Result<NfdCString, super::Error> {
        NfdCString::new(path.as_os_str().as_bytes())
            .map_err(|_| super::C_STRING_INTERIOR_NULL_ERROR_MESSAGE)
    }

    pub fn str_to_native(s: &str) -> Result<NfdCString, super::Error> {
        NfdCString::new(s).map_err(|_| super::C_STRING_INTERIOR_NULL_ERROR_MESSAGE)
    }
}

pub use pathutil::NfdCString;
pub use pathutil::NfdPathBuf;

pub fn wrap_path(path: *mut ffi::nfdnchar_t) -> NfdPathBuf {
    pathutil::wrap_path(path)
}

pub fn unwrap_path(path: &Path) -> Result<NfdCString, Error> {
    pathutil::unwrap_path(path)
}

pub fn str_to_native(s: &str) -> Result<NfdCString, Error> {
    pathutil::str_to_native(s)
}
