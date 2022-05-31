#[link(name = "windows")]
extern "system" {
    pub fn DavAddConnection(connectionhandle: *mut ::win32_foundation_sys::HANDLE, remotename: ::windows_core_sys::PCWSTR, username: ::windows_core_sys::PCWSTR, password: ::windows_core_sys::PCWSTR, clientcert: *const u8, certsize: u32) -> u32;
    pub fn DavCancelConnectionsToServer(lpname: ::windows_core_sys::PCWSTR, fforce: ::win32_foundation_sys::BOOL) -> u32;
    pub fn DavDeleteConnection(connectionhandle: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn DavFlushFile(hfile: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn DavGetExtendedError(hfile: ::win32_foundation_sys::HANDLE, exterror: *mut u32, exterrorstring: ::windows_core_sys::PWSTR, cchsize: *mut u32) -> u32;
    pub fn DavGetHTTPFromUNCPath(uncpath: ::windows_core_sys::PCWSTR, url: ::windows_core_sys::PWSTR, lpsize: *mut u32) -> u32;
    pub fn DavGetTheLockOwnerOfTheFile(filename: ::windows_core_sys::PCWSTR, lockownername: ::windows_core_sys::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
    pub fn DavGetUNCFromHTTPPath(url: ::windows_core_sys::PCWSTR, uncpath: ::windows_core_sys::PWSTR, lpsize: *mut u32) -> u32;
    pub fn DavInvalidateCache(urlname: ::windows_core_sys::PCWSTR) -> u32;
    pub fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32;
    pub fn DavUnregisterAuthCallback(hcallback: u32);
}
pub type AUTHNEXTSTEP = i32;
pub const DefaultBehavior: AUTHNEXTSTEP = 0i32;
pub const RetryRequest: AUTHNEXTSTEP = 1i32;
pub const CancelRequest: AUTHNEXTSTEP = 2i32;
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[repr(C)]
pub struct DAV_CALLBACK_AUTH_BLOB {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub ulSize: u32,
    pub ulType: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: ::windows_core_sys::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: ::windows_core_sys::PWSTR,
    pub ulPasswordLength: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_UNP {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_UNP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: ::win32_foundation_sys::BOOL,
    pub bSave: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for DAV_CALLBACK_CRED {}
impl ::core::clone::Clone for DAV_CALLBACK_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFNDAVAUTHCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpwzservername: ::windows_core_sys::PCWSTR, lpwzremotename: ::windows_core_sys::PCWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
pub type PFNDAVAUTHCALLBACK_FREECRED = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32>;
