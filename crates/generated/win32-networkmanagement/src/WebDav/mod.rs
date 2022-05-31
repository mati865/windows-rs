#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHNEXTSTEP(pub i32);
pub const DefaultBehavior: AUTHNEXTSTEP = AUTHNEXTSTEP(0i32);
pub const RetryRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(1i32);
pub const CancelRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(2i32);
impl ::core::marker::Copy for AUTHNEXTSTEP {}
impl ::core::clone::Clone for AUTHNEXTSTEP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHNEXTSTEP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHNEXTSTEP {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHNEXTSTEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHNEXTSTEP").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_BLOB").field("pBuffer", &self.pBuffer).field("ulSize", &self.ulSize).field("ulType", &self.ulType).finish()
    }
}
unsafe impl ::windows_core::Abi for DAV_CALLBACK_AUTH_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_AUTH_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: ::windows_core::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: ::windows_core::PWSTR,
    pub ulPasswordLength: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_UNP {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_UNP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_UNP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_UNP").field("pszUserName", &self.pszUserName).field("ulUserNameLength", &self.ulUserNameLength).field("pszPassword", &self.pszPassword).field("ulPasswordLength", &self.ulPasswordLength).finish()
    }
}
unsafe impl ::windows_core::Abi for DAV_CALLBACK_AUTH_UNP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_AUTH_UNP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: ::win32_foundation::BOOL,
    pub bSave: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for DAV_CALLBACK_CRED {}
impl ::core::clone::Clone for DAV_CALLBACK_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DAV_CALLBACK_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_CRED").field("AuthBlob", &self.AuthBlob).field("UNPBlob", &self.UNPBlob).field("bAuthBlobValid", &self.bAuthBlobValid).field("bSave", &self.bSave).finish()
    }
}
unsafe impl ::windows_core::Abi for DAV_CALLBACK_CRED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_CRED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_CRED>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_CRED {}
impl ::core::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn DavAddConnection<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(connectionhandle: *mut ::win32_foundation::HANDLE, remotename: Param1, username: Param2, password: Param3, clientcert: *const u8, certsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavAddConnection(connectionhandle: *mut ::win32_foundation::HANDLE, remotename: ::windows_core::PCWSTR, username: ::windows_core::PCWSTR, password: ::windows_core::PCWSTR, clientcert: *const u8, certsize: u32) -> u32;
        }
        ::core::mem::transmute(DavAddConnection(::core::mem::transmute(connectionhandle), remotename.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(clientcert), ::core::mem::transmute(certsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavCancelConnectionsToServer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(lpname: Param0, fforce: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavCancelConnectionsToServer(lpname: ::windows_core::PCWSTR, fforce: ::win32_foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DavCancelConnectionsToServer(lpname.into_param().abi(), fforce.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavDeleteConnection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(connectionhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavDeleteConnection(connectionhandle: ::win32_foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(DavDeleteConnection(connectionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavFlushFile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hfile: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavFlushFile(hfile: ::win32_foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(DavFlushFile(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavGetExtendedError<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hfile: Param0, exterror: *mut u32, exterrorstring: ::windows_core::PWSTR, cchsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetExtendedError(hfile: ::win32_foundation::HANDLE, exterror: *mut u32, exterrorstring: ::windows_core::PWSTR, cchsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetExtendedError(hfile.into_param().abi(), ::core::mem::transmute(exterror), ::core::mem::transmute(exterrorstring), ::core::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavGetHTTPFromUNCPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(uncpath: Param0, url: ::windows_core::PWSTR, lpsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetHTTPFromUNCPath(uncpath: ::windows_core::PCWSTR, url: ::windows_core::PWSTR, lpsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetHTTPFromUNCPath(uncpath.into_param().abi(), ::core::mem::transmute(url), ::core::mem::transmute(lpsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavGetTheLockOwnerOfTheFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(filename: Param0, lockownername: ::windows_core::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetTheLockOwnerOfTheFile(filename: ::windows_core::PCWSTR, lockownername: ::windows_core::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetTheLockOwnerOfTheFile(filename.into_param().abi(), ::core::mem::transmute(lockownername), ::core::mem::transmute(lockownernamelengthinbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavGetUNCFromHTTPPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(url: Param0, uncpath: ::windows_core::PWSTR, lpsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetUNCFromHTTPPath(url: ::windows_core::PCWSTR, uncpath: ::windows_core::PWSTR, lpsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetUNCFromHTTPPath(url.into_param().abi(), ::core::mem::transmute(uncpath), ::core::mem::transmute(lpsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavInvalidateCache<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(urlname: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavInvalidateCache(urlname: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(DavInvalidateCache(urlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavRegisterAuthCallback(callback: ::windows_core::RawPtr, version: u32) -> u32;
        }
        ::core::mem::transmute(DavRegisterAuthCallback(::core::mem::transmute(callback), ::core::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavUnregisterAuthCallback(hcallback: u32);
        }
        DavUnregisterAuthCallback(::core::mem::transmute(hcallback))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PFNDAVAUTHCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpwzservername: ::windows_core::PCWSTR, lpwzremotename: ::windows_core::PCWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
pub type PFNDAVAUTHCALLBACK_FREECRED = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32>;
