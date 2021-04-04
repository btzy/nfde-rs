use std::ffi::CStr;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

mod ffi;
mod path;

pub struct Nfd {}

pub use path::NfdPathBuf;
pub struct NfdPathSetBuf {}

pub type Error = &'static str;
pub type InitResult = Result<Nfd, Error>;
pub enum DialogResult<T> {
    Ok(T),
    Cancel,
    Err(Error),
}
pub type SingleFileResult = DialogResult<NfdPathBuf>;
pub type MultiFileResult = DialogResult<NfdPathSetBuf>;

pub trait SingleFileDialogBuilder {
    fn show(&self) -> SingleFileResult;
}

pub trait DefaultPathDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: &P) -> Result<&'a mut Self, Error>;
}

pub trait FilterableDialogBuilder {
    fn add_filter<'a>(&'a mut self, name: &str, spec: &str) -> Result<&'a mut Self, Error>;
    fn add_filters<'a, 'b, 'c, I: Iterator<Item = (&'b str, &'c str)>>(
        &'a mut self,
        filters: I,
    ) -> Result<&'a mut Self, Error>;
}

pub struct OpenFileDialogBuilder {
    filters: Vec<ffi::nfdnfilteritem_t>,
    default_path: Option<path::NfdCString>,
}
pub struct SaveFileDialogBuilder {
    filters: Vec<ffi::nfdnfilteritem_t>,
    default_path: Option<path::NfdCString>,
    default_name: Option<path::NfdCString>,
}
pub struct PickFolderDialogBuilder {
    default_path: Option<path::NfdCString>,
}

impl Nfd {
    pub fn new() -> InitResult {
        let res = unsafe { ffi::NFD_Init() };
        wrap_init_result(res)
    }
    pub fn open_file(&self) -> OpenFileDialogBuilder {
        OpenFileDialogBuilder {
            filters: Vec::new(),
            default_path: None,
        }
    }
    pub fn save_file(&self) -> SaveFileDialogBuilder {
        SaveFileDialogBuilder {
            filters: Vec::new(),
            default_path: None,
            default_name: None,
        }
    }
    pub fn pick_folder(&self) -> PickFolderDialogBuilder {
        PickFolderDialogBuilder { default_path: None }
    }
}

impl Drop for Nfd {
    fn drop(&mut self) {
        unsafe {
            ffi::NFD_Quit();
        }
    }
}

impl DefaultPathDialogBuilder for OpenFileDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: &P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}
impl DefaultPathDialogBuilder for SaveFileDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: &P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}
impl DefaultPathDialogBuilder for PickFolderDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: &P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}

impl FilterableDialogBuilder for OpenFileDialogBuilder {
    fn add_filter<'a>(&'a mut self, name: &str, spec: &str) -> Result<&'a mut Self, Error> {
        self.filters.push(make_filter(name, spec)?);
        Ok(self)
    }
    fn add_filters<'a, 'b, 'c, I: Iterator<Item = (&'b str, &'c str)>>(
        &'a mut self,
        filters: I,
    ) -> Result<&'a mut Self, Error> {
        self.filters.append(&mut make_filters(filters)?);
        Ok(self)
    }
}
impl FilterableDialogBuilder for SaveFileDialogBuilder {
    fn add_filter<'a>(&'a mut self, name: &str, spec: &str) -> Result<&'a mut Self, Error> {
        self.filters.push(make_filter(name, spec)?);
        Ok(self)
    }
    fn add_filters<'a, 'b, 'c, I: Iterator<Item = (&'b str, &'c str)>>(
        &'a mut self,
        filters: I,
    ) -> Result<&'a mut Self, Error> {
        self.filters.append(&mut make_filters(filters)?);
        Ok(self)
    }
}

impl SaveFileDialogBuilder {
    pub fn default_name<'a, P: AsRef<Path>>(&'a mut self, name: &P) -> Result<&'a mut Self, Error> {
        self.default_name = Some(path::unwrap_path(name.as_ref())?);
        Ok(self)
    }
}

impl SingleFileDialogBuilder for OpenFileDialogBuilder {
    fn show(&self) -> SingleFileResult {
        let mut out: *mut ffi::nfdnchar_t = std::ptr::null_mut();
        let filters = self.filters.as_ptr();
        let filters_len = self.filters.len() as ffi::nfdfiltersize_t;
        let default_path = self
            .default_path
            .as_deref()
            .map_or_else(std::ptr::null, |p| p.as_ptr());
        let res = unsafe { ffi::NFD_OpenDialogN(&mut out, filters, filters_len, default_path) };
        wrap_single_file_result(res, out)
    }
}
impl SingleFileDialogBuilder for SaveFileDialogBuilder {
    fn show(&self) -> SingleFileResult {
        let mut out: *mut ffi::nfdnchar_t = std::ptr::null_mut();
        let filters = self.filters.as_ptr();
        let filters_len = self.filters.len() as ffi::nfdfiltersize_t;
        let default_path = self
            .default_path
            .as_deref()
            .map_or_else(std::ptr::null, |p| p.as_ptr());
        let default_name = self
            .default_name
            .as_deref()
            .map_or_else(std::ptr::null, |n| n.as_ptr());
        let res = unsafe {
            ffi::NFD_SaveDialogN(&mut out, filters, filters_len, default_path, default_name)
        };
        wrap_single_file_result(res, out)
    }
}
impl SingleFileDialogBuilder for PickFolderDialogBuilder {
    fn show(&self) -> SingleFileResult {
        let mut out: *mut ffi::nfdnchar_t = std::ptr::null_mut();
        let default_path = self
            .default_path
            .as_deref()
            .map_or_else(std::ptr::null, |p| p.as_ptr());
        let res = unsafe { ffi::NFD_PickFolderN(&mut out, default_path) };
        wrap_single_file_result(res, out)
    }
}

impl Drop for OpenFileDialogBuilder {
    fn drop(&mut self) {
        destroy_filters(&mut self.filters);
    }
}
impl Drop for SaveFileDialogBuilder {
    fn drop(&mut self) {
        destroy_filters(&mut self.filters);
    }
}

fn make_filters<'b, 'c, I: Iterator<Item = (&'b str, &'c str)>>(
    filters: I,
) -> Result<Vec<ffi::nfdnfilteritem_t>, Error> {
    filters
        .map(|(name, spec)| make_filter(name, spec))
        .collect::<Result<_, Error>>()
}
fn make_filter(name: &str, spec: &str) -> Result<ffi::nfdnfilteritem_t, Error> {
    if spec.is_empty() {
        return Err("Filter specification is empty");
    }
    Ok(ffi::nfdnfilteritem_t {
        name: path::str_to_native(name)?.into_raw(),
        spec: path::str_to_native(spec)?.into_raw(),
    })
}
fn destroy_filters(filters: &mut Vec<ffi::nfdnfilteritem_t>) {
    filters
        .drain(..)
        .rev()
        .for_each(|filter| destroy_filter(filter));
}
fn destroy_filter(filter: ffi::nfdnfilteritem_t) {
    unsafe { path::NfdCString::from_raw(filter.spec as *mut ffi::nfdnchar_t) };
}

fn wrap_init_result(res: ffi::nfdresult_t) -> InitResult {
    match res {
        ffi::nfdresult_t::NFD_ERROR => Err(get_nfd_error()),
        _ => Ok(Nfd {}),
    }
}

fn wrap_single_file_result(res: ffi::nfdresult_t, out: *mut ffi::nfdnchar_t) -> SingleFileResult {
    match res {
        ffi::nfdresult_t::NFD_ERROR => SingleFileResult::Err(get_nfd_error()),
        ffi::nfdresult_t::NFD_OKAY => SingleFileResult::Ok(path::wrap_path(out)),
        ffi::nfdresult_t::NFD_CANCEL => SingleFileResult::Cancel,
    }
}

fn get_nfd_error() -> Error {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ffi::NFD_GetError()).to_bytes()) }
}
