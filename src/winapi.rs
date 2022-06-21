pub const CP_ACP: DWORD = 0;
pub const CP_OEMCP: DWORD = 1;
#[allow(dead_code)] // in tests
pub const CP_UTF8: DWORD = 65001;

type CHAR = i8;
pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;

pub type WCHAR = u16;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;

pub type BOOL = i32;
pub type LPBOOL = *mut BOOL;

pub type DWORD = u32;

pub type UINT = u32;

#[allow(non_snake_case)]
extern "system" {
    pub fn MultiByteToWideChar(
        CodePage: UINT,
        dwFlags: DWORD,
        lpMultiByteStr: LPCSTR,
        cbMultiByte: i32,
        lpWideCharStr: LPWSTR,
        cchWideChar: i32,
    ) -> i32;
    pub fn WideCharToMultiByte(
        CodePage: UINT,
        dwFlags: DWORD,
        lpWideCharStr: LPCWSTR,
        cchWideChar: i32,
        lpMultiByteStr: LPSTR,
        cbMultiByte: i32,
        lpDefaultChar: LPCSTR,
        lpUsedDefaultChar: LPBOOL,
    ) -> i32;
}
