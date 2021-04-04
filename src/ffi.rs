#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_char;
use std::os::raw::c_uint;

#[cfg(target_os = "windows")]
pub type nfdnchar_t = u16; // Windows

#[cfg(not(target_os = "windows"))]
pub type nfdnchar_t = c_char; // non-Windows

pub type nfdfiltersize_t = c_uint;

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
    pub name: *const nfdnchar_t,
    pub spec: *const nfdnchar_t,
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
}
