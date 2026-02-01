use std::ffi::CStr;
use std::iter::IntoIterator;
use std::path::Path;
use std::result::Result;

mod ffi;
mod path;

pub struct Nfd {}

pub use path::NfdPathBuf;
pub struct NfdPathSetBuf {
    ptr: *mut ffi::nfdpathset_t,
}
pub use path::NfdPathSetPathBuf;

pub type Error = &'static str;
pub type InitResult = Result<Nfd, Error>;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DialogResult<T> {
    Ok(T),
    Cancel,
    Err(Error),
}
pub type SingleFileResult = DialogResult<NfdPathBuf>;
pub type MultipleFileResult = DialogResult<NfdPathSetBuf>;

pub trait SingleFileDialogBuilder {
    fn show(&self) -> SingleFileResult;
}
pub trait MultipleFileDialogBuilder {
    fn show(&self) -> MultipleFileResult;
}

pub trait DefaultPathDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<&'a mut Self, Error>;
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
pub struct OpenFileMultipleDialogBuilder {
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
unsafe impl Send for OpenFileDialogBuilder {}
unsafe impl Send for OpenFileMultipleDialogBuilder {}
unsafe impl Send for SaveFileDialogBuilder {}
unsafe impl Send for PickFolderDialogBuilder {}

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
    pub fn open_file_multiple(&self) -> OpenFileMultipleDialogBuilder {
        OpenFileMultipleDialogBuilder {
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
        unsafe { ffi::NFD_Quit() };
    }
}

impl<T> DialogResult<T> {
    pub fn into_result(self) -> Result<Option<T>, Error> {
        match self {
            DialogResult::Ok(val) => Ok(Some(val)),
            DialogResult::Cancel => Ok(None),
            DialogResult::Err(error) => Err(error),
        }
    }
    pub fn as_result(&self) -> Result<Option<&T>, Error> {
        match self {
            DialogResult::Ok(val) => Ok(Some(val)),
            DialogResult::Cancel => Ok(None),
            DialogResult::Err(error) => Err(error),
        }
    }
}

impl DefaultPathDialogBuilder for OpenFileDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}
impl DefaultPathDialogBuilder for OpenFileMultipleDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}
impl DefaultPathDialogBuilder for SaveFileDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<&'a mut Self, Error> {
        self.default_path = Some(path::unwrap_path(path.as_ref())?);
        Ok(self)
    }
}
impl DefaultPathDialogBuilder for PickFolderDialogBuilder {
    fn default_path<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<&'a mut Self, Error> {
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
impl FilterableDialogBuilder for OpenFileMultipleDialogBuilder {
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

impl MultipleFileDialogBuilder for OpenFileMultipleDialogBuilder {
    fn show(&self) -> MultipleFileResult {
        let mut out: *mut ffi::nfdpathset_t = std::ptr::null_mut();
        let filters = self.filters.as_ptr();
        let filters_len = self.filters.len() as ffi::nfdfiltersize_t;
        let default_path = self
            .default_path
            .as_deref()
            .map_or_else(std::ptr::null, |p| p.as_ptr());
        let res =
            unsafe { ffi::NFD_OpenDialogMultipleN(&mut out, filters, filters_len, default_path) };
        wrap_multiple_file_result(res, out)
    }
}

impl Drop for OpenFileDialogBuilder {
    fn drop(&mut self) {
        destroy_filters(&mut self.filters);
    }
}
impl Drop for OpenFileMultipleDialogBuilder {
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
    let _ = unsafe { path::NfdCString::from_raw(filter.spec) };
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

fn wrap_multiple_file_result(
    res: ffi::nfdresult_t,
    out: *mut ffi::nfdpathset_t,
) -> MultipleFileResult {
    match res {
        ffi::nfdresult_t::NFD_ERROR => MultipleFileResult::Err(get_nfd_error()),
        ffi::nfdresult_t::NFD_OKAY => MultipleFileResult::Ok(NfdPathSetBuf::new(out)),
        ffi::nfdresult_t::NFD_CANCEL => MultipleFileResult::Cancel,
    }
}

impl NfdPathSetBuf {
    fn new(ptr: *mut ffi::nfdpathset_t) -> Self {
        Self { ptr }
    }
    pub fn iter(&self) -> path_set::Iter<'_> {
        self.into_iter()
    }
}
impl Drop for NfdPathSetBuf {
    fn drop(&mut self) {
        unsafe { ffi::NFD_PathSet_Free(self.ptr) };
    }
}

impl IntoIterator for NfdPathSetBuf {
    type Item = <path_set::IntoIter as Iterator>::Item;
    type IntoIter = path_set::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        path_set::IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a NfdPathSetBuf {
    type Item = <path_set::Iter<'a> as Iterator>::Item;
    type IntoIter = path_set::Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        path_set::Iter::new(self)
    }
}

pub mod path_set {
    use super::ffi;
    use super::get_nfd_error;
    use super::path;
    use super::Error;
    use super::NfdPathSetBuf;
    use super::NfdPathSetPathBuf;
    use std::marker::PhantomData;
    pub struct Iter<'a> {
        pub(super) enumerator: ffi::nfdpathsetenum_t,
        phantom: PhantomData<&'a NfdPathSetBuf>,
    }
    pub struct IntoIter {
        pub(super) _buf: NfdPathSetBuf, // we just keep it here so that we can drop it later
        pub(super) iter: Iter<'static>, // we don't need the lifetime enforcement here, so we set it to 'static
    }

    impl<'a> Iter<'a> {
        pub(super) fn new(pathset: &'a NfdPathSetBuf) -> Self {
            let mut enumerator = ffi::nfdpathsetenum_t {
                ptr: std::ptr::null_mut(),
            };
            let res = unsafe { ffi::NFD_PathSet_GetEnum(pathset.ptr, &mut enumerator) };
            Self {
                enumerator: match res {
                    ffi::nfdresult_t::NFD_ERROR => ffi::nfdpathsetenum_t {
                        ptr: std::ptr::null_mut(),
                    },
                    ffi::nfdresult_t::NFD_OKAY => enumerator,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                },
                phantom: PhantomData,
            }
        }
    }

    impl IntoIter {
        pub(super) fn new(pathset: NfdPathSetBuf) -> Self {
            let iter = Iter::new(unsafe { std::mem::transmute(&pathset) }); // need to extend the lifetime
            Self {
                _buf: pathset,
                iter,
            }
        }
    }

    impl<'a> Drop for Iter<'a> {
        fn drop(&mut self) {
            #[cfg(not(target_os = "linux"))]
            if !self.enumerator.ptr.is_null() {
                unsafe { ffi::NFD_PathSet_FreeEnum(&mut self.enumerator) };
            }
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = Result<NfdPathSetPathBuf, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.enumerator.ptr.is_null() {
                None
            } else {
                let mut out: *mut ffi::nfdnchar_t = std::ptr::null_mut();
                let res = unsafe { ffi::NFD_PathSet_EnumNextN(&mut self.enumerator, &mut out) };
                wrap_pathset_file_result(&mut self.enumerator, res, out)
            }
        }
    }

    impl Iterator for IntoIter {
        type Item = Result<NfdPathSetPathBuf, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    }

    impl<'a> std::iter::FusedIterator for Iter<'a> {}
    impl std::iter::FusedIterator for IntoIter {}

    unsafe impl<'a> Send for Iter<'a> {}
    unsafe impl Send for IntoIter {}

    fn wrap_pathset_file_result(
        enumerator: &mut ffi::nfdpathsetenum_t,
        res: ffi::nfdresult_t,
        out: *mut ffi::nfdnchar_t,
    ) -> Option<Result<NfdPathSetPathBuf, Error>> {
        // note: we cannot use a normal NfdPathBuf because the freeing mechanism may be different
        match res {
            ffi::nfdresult_t::NFD_ERROR => Some(Err(get_nfd_error())),
            ffi::nfdresult_t::NFD_OKAY => {
                if !out.is_null() {
                    Some(Ok(path::wrap_pathset_path(out)))
                } else {
                    free_enum(enumerator);

                    None
                }
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn free_enum(enumerator: &mut ffi::nfdpathsetenum_t) {
        unsafe { ffi::NFD_PathSet_FreeEnum(enumerator) };
        enumerator.ptr = std::ptr::null_mut();
    }

    #[cfg(target_os = "linux")]
    fn free_enum(_enumerator: &mut ffi::nfdpathsetenum_t) {
        // do nothing, because on Linux, the PathSet enumerator is the actual PathSet
    }
}

fn get_nfd_error() -> Error {
    unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(ffi::NFD_GetError()).to_bytes()) }
}
