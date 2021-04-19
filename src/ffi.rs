#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_char;
use std::os::raw::c_uint;
use std::os::raw::c_void;

#[cfg(target_os = "windows")]
pub type nfdnchar_t = u16; // Windows

#[cfg(not(target_os = "windows"))]
pub type nfdnchar_t = c_char; // non-Windows

pub type nfdfiltersize_t = c_uint;

pub type nfdpathset_t = c_void;

#[repr(C)]
#[allow(dead_code)]
pub enum nfdresult_t {
    NFD_ERROR,
    NFD_OKAY,
    NFD_CANCEL,
}

#[repr(C)]
#[allow(dead_code)]
pub struct nfdnfilteritem_t {
    pub name: *mut nfdnchar_t, // note: the C API actually has *const nfdnchar_t, but we use *mut to avoid const casts here
    pub spec: *mut nfdnchar_t,
}

// We are using ptr==NULL to represent the lack of an enum (i.e. we already iterated to the end, or acquiring the enum failed).
// For Linux, it automatically becomes NULL when we reach the end, but we don't need to free the enum in Linux so it is okay.
#[repr(C)]
#[allow(dead_code)]
pub struct nfdpathsetenum_t {
    pub ptr: *mut c_void,
}

extern "C" {
    pub fn NFD_Init() -> nfdresult_t;
    pub fn NFD_Quit();
    pub fn NFD_GetError() -> *const c_char;
    pub fn NFD_FreePathN(filePath: *mut nfdnchar_t);
    pub fn NFD_OpenDialogN(
        outPath: *mut *mut nfdnchar_t,
        filterList: *const nfdnfilteritem_t,
        filterCount: nfdfiltersize_t,
        defaultPath: *const nfdnchar_t,
    ) -> nfdresult_t;
    pub fn NFD_OpenDialogMultipleN(
        outPaths: *mut *mut nfdpathset_t,
        filterList: *const nfdnfilteritem_t,
        filterCount: nfdfiltersize_t,
        defaultPath: *const nfdnchar_t,
    ) -> nfdresult_t;
    pub fn NFD_SaveDialogN(
        outPath: *mut *mut nfdnchar_t,
        filterList: *const nfdnfilteritem_t,
        filterCount: nfdfiltersize_t,
        defaultPath: *const nfdnchar_t,
        defaultName: *const nfdnchar_t,
    ) -> nfdresult_t;
    pub fn NFD_PickFolderN(
        outPath: *mut *mut nfdnchar_t,
        defaultPath: *const nfdnchar_t,
    ) -> nfdresult_t;
    pub fn NFD_PathSet_Free(pathSet: *mut nfdpathset_t);
    pub fn NFD_PathSet_GetEnum(
        pathSet: *mut nfdpathset_t,
        outEnumerator: *mut nfdpathsetenum_t,
    ) -> nfdresult_t;

    #[cfg(not(target_os = "linux"))]
    pub fn NFD_PathSet_FreeEnum(enumerator: *mut nfdpathsetenum_t);

    pub fn NFD_PathSet_EnumNextN(
        enumerator: *mut nfdpathsetenum_t,
        outPath: *mut *mut nfdnchar_t,
    ) -> nfdresult_t;
    #[cfg_attr(not(target_os = "linux"), link_name = "NFD_FreePathN")]
    pub fn NFD_PathSet_FreePathN(filePath: *mut nfdnchar_t);
}
