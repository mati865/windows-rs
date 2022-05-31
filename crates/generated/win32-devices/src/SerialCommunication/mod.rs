pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
#[inline]
pub unsafe fn ComDBClaimNextFreePort<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, comnumber: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClaimNextFreePort(hcomdb: HCOMDB, comnumber: *mut u32) -> i32;
        }
        ::core::mem::transmute(ComDBClaimNextFreePort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBClaimPort<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hcomdb: Param0, comnumber: u32, forceclaim: Param2, forced: *mut ::win32_foundation::BOOL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClaimPort(hcomdb: HCOMDB, comnumber: u32, forceclaim: ::win32_foundation::BOOL, forced: *mut ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(ComDBClaimPort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber), forceclaim.into_param().abi(), ::core::mem::transmute(forced)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBClose<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>>(hcomdb: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBClose(hcomdb: HCOMDB) -> i32;
        }
        ::core::mem::transmute(ComDBClose(hcomdb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBGetCurrentPortUsage<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBGetCurrentPortUsage(hcomdb: HCOMDB, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32;
        }
        ::core::mem::transmute(ComDBGetCurrentPortUsage(hcomdb.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(reporttype), ::core::mem::transmute(maxportsreported)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBOpen(phcomdb: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBOpen(phcomdb: *mut isize) -> i32;
        }
        ::core::mem::transmute(ComDBOpen(::core::mem::transmute(phcomdb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBReleasePort<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, comnumber: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBReleasePort(hcomdb: HCOMDB, comnumber: u32) -> i32;
        }
        ::core::mem::transmute(ComDBReleasePort(hcomdb.into_param().abi(), ::core::mem::transmute(comnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ComDBResizeDatabase<'a, Param0: ::windows_core::IntoParam<'a, HCOMDB>>(hcomdb: Param0, newsize: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ComDBResizeDatabase(hcomdb: HCOMDB, newsize: u32) -> i32;
        }
        ::core::mem::transmute(ComDBResizeDatabase(hcomdb.into_param().abi(), ::core::mem::transmute(newsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCOMDB(pub isize);
impl HCOMDB {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCOMDB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCOMDB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCOMDB {}
impl ::core::fmt::Debug for HCOMDB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCOMDB").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HCOMDB {
    type Abi = Self;
}
