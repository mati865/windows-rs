#[cfg(feature = "CallObj")]
pub mod CallObj;
#[cfg(feature = "ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Events")]
pub mod Events;
#[cfg(feature = "Marshal")]
pub mod Marshal;
#[cfg(feature = "StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "UI")]
pub mod UI;
#[cfg(feature = "Urlmon")]
pub mod Urlmon;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ADVF(pub i32);
pub const ADVF_NODATA: ADVF = ADVF(1i32);
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::core::marker::Copy for ADVF {}
impl ::core::clone::Clone for ADVF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ADVF {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADVF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVF").field(&self.0).finish()
    }
}
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APTTYPE(pub i32);
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::core::marker::Copy for APTTYPE {}
impl ::core::clone::Clone for APTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APTTYPEQUALIFIER(pub i32);
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
impl ::core::marker::Copy for APTTYPEQUALIFIER {}
impl ::core::clone::Clone for APTTYPEQUALIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPEQUALIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APTTYPEQUALIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPEQUALIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPEQUALIFIER").field(&self.0).finish()
    }
}
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[repr(C)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATEINFO").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHENTICATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHENTICATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
impl ::core::default::Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationType(pub i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::core::marker::Copy for ApplicationType {}
impl ::core::clone::Clone for ApplicationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ApplicationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct AsyncIAdviseSink(::windows_core::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows_core::Interface::vtable(self).Begin_OnDataChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnDataChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows_core::Interface::vtable(self).Begin_OnViewChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnViewChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).Begin_OnRename)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
    pub unsafe fn Finish_OnRename(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnRename)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnSave(&self) {
        (::windows_core::Interface::vtable(self).Begin_OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnSave(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnClose(&self) {
        (::windows_core::Interface::vtable(self).Begin_OnClose)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnClose(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnClose)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink> for ::windows_core::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink> for ::windows_core::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIAdviseSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink {}
impl ::core::fmt::Debug for AsyncIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_OnDataChange: usize,
    pub Finish_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub Finish_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr),
    pub Finish_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct AsyncIAdviseSink2(::windows_core::IUnknown);
impl AsyncIAdviseSink2 {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows_core::Interface::vtable(self).base__.Begin_OnDataChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows_core::Interface::vtable(self).base__.Finish_OnDataChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows_core::Interface::vtable(self).base__.Begin_OnViewChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows_core::Interface::vtable(self).base__.Finish_OnViewChange)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).base__.Begin_OnRename)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
    pub unsafe fn Finish_OnRename(&self) {
        (::windows_core::Interface::vtable(self).base__.Finish_OnRename)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnSave(&self) {
        (::windows_core::Interface::vtable(self).base__.Begin_OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnSave(&self) {
        (::windows_core::Interface::vtable(self).base__.Finish_OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnClose(&self) {
        (::windows_core::Interface::vtable(self).base__.Begin_OnClose)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnClose(&self) {
        (::windows_core::Interface::vtable(self).base__.Finish_OnClose)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).Begin_OnLinkSrcChange)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        (::windows_core::Interface::vtable(self).Finish_OnLinkSrcChange)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for ::windows_core::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for ::windows_core::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, AsyncIAdviseSink> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, AsyncIAdviseSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, AsyncIAdviseSink> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, AsyncIAdviseSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink2 {}
impl ::core::fmt::Debug for AsyncIAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base__: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct AsyncIMultiQI(::windows_core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(::windows_core::Interface::as_raw(self), pmqis.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pmqis))).ok()
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmqis)).ok()
    }
}
impl ::core::convert::From<AsyncIMultiQI> for ::windows_core::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIMultiQI> for ::windows_core::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIMultiQI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMultiQI {}
impl ::core::fmt::Debug for AsyncIMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct AsyncIPipeByte(::windows_core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Push)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeByte> for ::windows_core::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeByte> for ::windows_core::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIPipeByte {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeByte {}
impl ::core::fmt::Debug for AsyncIPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct AsyncIPipeDouble(::windows_core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[f64]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Push)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeDouble> for ::windows_core::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeDouble> for ::windows_core::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIPipeDouble {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeDouble {}
impl ::core::fmt::Debug for AsyncIPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct AsyncIPipeLong(::windows_core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_Push)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeLong> for ::windows_core::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeLong> for ::windows_core::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIPipeLong {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeLong {}
impl ::core::fmt::Debug for AsyncIPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct AsyncIUnknown(::windows_core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_QueryInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finish_QueryInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn Begin_AddRef(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_AddRef)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Finish_AddRef)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Begin_Release(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin_Release)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Finish_Release)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<AsyncIUnknown> for ::windows_core::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIUnknown> for ::windows_core::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsyncIUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIUnknown {}
impl ::core::fmt::Debug for AsyncIUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: ::windows_core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: ::windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: ::win32_security::SECURITY_ATTRIBUTES,
    pub iid: ::windows_core::GUID,
    pub pUnk: ::core::option::Option<::windows_core::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            szExtraInfo: self.szExtraInfo,
            stgmedData: self.stgmedData.clone(),
            grfBindInfoF: self.grfBindInfoF,
            dwBindVerb: self.dwBindVerb,
            szCustomVerb: self.szCustomVerb,
            cbstgmedData: self.cbstgmedData,
            dwOptions: self.dwOptions,
            dwOptionsFlags: self.dwOptionsFlags,
            dwCodePage: self.dwCodePage,
            securityAttributes: self.securityAttributes,
            iid: self.iid,
            pUnk: self.pUnk.clone(),
            dwReserved: self.dwReserved,
        }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows_core::Abi for BINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for BINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szExtraInfo == other.szExtraInfo && self.stgmedData == other.stgmedData && self.grfBindInfoF == other.grfBindInfoF && self.dwBindVerb == other.dwBindVerb && self.szCustomVerb == other.szCustomVerb && self.cbstgmedData == other.cbstgmedData && self.dwOptions == other.dwOptions && self.dwOptionsFlags == other.dwOptionsFlags && self.dwCodePage == other.dwCodePage && self.securityAttributes == other.securityAttributes && self.iid == other.iid && self.pUnk == other.pUnk && self.dwReserved == other.dwReserved
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for BINDINFO {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BINDINFOF(pub i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::core::marker::Copy for BINDINFOF {}
impl ::core::clone::Clone for BINDINFOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDINFOF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BINDINFOF {
    type Abi = Self;
}
impl ::core::fmt::Debug for BINDINFOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFOF").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::core::mem::ManuallyDrop<::core::option::Option<ITypeComp>>,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for BINDPTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for BINDPTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BINDPTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for BINDPTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BIND_FLAGS(pub i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::core::marker::Copy for BIND_FLAGS {}
impl ::core::clone::Clone for BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BIND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
unsafe impl ::windows_core::Abi for BIND_OPTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
impl ::core::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl ::core::marker::Copy for BIND_OPTS2 {}
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS2").field("__AnonymousBase_objidl_L9017_C36", &self.__AnonymousBase_objidl_L9017_C36).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
unsafe impl ::windows_core::Abi for BIND_OPTS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS2 {}
impl ::core::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: ::win32_foundation::HWND,
}
impl ::core::marker::Copy for BIND_OPTS3 {}
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS3").field("__AnonymousBase_objidl_L9041_C36", &self.__AnonymousBase_objidl_L9041_C36).field("hwnd", &self.hwnd).finish()
    }
}
unsafe impl ::windows_core::Abi for BIND_OPTS3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS3>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS3 {}
impl ::core::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
unsafe impl ::windows_core::Abi for BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BLOB {}
impl ::core::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows_core::Abi for BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
impl ::core::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows_core::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
impl ::core::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn BindMoniker<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(pmk: Param0, grfopt: u32, iidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindMoniker(pmk: ::windows_core::RawPtr, grfopt: u32, iidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        BindMoniker(pmk.into_param().abi(), ::core::mem::transmute(grfopt), ::core::mem::transmute(iidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLCONV(pub i32);
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::core::marker::Copy for CALLCONV {}
impl ::core::clone::Clone for CALLCONV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLCONV {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLCONV {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLCONV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLCONV").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLTYPE(pub i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::core::marker::Copy for CALLTYPE {}
impl ::core::clone::Clone for CALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CATEGORYINFO {
    pub catid: ::windows_core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows_core::Abi for CATEGORYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CATEGORYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
impl ::core::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLSCTX(pub u32);
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
impl ::core::marker::Copy for CLSCTX {}
impl ::core::clone::Clone for CLSCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLSCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CLSCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLSCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLSCTX").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpszprogid: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: ::windows_core::PCWSTR, lpclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpszprogid: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: ::windows_core::PCWSTR, lpclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpsz: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: ::windows_core::PCWSTR, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows_core::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHIDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
impl ::core::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: ::windows_core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl ::core::marker::Copy for COAUTHINFO {}
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHINFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pwszServerPrincName", &self.pwszServerPrincName).field("dwAuthnLevel", &self.dwAuthnLevel).field("dwImpersonationLevel", &self.dwImpersonationLevel).field("pAuthIdentityData", &self.pAuthIdentityData).field("dwCapabilities", &self.dwCapabilities).finish()
    }
}
unsafe impl ::windows_core::Abi for COAUTHINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COAUTHINFO {}
impl ::core::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COINIT(pub u32);
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::core::marker::Copy for COINIT {}
impl ::core::clone::Clone for COINIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COINIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COINITBASE(pub i32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::marker::Copy for COINITBASE {}
impl ::core::clone::Clone for COINITBASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITBASE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COINITBASE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINITBASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITBASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMSD(pub i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::core::marker::Copy for COMSD {}
impl ::core::clone::Clone for COMSD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMSD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMSD {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMSD").field(&self.0).finish()
    }
}
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
pub struct CONNECTDATA {
    pub pUnk: ::core::option::Option<::windows_core::IUnknown>,
    pub dwCookie: u32,
}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), dwCookie: self.dwCookie }
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
unsafe impl ::windows_core::Abi for CONNECTDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
impl ::core::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: ::windows_core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for COSERVERINFO {}
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
unsafe impl ::windows_core::Abi for COSERVERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COSERVERINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COSERVERINFO {}
impl ::core::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COWAIT_FLAGS(pub i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::core::marker::Copy for COWAIT_FLAGS {}
impl ::core::clone::Clone for COWAIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COWAIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COWAIT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COWAIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COWAIT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_DEVICE_CATALOG_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_DEVICE_CATALOG_COOKIE {}
impl ::core::fmt::Debug for CO_DEVICE_CATALOG_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_DEVICE_CATALOG_COOKIE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
impl ::core::marker::Copy for CO_MARSHALING_CONTEXT_ATTRIBUTES {}
impl ::core::clone::Clone for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MARSHALING_CONTEXT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_MTA_USAGE_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_MTA_USAGE_COOKIE {}
impl ::core::fmt::Debug for CO_MTA_USAGE_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MTA_USAGE_COOKIE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[repr(C)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
unsafe impl ::windows_core::Abi for CSPLATFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSPLATFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
impl ::core::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for CUSTDATA {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATA").field("cCustData", &self.cCustData).field("prgCustData", &self.prgCustData).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for CUSTDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CUSTDATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for CUSTDATA {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct CUSTDATAITEM {
    pub guid: ::windows_core::GUID,
    pub varValue: VARIANT,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        Self { guid: self.guid, varValue: self.varValue.clone() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for CUSTDATAITEM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.varValue == other.varValue
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for CUSTDATAITEM {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CWMO_FLAGS(pub i32);
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::core::marker::Copy for CWMO_FLAGS {}
impl ::core::clone::Clone for CWMO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWMO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CWMO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CWMO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWMO_FLAGS").field(&self.0).finish()
    }
}
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for CY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY {}
impl ::core::default::Default for CY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY_0").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
unsafe impl ::windows_core::Abi for CY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY_0 {}
impl ::core::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAddRefServerProcess() -> u32;
        }
        ::core::mem::transmute(CoAddRefServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0, lpvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: *mut ::core::ffi::c_void, lpvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::core::mem::transmute(lpvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowUnmarshalerCLSID(clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        CoAllowUnmarshalerCLSID(::core::mem::transmute(clsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoBuildVersion() -> u32;
        }
        ::core::mem::transmute(CoBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows_core::HRESULT;
        }
        CoCancelCall(::core::mem::transmute(dwthreadid), ::core::mem::transmute(ultimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCopyProxy<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pproxy: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCopyProxy(pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CoCopyProxy(pproxy.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkouter: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateFreeThreadedMarshaler(punkouter: *mut ::core::ffi::c_void, ppunkmarshal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateGuid(pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CoCreateGuid(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(rclsid: *const ::windows_core::GUID, punkouter: Param1, dwclscontext: CLSCTX) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstance(rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclscontext), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(clsid: *const ::windows_core::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, presults: &mut [MULTI_QI]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_core::HRESULT;
        }
        CoCreateInstanceEx(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pserverinfo), presults.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(presults))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(clsid: *const ::windows_core::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, presults: &mut [MULTI_QI]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows_core::HRESULT;
        }
        CoCreateInstanceFromApp(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(reserved), presults.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(presults))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDecrementMTAUsage<'a, Param0: ::windows_core::IntoParam<'a, CO_MTA_USAGE_COOKIE>>(cookie: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows_core::HRESULT;
        }
        CoDecrementMTAUsage(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoDisableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectContext(dwtimeout: u32) -> ::windows_core::HRESULT;
        }
        CoDisconnectContext(::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisconnectObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0, dwreserved: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectObject(punk: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT;
        }
        CoDisconnectObject(punk.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut ::win32_foundation::FILETIME) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut ::win32_foundation::FILETIME) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CoDosDateTimeToFileTime(::core::mem::transmute(ndosdate), ::core::mem::transmute(ndostime), ::core::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoEnableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows_core::Result<::win32_foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::FILETIME>::zeroed();
        CoFileTimeNow(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const ::win32_foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeToDosDateTime(lpfiletime: *const ::win32_foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CoFileTimeToDosDateTime(::core::mem::transmute(lpfiletime), ::core::mem::transmute(lpdosdate), ::core::mem::transmute(lpdostime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeAllLibraries();
        }
        CoFreeAllLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hinst: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeLibrary(hinst: ::win32_foundation::HINSTANCE);
        }
        CoFreeLibrary(hinst.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibraries();
        }
        CoFreeUnusedLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
        }
        CoFreeUnusedLibrariesEx(::core::mem::transmute(dwunloaddelay), ::core::mem::transmute(dwreserved))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows_core::HRESULT;
        }
        CoGetApartmentType(::core::mem::transmute(papttype), ::core::mem::transmute(paptqualifier)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows_core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallContext(riid: *const ::windows_core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetCallContext(::core::mem::transmute(riid), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        CoGetCallerTID(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetCancelObject(::core::mem::transmute(dwthreadid), ::core::mem::transmute(iid), ::core::mem::transmute(ppunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows_core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObject(rclsid: *const ::windows_core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetClassObject(::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(pvreserved), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows_core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetContextToken(ptoken: *mut usize) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<usize>::zeroed();
        CoGetContextToken(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CoGetCurrentLogicalThreadId(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentProcess() -> u32;
        }
        ::core::mem::transmute(CoGetCurrentProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows_core::Result<IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CoGetMalloc(::core::mem::transmute(dwmemcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: ::windows_core::PCWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::core::mem::transmute(pbindoptions), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObjectContext(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetObjectContext(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetPSClsid(riid: *const ::windows_core::GUID, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CoGetPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut ::win32_security::PSECURITY_DESCRIPTOR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut ::win32_security::PSECURITY_DESCRIPTOR) -> ::windows_core::HRESULT;
        }
        CoGetSystemSecurityPermissions(::core::mem::transmute(comsdtype), ::core::mem::transmute(ppsd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetTreatAsClass(clsidold: *const ::windows_core::GUID, pclsidnew: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        CoGetTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoImpersonateClient() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoImpersonateClient() -> ::windows_core::HRESULT;
        }
        CoImpersonateClient().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows_core::Result<CO_MTA_USAGE_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<CO_MTA_USAGE_COOKIE>::zeroed();
        CoIncrementMTAUsage(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoInitialize(::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows_core::HRESULT;
        }
        CoInitializeEx(::core::mem::transmute(pvreserved), ::core::mem::transmute(dwcoinit)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoInitializeSecurity<'a, Param0: ::windows_core::IntoParam<'a, ::win32_security::PSECURITY_DESCRIPTOR>>(psecdesc: Param0, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: ::win32_security::PSECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoInitializeSecurity(psecdesc.into_param().abi(), ::core::mem::transmute(cauthsvc), ::core::mem::transmute(asauthsvc), ::core::mem::transmute(preserved1), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthlist), ::core::mem::transmute(dwcapabilities), ::core::mem::transmute(preserved3)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows_core::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszmachinename: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInvalidateRemoteMachineBindings(pszmachinename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        CoInvalidateRemoteMachineBindings(pszmachinename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows_core::GUID) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsOle1Class(rclsid: *const ::windows_core::GUID) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CoIsOle1Class(::core::mem::transmute(rclsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> ::win32_foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLoadLibrary(lpszlibname: ::windows_core::PCWSTR, bautofree: ::win32_foundation::BOOL) -> ::win32_foundation::HINSTANCE;
        }
        ::core::mem::transmute(CoLoadLibrary(lpszlibname.into_param().abi(), bautofree.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoLockObjectExternal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(punk: Param0, flock: Param1, flastunlockreleases: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLockObjectExternal(punk: *mut ::core::ffi::c_void, flock: ::win32_foundation::BOOL, flastunlockreleases: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        CoLockObjectExternal(punk.into_param().abi(), flock.into_param().abi(), flastunlockreleases.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows_core::HRESULT;
        }
        CoQueryAuthenticationServices(::core::mem::transmute(pcauthsvc), ::core::mem::transmute(asauthsvc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::HRESULT;
        }
        CoQueryClientBlanket(::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: *mut ::core::ffi::c_void, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows_core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_core::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(pwauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterActivationFilter<'a, Param0: ::windows_core::IntoParam<'a, IActivationFilter>>(pactivationfilter: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterActivationFilter(pactivationfilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CoRegisterActivationFilter(pactivationfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterChannelHook<'a, Param1: ::windows_core::IntoParam<'a, IChannelHook>>(extensionuuid: *const ::windows_core::GUID, pchannelhook: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterChannelHook(extensionuuid: *const ::windows_core::GUID, pchannelhook: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CoRegisterChannelHook(::core::mem::transmute(extensionuuid), pchannelhook.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterClassObject<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(rclsid: *const ::windows_core::GUID, punk: Param1, dwclscontext: CLSCTX, flags: REGCLS) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterClassObject(rclsid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, flags: REGCLS, lpdwregister: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        CoRegisterClassObject(::core::mem::transmute(rclsid), punk.into_param().abi(), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(deviceinstanceid: Param0) -> ::windows_core::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: ::windows_core::PCWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<CO_DEVICE_CATALOG_COOKIE>::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CO_DEVICE_CATALOG_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterInitializeSpy<'a, Param0: ::windows_core::IntoParam<'a, IInitializeSpy>>(pspy: Param0) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterInitializeSpy(pspy: ::windows_core::RawPtr, pulicookie: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        CoRegisterInitializeSpy(pspy.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterMallocSpy<'a, Param0: ::windows_core::IntoParam<'a, IMallocSpy>>(pmallocspy: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMallocSpy(pmallocspy: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CoRegisterMallocSpy(pmallocspy.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const ::windows_core::GUID, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterPSClsid(riid: *const ::windows_core::GUID, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        CoRegisterPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterSurrogate<'a, Param0: ::windows_core::IntoParam<'a, ISurrogate>>(psurrogate: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterSurrogate(psurrogate: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CoRegisterSurrogate(psurrogate.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseServerProcess() -> u32;
        }
        ::core::mem::transmute(CoReleaseServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoResumeClassObjects() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoResumeClassObjects() -> ::windows_core::HRESULT;
        }
        CoResumeClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevertToSelf() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevertToSelf() -> ::windows_core::HRESULT;
        }
        CoRevertToSelf().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeClassObject(dwregister: u32) -> ::windows_core::HRESULT;
        }
        CoRevokeClassObject(::core::mem::transmute(dwregister)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<'a, Param0: ::windows_core::IntoParam<'a, CO_DEVICE_CATALOG_COOKIE>>(cookie: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows_core::HRESULT;
        }
        CoRevokeDeviceCatalog(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows_core::HRESULT;
        }
        CoRevokeInitializeSpy(::core::mem::transmute(ulicookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeMallocSpy() -> ::windows_core::HRESULT;
        }
        CoRevokeMallocSpy().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSetCancelObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetCancelObject(punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoSetCancelObject(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSuspendClassObjects() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSuspendClassObjects() -> ::windows_core::HRESULT;
        }
        CoSuspendClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSwitchCallContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pnewobject: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSwitchCallContext(pnewobject: *mut ::core::ffi::c_void, ppoldobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemAlloc(::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
        }
        CoTaskMemFree(::core::mem::transmute(pv))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemRealloc(::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTestCancel() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTestCancel() -> ::windows_core::HRESULT;
        }
        CoTestCancel().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const ::windows_core::GUID, clsidnew: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTreatAsClass(clsidold: *const ::windows_core::GUID, clsidnew: *const ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        CoTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(clsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUninitialize();
        }
        CoUninitialize()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[::win32_foundation::HANDLE]) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const ::win32_foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        CoWaitForMultipleHandles(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), phandles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(phandles)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[::win32_foundation::HANDLE]) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const ::win32_foundation::HANDLE, lpdwindex: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        CoWaitForMultipleObjects(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), phandles.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(phandles)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
unsafe impl ::windows_core::Abi for ComCallData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComCallData>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComCallData {}
impl ::core::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAntiMoniker(ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateAntiMoniker(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows_core::Result<IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateBindCtx(::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateClassMoniker(rclsid: *const ::windows_core::GUID, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateClassMoniker(::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows_core::Result<IDataAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataAdviseHolder(ppdaholder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateDataAdviseHolder(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkouter: Param0, rclsid: *const ::windows_core::GUID, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpszpathname: Param0) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: ::windows_core::PCWSTR, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateFileMoniker(lpszpathname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateGenericComposite<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(pmkfirst: Param0, pmkrest: Param1) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGenericComposite(pmkfirst: ::windows_core::RawPtr, pmkrest: ::windows_core::RawPtr, ppmkcomposite: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateGenericComposite(pmkfirst.into_param().abi(), pmkrest.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateIUriBuilder<'a, Param0: ::windows_core::IntoParam<'a, IUri>>(piuri: Param0, dwflags: u32, dwreserved: usize) -> ::windows_core::Result<IUriBuilder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIUriBuilder(piuri: ::windows_core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: ::windows_core::PCWSTR, lpszitem: ::windows_core::PCWSTR, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateItemMoniker(lpszdelim.into_param().abi(), lpszitem.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateObjrefMoniker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateObjrefMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateObjrefMoniker(punk.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreatePointerMoniker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punk: Param0) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePointerMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows_core::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: ::win32_foundation::HWND, psztitle: ::windows_core::PCWSTR, pibsccaller: ::windows_core::RawPtr, ppibsc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows_core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: ::windows_core::PCWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows_core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: ::windows_core::PCSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::core::mem::transmute(dwencodingflags), ::core::mem::transmute(dwcodepage), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows_core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: ::windows_core::PCWSTR, pwzfragment: ::windows_core::PCWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DATADIR(pub i32);
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::core::marker::Copy for DATADIR {}
impl ::core::clone::Clone for DATADIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DATADIR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DATADIR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DATADIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATADIR").field(&self.0).finish()
    }
}
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOM_CALL_STATE(pub i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::core::marker::Copy for DCOM_CALL_STATE {}
impl ::core::clone::Clone for DCOM_CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOM_CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DCOM_CALL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOM_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOM_CALL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DESCKIND(pub i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::core::marker::Copy for DESCKIND {}
impl ::core::clone::Clone for DESCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DESCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DESCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DESCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESCKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for DISPPARAMS {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPPARAMS").field("rgvarg", &self.rgvarg).field("rgdispidNamedArgs", &self.rgdispidNamedArgs).field("cArgs", &self.cArgs).field("cNamedArgs", &self.cNamedArgs).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for DISPPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISPPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for DISPPARAMS {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_ERRBASE: u32 = 4096u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DVASPECT(pub i32);
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1i32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2i32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4i32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8i32);
impl ::core::marker::Copy for DVASPECT {}
impl ::core::clone::Clone for DVASPECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVASPECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DVASPECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DVASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVTARGETDEVICE").field("tdSize", &self.tdSize).field("tdDriverNameOffset", &self.tdDriverNameOffset).field("tdDeviceNameOffset", &self.tdDeviceNameOffset).field("tdPortNameOffset", &self.tdPortNameOffset).field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset).field("tdData", &self.tdData).finish()
    }
}
unsafe impl ::windows_core::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DVTARGETDEVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
impl ::core::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
unsafe impl ::windows_core::Abi for DWORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
impl ::core::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows_core::HRESULT) -> ::windows_core::HRESULT;
        }
        DcomChannelSetHResult(::core::mem::transmute(pvreserved), ::core::mem::transmute(pulreserved), ::core::mem::transmute(appshr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for ELEMDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for ELEMDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for ELEMDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for ELEMDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for ELEMDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for ELEMDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1i32);
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(32i32);
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(64i32);
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(128i32);
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(256i32);
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2i32);
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4i32);
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(512i32);
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
impl ::core::marker::Copy for EOLE_AUTHENTICATION_CAPABILITIES {}
impl ::core::clone::Clone for EOLE_AUTHENTICATION_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOLE_AUTHENTICATION_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOLE_AUTHENTICATION_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOLE_AUTHENTICATION_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: ::win32_foundation::BSTR,
    pub bstrDescription: ::win32_foundation::BSTR,
    pub bstrHelpFile: ::win32_foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        Self {
            wCode: self.wCode,
            wReserved: self.wReserved,
            bstrSource: self.bstrSource.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrHelpFile: self.bstrHelpFile.clone(),
            dwHelpContext: self.dwHelpContext,
            pvReserved: self.pvReserved,
            pfnDeferredFillIn: self.pfnDeferredFillIn,
            scode: self.scode,
        }
    }
}
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPINFO").field("wCode", &self.wCode).field("wReserved", &self.wReserved).field("bstrSource", &self.bstrSource).field("bstrDescription", &self.bstrDescription).field("bstrHelpFile", &self.bstrHelpFile).field("dwHelpContext", &self.dwHelpContext).field("pvReserved", &self.pvReserved).field("pfnDeferredFillIn", &self.pfnDeferredFillIn.map(|f| f as usize)).field("scode", &self.scode).finish()
    }
}
unsafe impl ::windows_core::Abi for EXCEPINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for EXCEPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wCode == other.wCode && self.wReserved == other.wReserved && self.bstrSource == other.bstrSource && self.bstrDescription == other.bstrDescription && self.bstrHelpFile == other.bstrHelpFile && self.dwHelpContext == other.dwHelpContext && self.pvReserved == other.pvReserved && self.pfnDeferredFillIn.map(|f| f as usize) == other.pfnDeferredFillIn.map(|f| f as usize) && self.scode == other.scode
    }
}
impl ::core::cmp::Eq for EXCEPINFO {}
impl ::core::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EXTCONN(pub i32);
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::core::marker::Copy for EXTCONN {}
impl ::core::clone::Clone for EXTCONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXTCONN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EXTCONN {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXTCONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXTCONN").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows_core::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
impl ::core::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows_core::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
impl ::core::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows_core::Abi for FLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
unsafe impl ::windows_core::Abi for FORMATETC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMATETC>()) == 0 }
    }
}
impl ::core::cmp::Eq for FORMATETC {}
impl ::core::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for FUNCDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for FUNCDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for FUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FUNCDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for FUNCDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FUNCKIND(pub i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::core::marker::Copy for FUNCKIND {}
impl ::core::clone::Clone for FUNCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FUNCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FUNCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for FUNCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FUNCKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows_core::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows_core::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_EH_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_EH_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_EH_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_EH_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_EH_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
impl ::core::marker::Copy for GLOBALOPT_PROPERTIES {}
impl ::core::clone::Clone for GLOBALOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(2i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(4i32);
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(128i32);
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
impl ::core::marker::Copy for GLOBALOPT_RO_FLAGS {}
impl ::core::clone::Clone for GLOBALOPT_RO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
impl ::core::marker::Copy for GLOBALOPT_RPCTP_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_RPCTP_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RPCTP_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RPCTP_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RPCTP_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_UNMARSHALING_POLICY_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_UNMARSHALING_POLICY_VALUES").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(szfilename: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: ::windows_core::PCWSTR, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        GetClassFile(szfilename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows_core::Result<IErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        GetErrorInfo(::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows_core::Result<IRunningObjectTable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        GetRunningObjectTable(::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows_core::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HYPER_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
impl ::core::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IActivationFilter(::windows_core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).HandleActivation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwactivationtype), ::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IActivationFilter> for ::windows_core::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFilter> for ::windows_core::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActivationFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFilter {}
impl ::core::fmt::Debug for IActivationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActivationFilter {
    type Vtable = IActivationFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub HandleActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows_core::GUID, preplacementclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAddrExclusionControl(::windows_core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows_core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCurrentAddrExclusionList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppenumerator)).ok()
    }
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, penumerator: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateAddrExclusionList)(::windows_core::Interface::as_raw(self), penumerator.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAddrExclusionControl> for ::windows_core::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrExclusionControl> for ::windows_core::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAddrExclusionControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrExclusionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrExclusionControl {}
impl ::core::fmt::Debug for IAddrExclusionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrExclusionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAddrTrackingControl(::windows_core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableCOMDynamicAddrTracking)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableCOMDynamicAddrTracking)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IAddrTrackingControl> for ::windows_core::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrTrackingControl> for ::windows_core::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAddrTrackingControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrTrackingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrTrackingControl {}
impl ::core::fmt::Debug for IAddrTrackingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrTrackingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAdviseSink(::windows_core::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows_core::Interface::vtable(self).OnDataChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows_core::Interface::vtable(self).OnViewChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    pub unsafe fn OnRename<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).OnRename)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
    pub unsafe fn OnSave(&self) {
        (::windows_core::Interface::vtable(self).OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnClose(&self) {
        (::windows_core::Interface::vtable(self).OnClose)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IAdviseSink> for ::windows_core::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink> for ::windows_core::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAdviseSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink {}
impl ::core::fmt::Debug for IAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAdviseSink {
    type Vtable = IAdviseSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataChange: usize,
    pub OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr),
    pub OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IAdviseSink2(::windows_core::IUnknown);
impl IAdviseSink2 {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows_core::Interface::vtable(self).base__.OnDataChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows_core::Interface::vtable(self).base__.OnViewChange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    pub unsafe fn OnRename<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).base__.OnRename)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
    pub unsafe fn OnSave(&self) {
        (::windows_core::Interface::vtable(self).base__.OnSave)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnClose(&self) {
        (::windows_core::Interface::vtable(self).base__.OnClose)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows_core::Interface::vtable(self).OnLinkSrcChange)(::windows_core::Interface::as_raw(self), pmk.into_param().abi())
    }
}
impl ::core::convert::From<IAdviseSink2> for ::windows_core::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for ::windows_core::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdviseSink2> for IAdviseSink {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for IAdviseSink {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAdviseSink> for IAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAdviseSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAdviseSink> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAdviseSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink2 {}
impl ::core::fmt::Debug for IAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_Vtbl {
    pub base__: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr),
}
#[repr(transparent)]
pub struct IAgileObject(::windows_core::IUnknown);
impl IAgileObject {}
impl ::core::convert::From<IAgileObject> for ::windows_core::IUnknown {
    fn from(value: IAgileObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows_core::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAgileObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IAsyncManager(::windows_core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompleteCall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result)).ok()
    }
    pub unsafe fn GetCallContext(&self, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCallContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAsyncManager> for ::windows_core::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncManager> for ::windows_core::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAsyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncManager {}
impl ::core::fmt::Debug for IAsyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAsyncManager {
    type Vtable = IAsyncManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CompleteCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAsyncRpcChannelBuffer(::windows_core::IUnknown);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.FreeBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDestCtx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.IsConnected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProtocolVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Send<'a, Param1: ::windows_core::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), psync.into_param().abi(), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDestCtxEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for ::windows_core::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for ::windows_core::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer2> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer2> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncRpcChannelBuffer {}
impl ::core::fmt::Debug for IAsyncRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: ::windows_core::RawPtr, pulstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAuthenticate(::windows_core::IUnknown);
impl IAuthenticate {
    pub unsafe fn Authenticate(&self, phwnd: *mut ::win32_foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
}
impl ::core::convert::From<IAuthenticate> for ::windows_core::IUnknown {
    fn from(value: IAuthenticate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticate> for ::windows_core::IUnknown {
    fn from(value: &IAuthenticate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAuthenticate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAuthenticate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticate {}
impl ::core::fmt::Debug for IAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAuthenticate {
    type Vtable = IAuthenticate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut ::win32_foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAuthenticateEx(::windows_core::IUnknown);
impl IAuthenticateEx {
    pub unsafe fn Authenticate(&self, phwnd: *mut ::win32_foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Authenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut ::win32_foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AuthenticateEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pauthinfo)).ok()
    }
}
impl ::core::convert::From<IAuthenticateEx> for ::windows_core::IUnknown {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for ::windows_core::IUnknown {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAuthenticateEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAuthenticateEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAuthenticateEx> for IAuthenticate {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for IAuthenticate {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAuthenticate> for IAuthenticateEx {
    fn into_param(self) -> ::windows_core::Param<'a, IAuthenticate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAuthenticate> for &'a IAuthenticateEx {
    fn into_param(self) -> ::windows_core::Param<'a, IAuthenticate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticateEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticateEx {}
impl ::core::fmt::Debug for IAuthenticateEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticateEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAuthenticateEx {
    type Vtable = IAuthenticateEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_Vtbl {
    pub base__: IAuthenticate_Vtbl,
    pub AuthenticateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut ::win32_foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBindCtx(::windows_core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterObjectBound)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeObjectBound)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseBoundObjects)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBindOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbindopts)).ok()
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBindOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbindopts)).ok()
    }
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows_core::Result<IRunningObjectTable> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRunningObjectTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningObjectTable>(result__)
    }
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterObjectParam)(::windows_core::Interface::as_raw(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetObjectParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszkey: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectParam)(::windows_core::Interface::as_raw(self), pszkey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn EnumObjectParam(&self) -> ::windows_core::Result<IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumObjectParam)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumString>(result__)
    }
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszkey: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeObjectParam)(::windows_core::Interface::as_raw(self), pszkey.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindCtx> for ::windows_core::IUnknown {
    fn from(value: IBindCtx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindCtx> for ::windows_core::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBindCtx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCtx {}
impl ::core::fmt::Debug for IBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCtx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBindCtx {
    type Vtable = IBindCtx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevokeObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseBoundObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows_core::HRESULT,
    pub GetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows_core::HRESULT,
    pub GetRunningObjectTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RegisterObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevokeObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBindHost(::windows_core::IUnknown);
impl IBindHost {
    pub unsafe fn CreateMoniker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateMoniker)(::windows_core::Interface::as_raw(self), szname.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, IBindCtx>, Param2: ::windows_core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MonikerBindToStorage)(::windows_core::Interface::as_raw(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, IBindCtx>, Param2: ::windows_core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MonikerBindToObject)(::windows_core::Interface::as_raw(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
}
impl ::core::convert::From<IBindHost> for ::windows_core::IUnknown {
    fn from(value: IBindHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindHost> for ::windows_core::IUnknown {
    fn from(value: &IBindHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBindHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHost {}
impl ::core::fmt::Debug for IBindHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBindHost {
    type Vtable = IBindHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, pbc: ::windows_core::RawPtr, ppmk: *mut ::windows_core::RawPtr, dwreserved: u32) -> ::windows_core::HRESULT,
    pub MonikerBindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr, pbc: ::windows_core::RawPtr, pbsc: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MonikerBindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr, pbc: ::windows_core::RawPtr, pbsc: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBindStatusCallback(::windows_core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<'a, Param1: ::windows_core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStartBinding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLowResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved)).ok()
    }
    pub unsafe fn OnProgress<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    pub unsafe fn OnStopBinding<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hresult: ::windows_core::HRESULT, szerror: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStopBinding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBindInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riid: *const ::windows_core::GUID, punk: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindStatusCallback> for ::windows_core::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallback> for ::windows_core::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBindStatusCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallback {}
impl ::core::fmt::Debug for IBindStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStartBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows_core::HRESULT,
    pub OnLowResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub OnStopBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, szerror: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataAvailable: usize,
    pub OnObjectAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBindStatusCallbackEx(::windows_core::IUnknown);
impl IBindStatusCallbackEx {
    pub unsafe fn OnStartBinding<'a, Param1: ::windows_core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnStartBinding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnLowResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved)).ok()
    }
    pub unsafe fn OnProgress<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    pub unsafe fn OnStopBinding<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hresult: ::windows_core::HRESULT, szerror: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnStopBinding)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBindInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnDataAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riid: *const ::windows_core::GUID, punk: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnObjectAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBindInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for ::windows_core::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for ::windows_core::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBindStatusCallback> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows_core::Param<'a, IBindStatusCallback> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBindStatusCallback> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows_core::Param<'a, IBindStatusCallback> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallbackEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallbackEx {}
impl ::core::fmt::Debug for IBindStatusCallbackEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallbackEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base__: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
#[repr(transparent)]
pub struct IBinding(::windows_core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Suspend)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(npriority)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows_core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows_core::PWSTR, pdwreserved: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBindResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclsidprotocol), ::core::mem::transmute(pdwresult), ::core::mem::transmute(pszresult), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBinding> for ::windows_core::IUnknown {
    fn from(value: IBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBinding> for ::windows_core::IUnknown {
    fn from(value: &IBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBinding {}
impl ::core::fmt::Debug for IBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBinding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows_core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows_core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows_core::PWSTR, pdwreserved: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBlockingLock(::windows_core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Lock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn Unlock(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unlock)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IBlockingLock> for ::windows_core::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBlockingLock> for ::windows_core::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBlockingLock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBlockingLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBlockingLock {}
impl ::core::fmt::Debug for IBlockingLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockingLock").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBlockingLock {
    type Vtable = IBlockingLock_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICallFactory(::windows_core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riid: *const ::windows_core::GUID, pctrlunk: Param1, riid2: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CreateCall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), pctrlunk.into_param().abi(), ::core::mem::transmute(riid2), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ICallFactory> for ::windows_core::IUnknown {
    fn from(value: ICallFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFactory> for ::windows_core::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFactory {}
impl ::core::fmt::Debug for ICallFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallFactory {
    type Vtable = ICallFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICancelMethodCalls(::windows_core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulseconds)).ok()
    }
    pub unsafe fn TestCancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TestCancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICancelMethodCalls> for ::windows_core::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICancelMethodCalls> for ::windows_core::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICancelMethodCalls {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICancelMethodCalls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICancelMethodCalls {}
impl ::core::fmt::Debug for ICancelMethodCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICancelMethodCalls").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows_core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICatInformation(::windows_core::IUnknown);
impl ICatInformation {
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows_core::Result<IEnumCATEGORYINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows_core::GUID, lcid: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetCategoryDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(lcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn EnumClassesOfCategories(&self, rgcatidimpl: &[::windows_core::GUID], rgcatidreq: &[::windows_core::GUID]) -> ::windows_core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumClassesOfCategories)(::windows_core::Interface::as_raw(self), rgcatidimpl.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatidimpl)), rgcatidreq.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatidreq)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumGUID>(result__)
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows_core::GUID, rgcatidimpl: &[::windows_core::GUID], rgcatidreq: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsClassOfCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), rgcatidimpl.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatidimpl)), rgcatidreq.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatidreq))).ok()
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumImplCategoriesOfClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumGUID>(result__)
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumReqCategoriesOfClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<ICatInformation> for ::windows_core::IUnknown {
    fn from(value: ICatInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatInformation> for ::windows_core::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICatInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatInformation {}
impl ::core::fmt::Debug for ICatInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICatInformation {
    type Vtable = ICatInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCategoryDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows_core::GUID, lcid: u32, pszdesc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub EnumClassesOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID, ppenumclsid: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ppenumcatid: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ppenumcatid: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICatRegister(::windows_core::IUnknown);
impl ICatRegister {
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterCategories)(::windows_core::Interface::as_raw(self), rgcategoryinfo.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcategoryinfo))).ok()
    }
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegisterCategories)(::windows_core::Interface::as_raw(self), rgcatid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatid))).ok()
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows_core::GUID, rgcatid: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterClassImplCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatid))).ok()
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows_core::GUID, rgcatid: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegisterClassImplCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatid))).ok()
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows_core::GUID, rgcatid: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterClassReqCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatid))).ok()
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows_core::GUID, rgcatid: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnRegisterClassReqCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(rgcatid))).ok()
    }
}
impl ::core::convert::From<ICatRegister> for ::windows_core::IUnknown {
    fn from(value: ICatRegister) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatRegister> for ::windows_core::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICatRegister {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatRegister {}
impl ::core::fmt::Debug for ICatRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatRegister").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICatRegister {
    type Vtable = ICatRegister_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows_core::HRESULT,
    pub UnRegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IChannelHook(::windows_core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32) {
        (::windows_core::Interface::vtable(self).ClientGetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize))
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        (::windows_core::Interface::vtable(self).ClientFillBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer))
    }
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows_core::HRESULT) {
        (::windows_core::Interface::vtable(self).ClientNotify)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep), ::core::mem::transmute(hrfault))
    }
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        (::windows_core::Interface::vtable(self).ServerNotify)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep))
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, hrfault: ::windows_core::HRESULT, pdatasize: *mut u32) {
        (::windows_core::Interface::vtable(self).ServerGetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(hrfault), ::core::mem::transmute(pdatasize))
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows_core::HRESULT) {
        (::windows_core::Interface::vtable(self).ServerFillBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(hrfault))
    }
}
impl ::core::convert::From<IChannelHook> for ::windows_core::IUnknown {
    fn from(value: IChannelHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelHook> for ::windows_core::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IChannelHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelHook {}
impl ::core::fmt::Debug for IChannelHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelHook").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IChannelHook {
    type Vtable = IChannelHook_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ClientGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows_core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub ServerGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, hrfault: ::windows_core::HRESULT, pdatasize: *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows_core::HRESULT),
}
#[repr(transparent)]
pub struct IClassActivator(::windows_core::IUnknown);
impl IClassActivator {
    pub unsafe fn GetClassObject<T: ::windows_core::Interface>(&self, rclsid: *const ::windows_core::GUID, dwclasscontext: u32, locale: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetClassObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(locale), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IClassActivator> for ::windows_core::IUnknown {
    fn from(value: IClassActivator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassActivator> for ::windows_core::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IClassActivator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassActivator {}
impl ::core::fmt::Debug for IClassActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IClassActivator {
    type Vtable = IClassActivator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClassObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IClassFactory(::windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, punkouter: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn LockServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockServer)(::windows_core::Interface::as_raw(self), flock.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IClassFactory> for ::windows_core::IUnknown {
    fn from(value: IClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassFactory> for ::windows_core::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IClassFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassFactory {}
impl ::core::fmt::Debug for IClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IClassFactory {
    type Vtable = IClassFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IClientSecurity(::windows_core::IUnknown);
impl IClientSecurity {
    pub unsafe fn QueryBlanket<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryBlanket)(::windows_core::Interface::as_raw(self), pproxy.into_param().abi(), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    pub unsafe fn SetBlanket<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlanket)(::windows_core::Interface::as_raw(self), pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    pub unsafe fn CopyProxy<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pproxy: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).CopyProxy)(::windows_core::Interface::as_raw(self), pproxy.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IClientSecurity> for ::windows_core::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClientSecurity> for ::windows_core::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IClientSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClientSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClientSecurity {}
impl ::core::fmt::Debug for IClientSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClientSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IClientSecurity {
    type Vtable = IClientSecurity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::HRESULT,
    pub SetBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::HRESULT,
    pub CopyProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComThreadingInfo(::windows_core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows_core::Result<APTTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<APTTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrentApartmentType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<APTTYPE>(result__)
    }
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows_core::Result<THDTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<THDTYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrentThreadType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<THDTYPE>(result__)
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetCurrentLogicalThreadId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCurrentLogicalThreadId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rguid)).ok()
    }
}
impl ::core::convert::From<IComThreadingInfo> for ::windows_core::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComThreadingInfo> for ::windows_core::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComThreadingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadingInfo {}
impl ::core::fmt::Debug for IComThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows_core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows_core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IConnectionPoint(::windows_core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetConnectionInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows_core::Result<IConnectionPointContainer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetConnectionPointContainer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IConnectionPointContainer>(result__)
    }
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punksink: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), punksink.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcookie)).ok()
    }
    pub unsafe fn EnumConnections(&self) -> ::windows_core::Result<IEnumConnections> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumConnections)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IConnectionPoint> for ::windows_core::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPoint> for ::windows_core::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IConnectionPoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPoint {}
impl ::core::fmt::Debug for IConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConnectionPoint {
    type Vtable = IConnectionPoint_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcpc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IConnectionPointContainer(::windows_core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows_core::Result<IEnumConnectionPoints> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumConnectionPoints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumConnectionPoints>(result__)
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<IConnectionPoint> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).FindConnectionPoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IConnectionPoint>(result__)
    }
}
impl ::core::convert::From<IConnectionPointContainer> for ::windows_core::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPointContainer> for ::windows_core::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IConnectionPointContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPointContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPointContainer {}
impl ::core::fmt::Debug for IConnectionPointContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPointContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct IContext(pub u8);
#[repr(transparent)]
pub struct IContextCallback(::windows_core::IUnknown);
impl IContextCallback {
    pub unsafe fn ContextCallback<'a, Param4: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows_core::GUID, imethod: i32, punk: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContextCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pparam), ::core::mem::transmute(riid), ::core::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IContextCallback> for ::windows_core::IUnknown {
    fn from(value: IContextCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextCallback> for ::windows_core::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContextCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextCallback {}
impl ::core::fmt::Debug for IContextCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IContextCallback {
    type Vtable = IContextCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ContextCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: ::windows_core::RawPtr, pparam: *const ComCallData, riid: *const ::windows_core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDLDESC").field("dwReserved", &self.dwReserved).field("wIDLFlags", &self.wIDLFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for IDLDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IDLDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IDLDESC {}
impl ::core::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IDataAdviseHolder(::windows_core::IUnknown);
impl IDataAdviseHolder {
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, IDataObject>, Param3: ::windows_core::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), pdataobject.into_param().abi(), ::core::mem::transmute(pfetc), ::core::mem::transmute(advf), padvise.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwconnection)).ok()
    }
    pub unsafe fn EnumAdvise(&self) -> ::windows_core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumAdvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATDATA>(result__)
    }
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows_core::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOnDataChange)(::windows_core::Interface::as_raw(self), pdataobject.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(advf)).ok()
    }
}
impl ::core::convert::From<IDataAdviseHolder> for ::windows_core::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataAdviseHolder> for ::windows_core::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDataAdviseHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataAdviseHolder {}
impl ::core::fmt::Debug for IDataAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataAdviseHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows_core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows_core::RawPtr, pdwconnection: *mut u32) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows_core::RawPtr, dwreserved: u32, advf: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDataObject(::windows_core::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows_core::Result<STGMEDIUM> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<STGMEDIUM>>::zeroed();
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetcin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<STGMEDIUM>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDataHere)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium)).ok()
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryGetData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc)).ok()
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetCanonicalFormatEtc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatectin), ::core::mem::transmute(pformatetcout)))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows_core::Result<IEnumFORMATETC> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFormatEtc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwdirection), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFORMATETC>(result__)
    }
    pub unsafe fn DAdvise<'a, Param2: ::windows_core::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).DAdvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(advf), padvsink.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DUnadvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwconnection)).ok()
    }
    pub unsafe fn EnumDAdvise(&self) -> ::windows_core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumDAdvise)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IDataObject> for ::windows_core::IUnknown {
    fn from(value: IDataObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataObject> for ::windows_core::IUnknown {
    fn from(value: &IDataObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDataObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObject {}
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDataObject {
    type Vtable = IDataObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetData: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetDataHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetDataHere: usize,
    pub QueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows_core::HRESULT,
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows_core::RawPtr, pdwconnection: *mut u32) -> ::windows_core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDispatch(::windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows_core::GUID, rgsznames: *const ::windows_core::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIDsOfNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
impl ::core::convert::From<IDispatch> for ::windows_core::IUnknown {
    fn from(value: IDispatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispatch> for ::windows_core::IUnknown {
    fn from(value: &IDispatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDispatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDispatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispatch {}
impl ::core::fmt::Debug for IDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDispatch {
    type Vtable = IDispatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const ::windows_core::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct IEnumCATEGORYINFO(::windows_core::IUnknown);
impl IEnumCATEGORYINFO {
    pub unsafe fn Next(&self, rgelt: &mut [CATEGORYINFO], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumCATEGORYINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
impl ::core::convert::From<IEnumCATEGORYINFO> for ::windows_core::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumCATEGORYINFO> for ::windows_core::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCATEGORYINFO {}
impl ::core::fmt::Debug for IEnumCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCATEGORYINFO").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumConnectionPoints(::windows_core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, ppcp: &mut [::core::option::Option<IConnectionPoint>], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppcp.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(ppcp)), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cconnections)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumConnectionPoints> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumConnectionPoints>(result__)
    }
}
impl ::core::convert::From<IEnumConnectionPoints> for ::windows_core::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnectionPoints> for ::windows_core::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumConnectionPoints {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnectionPoints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnectionPoints {}
impl ::core::fmt::Debug for IEnumConnectionPoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnectionPoints").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut ::windows_core::RawPtr, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumConnections(::windows_core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, rgcd: &mut [CONNECTDATA], pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgcd.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgcd)), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cconnections)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumConnections> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IEnumConnections> for ::windows_core::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnections> for ::windows_core::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumConnections {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnections {}
impl ::core::fmt::Debug for IEnumConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumConnections {
    type Vtable = IEnumConnections_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct IEnumContextProps(pub u8);
#[repr(transparent)]
pub struct IEnumFORMATETC(::windows_core::IUnknown);
impl IEnumFORMATETC {
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumFORMATETC> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFORMATETC>(result__)
    }
}
impl ::core::convert::From<IEnumFORMATETC> for ::windows_core::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFORMATETC> for ::windows_core::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumFORMATETC {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFORMATETC {}
impl ::core::fmt::Debug for IEnumFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFORMATETC").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumGUID(::windows_core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, rgelt: &mut [::windows_core::GUID], pceltfetched: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)))
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumGUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<IEnumGUID> for ::windows_core::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumGUID> for ::windows_core::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumGUID {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumGUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumGUID {}
impl ::core::fmt::Debug for IEnumGUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumGUID").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumGUID {
    type Vtable = IEnumGUID_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumMoniker(::windows_core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IMoniker>], pceltfetched: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)))
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IEnumMoniker> for ::windows_core::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMoniker> for ::windows_core::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMoniker {}
impl ::core::fmt::Debug for IEnumMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumMoniker {
    type Vtable = IEnumMoniker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::RawPtr, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumSTATDATA(::windows_core::IUnknown);
impl IEnumSTATDATA {
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IEnumSTATDATA> for ::windows_core::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATDATA> for ::windows_core::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumSTATDATA {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATDATA {}
impl ::core::fmt::Debug for IEnumSTATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATDATA").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumString(::windows_core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(&self, rgelt: &mut [::windows_core::PWSTR], pceltfetched: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)))
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumString>(result__)
    }
}
impl ::core::convert::From<IEnumString> for ::windows_core::IUnknown {
    fn from(value: IEnumString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumString> for ::windows_core::IUnknown {
    fn from(value: &IEnumString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumString {}
impl ::core::fmt::Debug for IEnumString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumString").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumString {
    type Vtable = IEnumString_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumUnknown(::windows_core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<::windows_core::IUnknown>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<IEnumUnknown> for ::windows_core::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumUnknown> for ::windows_core::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumUnknown {}
impl ::core::fmt::Debug for IEnumUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumUnknown {
    type Vtable = IEnumUnknown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IErrorInfo(::windows_core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetGUID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetHelpFile(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetHelpFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetHelpContext(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetHelpContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IErrorInfo> for ::windows_core::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IErrorLog(::windows_core::IUnknown);
impl IErrorLog {
    pub unsafe fn AddError<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpropname: Param0, pexcepinfo: *const EXCEPINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddError)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), ::core::mem::transmute(pexcepinfo)).ok()
    }
}
impl ::core::convert::From<IErrorLog> for ::windows_core::IUnknown {
    fn from(value: IErrorLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorLog> for ::windows_core::IUnknown {
    fn from(value: &IErrorLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IErrorLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IErrorLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLog {}
impl ::core::fmt::Debug for IErrorLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IErrorLog {
    type Vtable = IErrorLog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IExternalConnection(::windows_core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).AddConnection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved)))
    }
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).ReleaseConnection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
impl ::core::convert::From<IExternalConnection> for ::windows_core::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExternalConnection> for ::windows_core::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IExternalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExternalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExternalConnection {}
impl ::core::fmt::Debug for IExternalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExternalConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IExternalConnection {
    type Vtable = IExternalConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32,
    pub ReleaseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: ::win32_foundation::BOOL) -> u32,
}
#[repr(transparent)]
pub struct IFastRundown(::windows_core::IUnknown);
impl IFastRundown {}
impl ::core::convert::From<IFastRundown> for ::windows_core::IUnknown {
    fn from(value: IFastRundown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFastRundown> for ::windows_core::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFastRundown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFastRundown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFastRundown {}
impl ::core::fmt::Debug for IFastRundown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFastRundown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFastRundown {
    type Vtable = IFastRundown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IForegroundTransfer(::windows_core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllowForegroundTransfer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpvreserved)).ok()
    }
}
impl ::core::convert::From<IForegroundTransfer> for ::windows_core::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForegroundTransfer> for ::windows_core::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IForegroundTransfer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForegroundTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForegroundTransfer {}
impl ::core::fmt::Debug for IForegroundTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForegroundTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransfer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGlobalInterfaceTable(::windows_core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0, riid: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterInterfaceInGlobal)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeInterfaceFromGlobal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcookie)).ok()
    }
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInterfaceFromGlobal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcookie), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IGlobalInterfaceTable> for ::windows_core::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalInterfaceTable> for ::windows_core::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGlobalInterfaceTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalInterfaceTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalInterfaceTable {}
impl ::core::fmt::Debug for IGlobalInterfaceTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalInterfaceTable").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGlobalOptions(::windows_core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Set)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows_core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::<usize>::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IGlobalOptions> for ::windows_core::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalOptions> for ::windows_core::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGlobalOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalOptions {}
impl ::core::fmt::Debug for IGlobalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGlobalOptions {
    type Vtable = IGlobalOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpsz: Param0) -> ::windows_core::Result<::windows_core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: ::windows_core::PCWSTR, lpiid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        IIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IInitializeSpy(::windows_core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreInitialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PostInitialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrcoinit), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreUninitialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PostUninitialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
impl ::core::convert::From<IInitializeSpy> for ::windows_core::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitializeSpy> for ::windows_core::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInitializeSpy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeSpy {}
impl ::core::fmt::Debug for IInitializeSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitializeSpy {
    type Vtable = IInitializeSpy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PreInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows_core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcoinit: ::windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows_core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows_core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInternalUnknown(::windows_core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryInternalInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IInternalUnknown> for ::windows_core::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternalUnknown> for ::windows_core::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInternalUnknown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInternalUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalUnknown {}
impl ::core::fmt::Debug for IInternalUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInternalUnknown {
    type Vtable = IInternalUnknown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMachineGlobalObjectTable(::windows_core::IUnknown);
impl IMachineGlobalObjectTable {
    pub unsafe fn RegisterObject<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, clsid: *const ::windows_core::GUID, identifier: Param1, object: Param2) -> ::windows_core::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut MachineGlobalObjectTableRegistrationToken__>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    pub unsafe fn GetObject<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, clsid: *const ::windows_core::GUID, identifier: Param1) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(token)).ok()
    }
}
impl ::core::convert::From<IMachineGlobalObjectTable> for ::windows_core::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMachineGlobalObjectTable> for ::windows_core::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMachineGlobalObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineGlobalObjectTable {}
impl ::core::fmt::Debug for IMachineGlobalObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineGlobalObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, identifier: ::windows_core::PCWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, identifier: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMalloc(::windows_core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Alloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cb)))
    }
    pub unsafe fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Realloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    pub unsafe fn Free(&self, pv: *const ::core::ffi::c_void) {
        (::windows_core::Interface::vtable(self).Free)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv))
    }
    pub unsafe fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv)))
    }
    pub unsafe fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).DidAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv)))
    }
    pub unsafe fn HeapMinimize(&self) {
        (::windows_core::Interface::vtable(self).HeapMinimize)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IMalloc> for ::windows_core::IUnknown {
    fn from(value: IMalloc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMalloc> for ::windows_core::IUnknown {
    fn from(value: &IMalloc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMalloc {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMalloc {}
impl ::core::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMalloc {
    type Vtable = IMalloc_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Alloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IMallocSpy(::windows_core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PreAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbrequest)))
    }
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PostAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pactual)))
    }
    pub unsafe fn PreFree<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PreFree)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    pub unsafe fn PostFree<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fspyed: Param0) {
        (::windows_core::Interface::vtable(self).PostFree)(::windows_core::Interface::as_raw(self), fspyed.into_param().abi())
    }
    pub unsafe fn PreRealloc<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: Param3) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PreRealloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequest), ::core::mem::transmute(cbrequest), ::core::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    pub unsafe fn PostRealloc<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pactual: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PostRealloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    pub unsafe fn PreGetSize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PreGetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    pub unsafe fn PostGetSize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PostGetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PreDidAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).PostDidAlloc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prequest), fspyed.into_param().abi(), ::core::mem::transmute(factual)))
    }
    pub unsafe fn PreHeapMinimize(&self) {
        (::windows_core::Interface::vtable(self).PreHeapMinimize)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PostHeapMinimize(&self) {
        (::windows_core::Interface::vtable(self).PostHeapMinimize)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IMallocSpy> for ::windows_core::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMallocSpy> for ::windows_core::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMallocSpy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMallocSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMallocSpy {}
impl ::core::fmt::Debug for IMallocSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMallocSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMallocSpy {
    type Vtable = IMallocSpy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PreAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub PreFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL) -> *mut ::core::ffi::c_void,
    pub PostFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL),
    pub PreRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL) -> usize,
    pub PostRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL) -> *mut ::core::ffi::c_void,
    pub PreGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL) -> *mut ::core::ffi::c_void,
    pub PostGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: ::win32_foundation::BOOL) -> usize,
    pub PreDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL) -> *mut ::core::ffi::c_void,
    pub PostDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: ::win32_foundation::BOOL, factual: i32) -> i32,
    pub PreHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IMoniker(::windows_core::IUnknown);
impl IMoniker {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.IsDirty)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Load)(::windows_core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, IStream>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Save)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn BindToObject<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindToObject)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    pub unsafe fn BindToStorage<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindToStorage)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn Reduce<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reduce)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), ::core::mem::transmute(dwreducehowfar), ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    pub unsafe fn ComposeWith<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows_core::Result<IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ComposeWith)(::windows_core::Interface::as_raw(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn Enum<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fforward: Param0) -> ::windows_core::Result<IEnumMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Enum)(::windows_core::Interface::as_raw(self), fforward.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMoniker>(result__)
    }
    pub unsafe fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), pmkothermoniker.into_param().abi()).ok()
    }
    pub unsafe fn Hash(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Hash)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn IsRunning<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>, Param2: ::windows_core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsRunning)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows_core::Result<::win32_foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::FILETIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetTimeOfLastChange)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::FILETIME>(result__)
    }
    pub unsafe fn Inverse(&self) -> ::windows_core::Result<IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Inverse)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows_core::Result<IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CommonPrefixWith)(::windows_core::Interface::as_raw(self), pmkother.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn RelativePathTo<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows_core::Result<IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RelativePathTo)(::windows_core::Interface::as_raw(self), pmkother.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn GetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, IMoniker>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ParseDisplayName)(::windows_core::Interface::as_raw(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmkout)).ok()
    }
    pub unsafe fn IsSystemMoniker(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).IsSystemMoniker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMoniker> for ::windows_core::IUnknown {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for ::windows_core::IUnknown {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMoniker> for IPersist {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersist {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for &'a IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMoniker> for IPersistStream {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersistStream {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersistStream> for IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, IPersistStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersistStream> for &'a IMoniker {
    fn into_param(self) -> ::windows_core::Param<'a, IPersistStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMoniker {}
impl ::core::fmt::Debug for IMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMoniker {
    type Vtable = IMoniker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_Vtbl {
    pub base__: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, riidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reduce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows_core::RawPtr, ppmkreduced: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ComposeWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkright: ::windows_core::RawPtr, fonlyifnotgeneric: ::win32_foundation::BOOL, ppmkcomposite: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforward: ::win32_foundation::BOOL, ppenummoniker: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkothermoniker: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Hash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, pmknewlyrunning: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, pfiletime: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommonPrefixWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows_core::RawPtr, ppmkprefix: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RelativePathTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows_core::RawPtr, ppmkrelpath: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, ppszdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub ParseDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr, pmktoleft: ::windows_core::RawPtr, pszdisplayname: ::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSystemMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMultiQI(::windows_core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryMultipleInterfaces)(::windows_core::Interface::as_raw(self), pmqis.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pmqis))).ok()
    }
}
impl ::core::convert::From<IMultiQI> for ::windows_core::IUnknown {
    fn from(value: IMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiQI> for ::windows_core::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMultiQI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiQI {}
impl ::core::fmt::Debug for IMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMultiQI {
    type Vtable = IMultiQI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct INTERFACEINFO {
    pub pUnk: ::core::option::Option<::windows_core::IUnknown>,
    pub iid: ::windows_core::GUID,
    pub wMethod: u16,
}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), iid: self.iid, wMethod: self.wMethod }
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
unsafe impl ::windows_core::Abi for INTERFACEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
impl ::core::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INVOKEKIND(pub i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::core::marker::Copy for INVOKEKIND {}
impl ::core::clone::Clone for INVOKEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INVOKEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for INVOKEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for INVOKEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INVOKEKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct INoMarshal(::windows_core::IUnknown);
impl INoMarshal {}
impl ::core::convert::From<INoMarshal> for ::windows_core::IUnknown {
    fn from(value: INoMarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INoMarshal> for ::windows_core::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INoMarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INoMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INoMarshal {}
impl ::core::fmt::Debug for INoMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INoMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INoMarshal {
    type Vtable = INoMarshal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IOplockStorage(::windows_core::IUnknown);
impl IOplockStorage {
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateStorageEx)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, T: ::windows_core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).OpenStorageEx)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IOplockStorage> for ::windows_core::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOplockStorage> for ::windows_core::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IOplockStorage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockStorage {}
impl ::core::fmt::Debug for IOplockStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IOplockStorage {
    type Vtable = IOplockStorage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPSFactoryBuffer(::windows_core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows_core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateProxy)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppproxy), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateStub<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, riid: *const ::windows_core::GUID, punkserver: Param1) -> ::windows_core::Result<IRpcStubBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateStub)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), punkserver.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRpcStubBuffer>(result__)
    }
}
impl ::core::convert::From<IPSFactoryBuffer> for ::windows_core::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPSFactoryBuffer> for ::windows_core::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPSFactoryBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPSFactoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPSFactoryBuffer {}
impl ::core::fmt::Debug for IPSFactoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSFactoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppproxy: *mut ::windows_core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersist(::windows_core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IPersist> for ::windows_core::IUnknown {
    fn from(value: IPersist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersist> for ::windows_core::IUnknown {
    fn from(value: &IPersist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersist {}
impl ::core::fmt::Debug for IPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersist").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersist {
    type Vtable = IPersist_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistFile(::windows_core::IUnknown);
impl IPersistFile {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveCompleted)(::windows_core::Interface::as_raw(self), pszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetCurFile(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetCurFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPersistFile> for ::windows_core::IUnknown {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for ::windows_core::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistFile> for IPersist {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for IPersist {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for IPersistFile {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for &'a IPersistFile {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistFile {}
impl ::core::fmt::Debug for IPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistFile {
    type Vtable = IPersistFile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, dwmode: u32) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, fremember: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszfilename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistMemory(::windows_core::IUnknown);
impl IPersistMemory {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Load(&self, pmem: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pmem)), pmem.len() as _).ok()
    }
    pub unsafe fn Save<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pmem: &mut [u8], fcleardirty: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pmem)), fcleardirty.into_param().abi(), pmem.len() as _).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitNew)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPersistMemory> for ::windows_core::IUnknown {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for ::windows_core::IUnknown {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistMemory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistMemory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistMemory> for IPersist {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for IPersist {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for IPersistMemory {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for &'a IPersistMemory {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMemory {}
impl ::core::fmt::Debug for IPersistMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMemory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistMemory {
    type Vtable = IPersistMemory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: ::win32_foundation::BOOL, cbsize: u32) -> ::windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistStream(::windows_core::IUnknown);
impl IPersistStream {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, IStream>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IPersistStream> for ::windows_core::IUnknown {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for ::windows_core::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistStream> for IPersist {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for IPersist {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for IPersistStream {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for &'a IPersistStream {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStream {}
impl ::core::fmt::Debug for IPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistStream {
    type Vtable = IPersistStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr, fcleardirty: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPersistStreamInit(::windows_core::IUnknown);
impl IPersistStreamInit {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Load<'a, Param0: ::windows_core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, IStream>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitNew)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPersistStreamInit> for ::windows_core::IUnknown {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for ::windows_core::IUnknown {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPersistStreamInit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistStreamInit> for IPersist {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for IPersist {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for IPersistStreamInit {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPersist> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows_core::Param<'a, IPersist> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStreamInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStreamInit {}
impl ::core::fmt::Debug for IPersistStreamInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStreamInit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistStreamInit {
    type Vtable = IPersistStreamInit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr, fcleardirty: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPipeByte(::windows_core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: &mut [u8], pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeByte> for ::windows_core::IUnknown {
    fn from(value: IPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeByte> for ::windows_core::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPipeByte {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeByte {}
impl ::core::fmt::Debug for IPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPipeByte {
    type Vtable = IPipeByte_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPipeDouble(::windows_core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: &mut [f64], pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: &[f64]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeDouble> for ::windows_core::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeDouble> for ::windows_core::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPipeDouble {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeDouble {}
impl ::core::fmt::Debug for IPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPipeDouble {
    type Vtable = IPipeDouble_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPipeLong(::windows_core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: &mut [i32], pcreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pull)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: &[i32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Push)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeLong> for ::windows_core::IUnknown {
    fn from(value: IPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeLong> for ::windows_core::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPipeLong {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeLong {}
impl ::core::fmt::Debug for IPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPipeLong {
    type Vtable = IPipeLong_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProcessInitControl(::windows_core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetInitializerTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsecondsremaining)).ok()
    }
}
impl ::core::convert::From<IProcessInitControl> for ::windows_core::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessInitControl> for ::windows_core::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProcessInitControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitControl {}
impl ::core::fmt::Debug for IProcessInitControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProcessInitControl {
    type Vtable = IProcessInitControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProcessLock(::windows_core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).AddRefOnProcess)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).ReleaseRefOnProcess)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IProcessLock> for ::windows_core::IUnknown {
    fn from(value: IProcessLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessLock> for ::windows_core::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProcessLock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessLock {}
impl ::core::fmt::Debug for IProcessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessLock").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProcessLock {
    type Vtable = IProcessLock_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[repr(transparent)]
pub struct IProgressNotify(::windows_core::IUnknown);
impl IProgressNotify {
    pub unsafe fn OnProgress<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprogresscurrent), ::core::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IProgressNotify> for ::windows_core::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProgressNotify> for ::windows_core::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProgressNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProgressNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressNotify {}
impl ::core::fmt::Debug for IProgressNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProgressNotify {
    type Vtable = IProgressNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: ::win32_foundation::BOOL, fowner: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IROTData(::windows_core::IUnknown);
impl IROTData {
    pub unsafe fn GetComparisonData(&self, pbdata: &mut [u8], pcbdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComparisonData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pbdata)), pbdata.len() as _, ::core::mem::transmute(pcbdata)).ok()
    }
}
impl ::core::convert::From<IROTData> for ::windows_core::IUnknown {
    fn from(value: IROTData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IROTData> for ::windows_core::IUnknown {
    fn from(value: &IROTData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IROTData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IROTData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IROTData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IROTData {}
impl ::core::fmt::Debug for IROTData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IROTData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IROTData {
    type Vtable = IROTData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetComparisonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReleaseMarshalBuffers(::windows_core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMarshalBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IReleaseMarshalBuffers> for ::windows_core::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReleaseMarshalBuffers> for ::windows_core::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReleaseMarshalBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReleaseMarshalBuffers {}
impl ::core::fmt::Debug for IReleaseMarshalBuffers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReleaseMarshalBuffers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcChannelBuffer(::windows_core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDestCtx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsConnected)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer> for ::windows_core::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer> for ::windows_core::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcChannelBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer {}
impl ::core::fmt::Debug for IRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcChannelBuffer2(::windows_core::IUnknown);
impl IRpcChannelBuffer2 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FreeBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDestCtx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsConnected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProtocolVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for ::windows_core::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for ::windows_core::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer2 {}
impl ::core::fmt::Debug for IRpcChannelBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base__: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcChannelBuffer3(::windows_core::IUnknown);
impl IRpcChannelBuffer3 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.FreeBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDestCtx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.IsConnected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProtocolVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Send)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Receive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(ulsize), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg)).ok()
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCallContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDestCtxEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn RegisterAsync<'a, Param1: ::windows_core::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterAsync)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for ::windows_core::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for ::windows_core::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer2> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRpcChannelBuffer2> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, IRpcChannelBuffer2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer3 {}
impl ::core::fmt::Debug for IRpcChannelBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows_core::HRESULT,
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcHelper(::windows_core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDCOMProtocolVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut ::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetIIDFromOBJREF)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pobjref), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IRpcHelper> for ::windows_core::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcHelper> for ::windows_core::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcHelper {}
impl ::core::fmt::Debug for IRpcHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcHelper {
    type Vtable = IRpcHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows_core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcOptions(::windows_core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Set)(::windows_core::Interface::as_raw(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    pub unsafe fn Query<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows_core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::<usize>::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IRpcOptions> for ::windows_core::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcOptions> for ::windows_core::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcOptions {}
impl ::core::fmt::Debug for IRpcOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcOptions {
    type Vtable = IRpcOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRpcProxyBuffer(::windows_core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IRpcProxyBuffer> for ::windows_core::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcProxyBuffer> for ::windows_core::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcProxyBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcProxyBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcProxyBuffer {}
impl ::core::fmt::Debug for IRpcProxyBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcProxyBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prpcchannelbuffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IRpcStubBuffer(::windows_core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkserver: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), punkserver.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Invoke<'a, Param1: ::windows_core::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows_core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsIIDSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid)))
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).CountRefs)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DebugServerQueryInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        (::windows_core::Interface::vtable(self).DebugServerRelease)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv))
    }
}
impl ::core::convert::From<IRpcStubBuffer> for ::windows_core::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcStubBuffer> for ::windows_core::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcStubBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcStubBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcStubBuffer {}
impl ::core::fmt::Debug for IRpcStubBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcStubBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsIIDSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::core::option::Option<IRpcStubBuffer>,
    pub CountRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub DebugServerQueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IRpcSyntaxNegotiate(::windows_core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NegotiateSyntax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsg)).ok()
    }
}
impl ::core::convert::From<IRpcSyntaxNegotiate> for ::windows_core::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcSyntaxNegotiate> for ::windows_core::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcSyntaxNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcSyntaxNegotiate {}
impl ::core::fmt::Debug for IRpcSyntaxNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcSyntaxNegotiate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRunnableObject(::windows_core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetRunningClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Run<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Run)(::windows_core::Interface::as_raw(self), pbc.into_param().abi()).ok()
    }
    pub unsafe fn IsRunning(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsRunning)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn LockRunning<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockRunning)(::windows_core::Interface::as_raw(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    pub unsafe fn SetContainedObject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fcontained: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContainedObject)(::windows_core::Interface::as_raw(self), fcontained.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRunnableObject> for ::windows_core::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunnableObject> for ::windows_core::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRunnableObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunnableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunnableObject {}
impl ::core::fmt::Debug for IRunnableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRunnableObject {
    type Vtable = IRunnableObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRunningClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub LockRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: ::win32_foundation::BOOL, flastunlockcloses: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetContainedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontained: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IRunningObjectTable(::windows_core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregister)).ok()
    }
    pub unsafe fn IsRunning<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsRunning)(::windows_core::Interface::as_raw(self), pmkobjectname.into_param().abi()).ok()
    }
    pub unsafe fn GetObject<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const ::win32_foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NoteChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregister), ::core::mem::transmute(pfiletime)).ok()
    }
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows_core::Result<::win32_foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::FILETIME>::zeroed();
        (::windows_core::Interface::vtable(self).GetTimeOfLastChange)(::windows_core::Interface::as_raw(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::FILETIME>(result__)
    }
    pub unsafe fn EnumRunning(&self) -> ::windows_core::Result<IEnumMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRunning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IRunningObjectTable> for ::windows_core::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunningObjectTable> for ::windows_core::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRunningObjectTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunningObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunningObjectTable {}
impl ::core::fmt::Debug for IRunningObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: u32, punkobject: *mut ::core::ffi::c_void, pmkobjectname: ::windows_core::RawPtr, pdwregister: *mut u32) -> ::windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows_core::RawPtr, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NoteChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const ::win32_foundation::FILETIME) -> ::windows_core::HRESULT,
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows_core::RawPtr, pfiletime: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT,
    pub EnumRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenummoniker: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISequentialStream(::windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)))
    }
}
impl ::core::convert::From<ISequentialStream> for ::windows_core::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISequentialStream> for ::windows_core::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISequentialStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISequentialStream {}
impl ::core::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISequentialStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISequentialStream {
    type Vtable = ISequentialStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServerSecurity(::windows_core::IUnknown);
impl IServerSecurity {
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryBlanket)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    pub unsafe fn ImpersonateClient(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImpersonateClient)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevertToSelf)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsImpersonating(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsImpersonating)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IServerSecurity> for ::windows_core::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServerSecurity> for ::windows_core::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServerSecurity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServerSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServerSecurity {}
impl ::core::fmt::Debug for IServerSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServerSecurity {
    type Vtable = IServerSecurity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub ImpersonateClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsImpersonating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
}
#[repr(transparent)]
pub struct IServiceProvider(::windows_core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService(&self, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IServiceProvider> for ::windows_core::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceProvider> for ::windows_core::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceProvider {}
impl ::core::fmt::Debug for IServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceProvider {
    type Vtable = IServiceProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub QueryService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IStdMarshalInfo(::windows_core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClassForHandler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(pclsid)).ok()
    }
}
impl ::core::convert::From<IStdMarshalInfo> for ::windows_core::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStdMarshalInfo> for ::windows_core::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStdMarshalInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStdMarshalInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStdMarshalInfo {}
impl ::core::fmt::Debug for IStdMarshalInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStdMarshalInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClassForHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IStream(::windows_core::IUnknown);
impl IStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbwritten)))
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).Seek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(libnewsize)).ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows_core::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: STGC) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockRegion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStream>(result__)
    }
}
impl ::core::convert::From<IStream> for ::windows_core::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ::windows_core::IUnknown {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStream {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows_core::Param<'a, ISequentialStream> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISequentialStream> for &'a IStream {
    fn into_param(self) -> ::windows_core::Param<'a, ISequentialStream> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStream {}
impl ::core::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IStream {
    type Vtable = IStream_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows_core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISupportErrorInfo(::windows_core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InterfaceSupportsErrorInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ISupportErrorInfo> for ::windows_core::IUnknown {
    fn from(value: ISupportErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &ISupportErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISupportErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISupportErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportErrorInfo {}
impl ::core::fmt::Debug for ISupportErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISupportErrorInfo {
    type Vtable = ISupportErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISurrogate(::windows_core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadDllServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsid)).ok()
    }
    pub unsafe fn FreeSurrogate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeSurrogate)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ISurrogate> for ::windows_core::IUnknown {
    fn from(value: ISurrogate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogate> for ::windows_core::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISurrogate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogate {}
impl ::core::fmt::Debug for ISurrogate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISurrogate {
    type Vtable = ISurrogate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LoadDllServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISurrogateService(::windows_core::IUnknown);
impl ISurrogateService {
    pub unsafe fn Init<'a, Param1: ::windows_core::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows_core::GUID, pprocesslock: Param1) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows_core::GUID, apptype: ApplicationType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplicationLaunch)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rguidapplid), ::core::mem::transmute(apptype)).ok()
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplicationFree)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rguidapplid)).ok()
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CatalogRefresh)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulreserved)).ok()
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessShutdown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(shutdowntype)).ok()
    }
}
impl ::core::convert::From<ISurrogateService> for ::windows_core::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogateService> for ::windows_core::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISurrogateService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogateService {}
impl ::core::fmt::Debug for ISurrogateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogateService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISurrogateService {
    type Vtable = ISurrogateService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows_core::GUID, pprocesslock: ::windows_core::RawPtr, pfapplicationaware: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ApplicationLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows_core::GUID, apptype: ApplicationType) -> ::windows_core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows_core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronize(::windows_core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Wait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Signal)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ISynchronize> for ::windows_core::IUnknown {
    fn from(value: ISynchronize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronize> for ::windows_core::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronize {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronize {}
impl ::core::fmt::Debug for ISynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronize {
    type Vtable = ISynchronize_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronizeContainer(::windows_core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<'a, Param0: ::windows_core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddSynchronize)(::windows_core::Interface::as_raw(self), psync.into_param().abi()).ok()
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows_core::Result<ISynchronize> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WaitMultiple)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISynchronize>(result__)
    }
}
impl ::core::convert::From<ISynchronizeContainer> for ::windows_core::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeContainer> for ::windows_core::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronizeContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeContainer {}
impl ::core::fmt::Debug for ISynchronizeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronizeEvent(::windows_core::IUnknown);
impl ISynchronizeEvent {
    pub unsafe fn GetHandle(&self) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    pub unsafe fn SetEventHandle(&self, ph: *const ::win32_foundation::HANDLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ph)).ok()
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ::windows_core::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ::windows_core::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISynchronizeHandle> for ISynchronizeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ISynchronizeHandle> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISynchronizeHandle> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ISynchronizeHandle> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeEvent {}
impl ::core::fmt::Debug for ISynchronizeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *const ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronizeHandle(::windows_core::IUnknown);
impl ISynchronizeHandle {
    pub unsafe fn GetHandle(&self) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).GetHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<ISynchronizeHandle> for ::windows_core::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeHandle> for ::windows_core::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronizeHandle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeHandle {}
impl ::core::fmt::Debug for ISynchronizeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISynchronizeMutex(::windows_core::IUnknown);
impl ISynchronizeMutex {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Wait)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Signal)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMutex(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMutex)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ::windows_core::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ::windows_core::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ISynchronize {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ISynchronize {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISynchronize> for ISynchronizeMutex {
    fn into_param(self) -> ::windows_core::Param<'a, ISynchronize> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISynchronize> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows_core::Param<'a, ISynchronize> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeMutex {}
impl ::core::fmt::Debug for ISynchronizeMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_Vtbl {
    pub base__: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITimeAndNoticeControl(::windows_core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuppressChanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(res1), ::core::mem::transmute(res2)).ok()
    }
}
impl ::core::convert::From<ITimeAndNoticeControl> for ::windows_core::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimeAndNoticeControl> for ::windows_core::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITimeAndNoticeControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimeAndNoticeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimeAndNoticeControl {}
impl ::core::fmt::Debug for ITimeAndNoticeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeAndNoticeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SuppressChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITypeComp(::windows_core::IUnknown);
impl ITypeComp {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Bind<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szname: Param0, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Bind)(::windows_core::Interface::as_raw(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(wflags), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pdesckind), ::core::mem::transmute(pbindptr)).ok()
    }
    pub unsafe fn BindType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szname: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindType)(::windows_core::Interface::as_raw(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pptcomp)).ok()
    }
}
impl ::core::convert::From<ITypeComp> for ::windows_core::IUnknown {
    fn from(value: ITypeComp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeComp> for ::windows_core::IUnknown {
    fn from(value: &ITypeComp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeComp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeComp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeComp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeComp {}
impl ::core::fmt::Debug for ITypeComp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeComp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeComp {
    type Vtable = ITypeComp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows_core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, lhashval: u32, pptinfo: *mut ::windows_core::RawPtr, pptcomp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITypeInfo(::windows_core::IUnknown);
impl ITypeInfo {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows_core::Result<*mut TYPEATTR> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut TYPEATTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows_core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeComp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows_core::Result<*mut FUNCDESC> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut FUNCDESC>::zeroed();
        (::windows_core::Interface::vtable(self).GetFuncDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows_core::Result<*mut VARDESC> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut VARDESC>::zeroed();
        (::windows_core::Interface::vtable(self).GetVarDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut VARDESC>(result__)
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut ::win32_foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetRefTypeOfImplType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetImplTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const ::windows_core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIDsOfNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDocumentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::win32_foundation::BSTR, pbstrname: *mut ::win32_foundation::BSTR, pwordinal: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDllEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRefTypeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddressOfMember)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, punkouter: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetMops)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetContainingTypeLib)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows_core::Interface::vtable(self).ReleaseTypeAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptypeattr))
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows_core::Interface::vtable(self).ReleaseFuncDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfuncdesc))
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows_core::Interface::vtable(self).ReleaseVarDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvardesc))
    }
}
impl ::core::convert::From<ITypeInfo> for ::windows_core::IUnknown {
    fn from(value: ITypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo> for ::windows_core::IUnknown {
    fn from(value: &ITypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo {}
impl ::core::fmt::Debug for ITypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeInfo {
    type Vtable = ITypeInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFuncDesc: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetVarDesc: usize,
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut ::win32_foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows_core::HRESULT,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows_core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgsznames: *const ::windows_core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    Invoke: usize,
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::win32_foundation::BSTR, pbstrname: *mut ::win32_foundation::BSTR, pwordinal: *mut u16) -> ::windows_core::HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetContainingTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows_core::RawPtr, pindex: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub ReleaseTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR),
    #[cfg(not(feature = "Win32_System_Ole"))]
    ReleaseTypeAttr: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub ReleaseFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC),
    #[cfg(not(feature = "Win32_System_Ole"))]
    ReleaseFuncDesc: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub ReleaseVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC),
    #[cfg(not(feature = "Win32_System_Ole"))]
    ReleaseVarDesc: usize,
}
#[repr(transparent)]
pub struct ITypeInfo2(::windows_core::IUnknown);
impl ITypeInfo2 {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows_core::Result<*mut TYPEATTR> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut TYPEATTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows_core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeComp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows_core::Result<*mut FUNCDESC> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut FUNCDESC>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFuncDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows_core::Result<*mut VARDESC> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut VARDESC>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetVarDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut VARDESC>(result__)
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut ::win32_foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRefTypeOfImplType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetImplTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const ::windows_core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIDsOfNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDocumentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::win32_foundation::BSTR, pbstrname: *mut ::win32_foundation::BSTR, pwordinal: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDllEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRefTypeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddressOfMember)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(&self, punkouter: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.CreateInstance)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMops)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetContainingTypeLib)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows_core::Interface::vtable(self).base__.ReleaseTypeAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptypeattr))
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows_core::Interface::vtable(self).base__.ReleaseFuncDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfuncdesc))
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows_core::Interface::vtable(self).base__.ReleaseVarDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvardesc))
    }
    pub unsafe fn GetTypeKind(&self) -> ::windows_core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::<TYPEKIND>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeKind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFuncIndexOfMemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetVarIndexOfMemId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetCustData(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFuncCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetParamCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetVarCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetImplTypeCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut ::win32_foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDocumentation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(memid), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllCustData(&self) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllFuncCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllParamCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllVarCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllImplTypeCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeInfo2> for ::windows_core::IUnknown {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ::windows_core::IUnknown {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITypeInfo2> for ITypeInfo {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ITypeInfo {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITypeInfo> for ITypeInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITypeInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITypeInfo> for &'a ITypeInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITypeInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo2 {}
impl ::core::fmt::Debug for ITypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeInfo2 {
    type Vtable = ITypeInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_Vtbl {
    pub base__: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows_core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFuncCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetParamCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetVarCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetImplTypeCustData: usize,
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut ::win32_foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllFuncCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllParamCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllVarCustData: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllImplTypeCustData: usize,
}
#[repr(transparent)]
pub struct ITypeLib(::windows_core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetTypeInfoCount)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows_core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::<TYPEKIND>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeInfoType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeInfoOfGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows_core::Result<*mut TLIBATTR> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut TLIBATTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetLibAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows_core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeComp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeComp>(result__)
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDocumentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    pub unsafe fn IsName(&self, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pfname: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    pub unsafe fn FindName(&self, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows_core::Interface::vtable(self).ReleaseTLibAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptlibattr))
    }
}
impl ::core::convert::From<ITypeLib> for ::windows_core::IUnknown {
    fn from(value: ITypeLib) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib> for ::windows_core::IUnknown {
    fn from(value: &ITypeLib) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeLib {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeLib {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib {}
impl ::core::fmt::Debug for ITypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeLib {
    type Vtable = ITypeLib_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows_core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pptinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows_core::HRESULT,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pfname: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pptinfo: *mut ::windows_core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows_core::HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR),
}
#[repr(transparent)]
pub struct ITypeLib2(::windows_core::IUnknown);
impl ITypeLib2 {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.GetTypeInfoCount)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows_core::Result<TYPEKIND> {
        let mut result__ = ::core::mem::MaybeUninit::<TYPEKIND>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeInfoType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeInfoOfGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows_core::Result<*mut TLIBATTR> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut TLIBATTR>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLibAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows_core::Result<ITypeComp> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeComp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITypeComp>(result__)
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut ::win32_foundation::BSTR, pbstrdocstring: *mut ::win32_foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDocumentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    pub unsafe fn IsName(&self, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pfname: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    pub unsafe fn FindName(&self, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows_core::Interface::vtable(self).base__.ReleaseTLibAttr)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptlibattr))
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetCustData(&self, guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VARIANT>(result__)
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLibStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcuniquenames), ::core::mem::transmute(pcchuniquenames)).ok()
    }
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut ::win32_foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDocumentation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetAllCustData(&self) -> ::windows_core::Result<CUSTDATA> {
        let mut result__ = ::core::mem::MaybeUninit::<CUSTDATA>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllCustData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeLib2> for ::windows_core::IUnknown {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ::windows_core::IUnknown {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeLib2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeLib2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITypeLib2> for ITypeLib {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ITypeLib {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITypeLib> for ITypeLib2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITypeLib> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ITypeLib> for &'a ITypeLib2 {
    fn into_param(self) -> ::windows_core::Param<'a, ITypeLib> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib2 {}
impl ::core::fmt::Debug for ITypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeLib2 {
    type Vtable = ITypeLib2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_Vtbl {
    pub base__: ITypeLib_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *mut VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetCustData: usize,
    pub GetLibStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows_core::HRESULT,
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut ::win32_foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetAllCustData: usize,
}
#[repr(transparent)]
pub struct ITypeLibRegistration(::windows_core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetLcid(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLcid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWin32Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetWin32Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetWin64Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetWin64Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetHelpDir(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetHelpDir)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistration> for ::windows_core::IUnknown {
    fn from(value: ITypeLibRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistration> for ::windows_core::IUnknown {
    fn from(value: &ITypeLibRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeLibRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeLibRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistration {}
impl ::core::fmt::Debug for ITypeLibRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeLibRegistration {
    type Vtable = ITypeLibRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetLcid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT,
    pub GetWin32Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin32path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin64path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetHelpDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phelpdir: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITypeLibRegistrationReader(::windows_core::IUnknown);
impl ITypeLibRegistrationReader {
    pub unsafe fn EnumTypeLibRegistrations(&self) -> ::windows_core::Result<IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumTypeLibRegistrations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistrationReader> for ::windows_core::IUnknown {
    fn from(value: ITypeLibRegistrationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistrationReader> for ::windows_core::IUnknown {
    fn from(value: &ITypeLibRegistrationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistrationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistrationReader {}
impl ::core::fmt::Debug for ITypeLibRegistrationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistrationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUri(::windows_core::IUnknown);
impl IUri {
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::win32_foundation::BSTR, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyBSTR)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyDWORD)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).HasProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAbsoluteUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthority(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDisplayUri(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetExtension(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetFragment(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFragment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetHost(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetHost)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPassword(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPassword)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPathAndQuery(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPathAndQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetQuery(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetRawUri(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRawUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetSchemeName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSchemeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetUserInfo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetUserInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetUserName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetHostType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetHostType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPort(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetScheme(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetScheme)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetZone(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetZone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), puri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUri> for ::windows_core::IUnknown {
    fn from(value: IUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUri> for ::windows_core::IUnknown {
    fn from(value: &IUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUri {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUri {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUri {}
impl ::core::fmt::Debug for IUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUri").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUri {
    type Vtable = IUri_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropertyBSTR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::win32_foundation::BSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetPropertyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::HRESULT,
    pub HasProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAbsoluteUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetAuthority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthority: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDisplayUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextension: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfragment: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhost: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpassword: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPathAndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrquery: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetRawUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrschemename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetHostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows_core::HRESULT,
    pub GetZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: ::windows_core::RawPtr, pfequal: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUriBuilder(::windows_core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUriSimple)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUriWithFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwuribuilderflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    pub unsafe fn GetIUri(&self) -> ::windows_core::Result<IUri> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUri>(result__)
    }
    pub unsafe fn SetIUri<'a, Param0: ::windows_core::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIUri)(::windows_core::Interface::as_raw(self), piuri.into_param().abi()).ok()
    }
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFragment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchfragment), ::core::mem::transmute(ppwzfragment)).ok()
    }
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHost)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchhost), ::core::mem::transmute(ppwzhost)).ok()
    }
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPassword)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchpassword), ::core::mem::transmute(ppwzpassword)).ok()
    }
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchpath), ::core::mem::transmute(ppwzpath)).ok()
    }
    pub unsafe fn GetPort(&self, pfhasport: *mut ::win32_foundation::BOOL, pdwport: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfhasport), ::core::mem::transmute(pdwport)).ok()
    }
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchquery), ::core::mem::transmute(ppwzquery)).ok()
    }
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSchemeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchschemename), ::core::mem::transmute(ppwzschemename)).ok()
    }
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcchusername), ::core::mem::transmute(ppwzusername)).ok()
    }
    pub unsafe fn SetFragment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFragment)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetHost<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHost)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetPassword<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPassword)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetPort<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPort)(::windows_core::Interface::as_raw(self), fhasport.into_param().abi(), ::core::mem::transmute(dwnewvalue)).ok()
    }
    pub unsafe fn SetQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuery)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetSchemeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSchemeName)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserName)(::windows_core::Interface::as_raw(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwpropertymask)).ok()
    }
    pub unsafe fn HasBeenModified(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).HasBeenModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUriBuilder> for ::windows_core::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUriBuilder> for ::windows_core::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUriBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilder {}
impl ::core::fmt::Debug for IUriBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUriBuilder {
    type Vtable = IUriBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateUriSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateUriWithFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piuri: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasport: *mut ::win32_foundation::BOOL, pdwport: *mut u32) -> ::windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fhasport: ::win32_foundation::BOOL, dwnewvalue: u32) -> ::windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows_core::HRESULT,
    pub HasBeenModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmodified: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUrlMon(::windows_core::IUnknown);
impl IUrlMon {
    pub unsafe fn AsyncGetClassBits<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, IBindCtx>>(&self, rclsid: *const ::windows_core::GUID, psztype: Param1, pszext: Param2, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: Param5, pbc: Param6, dwclasscontext: u32, riid: *const ::windows_core::GUID, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncGetClassBits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), psztype.into_param().abi(), pszext.into_param().abi(), ::core::mem::transmute(dwfileversionms), ::core::mem::transmute(dwfileversionls), pszcodebase.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(riid), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IUrlMon> for ::windows_core::IUnknown {
    fn from(value: IUrlMon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUrlMon> for ::windows_core::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUrlMon {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUrlMon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlMon {}
impl ::core::fmt::Debug for IUrlMon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlMon").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUrlMon {
    type Vtable = IUrlMon_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, psztype: ::windows_core::PCWSTR, pszext: ::windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: ::windows_core::PCWSTR, pbc: ::windows_core::RawPtr, dwclasscontext: u32, riid: *const ::windows_core::GUID, flags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWaitMultiple(::windows_core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows_core::Result<ISynchronize> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WaitMultiple)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(timeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISynchronize>(result__)
    }
    pub unsafe fn AddSynchronize<'a, Param0: ::windows_core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddSynchronize)(::windows_core::Interface::as_raw(self), psync.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWaitMultiple> for ::windows_core::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWaitMultiple> for ::windows_core::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWaitMultiple {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWaitMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaitMultiple {}
impl ::core::fmt::Debug for IWaitMultiple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaitMultiple").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWaitMultiple {
    type Vtable = IWaitMultiple_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl ::core::marker::Copy for LONG_SIZEDARR {}
impl ::core::clone::Clone for LONG_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LONG_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows_core::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LONG_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for LONG_SIZEDARR {}
impl ::core::default::Default for LONG_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type LPEXCEPFINO_DEFERRED_FILLIN = ::core::option::Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows_core::HRESULT>;
pub type LPFNCANUNLOADNOW = ::core::option::Option<unsafe extern "system" fn() -> ::windows_core::HRESULT>;
pub type LPFNGETCLASSOBJECT = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEMCTX(pub i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::core::marker::Copy for MEMCTX {}
impl ::core::clone::Clone for MEMCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MEMCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEMCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMCTX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MKREDUCE(pub i32);
pub const MKRREDUCE_ONE: MKREDUCE = MKREDUCE(196608i32);
pub const MKRREDUCE_TOUSER: MKREDUCE = MKREDUCE(131072i32);
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = MKREDUCE(65536i32);
pub const MKRREDUCE_ALL: MKREDUCE = MKREDUCE(0i32);
impl ::core::marker::Copy for MKREDUCE {}
impl ::core::clone::Clone for MKREDUCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKREDUCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MKREDUCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKREDUCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKREDUCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MKSYS(pub i32);
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
impl ::core::marker::Copy for MKSYS {}
impl ::core::clone::Clone for MKSYS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKSYS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MKSYS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKSYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKSYS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSHCTX(pub i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::core::marker::Copy for MSHCTX {}
impl ::core::clone::Clone for MSHCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MSHCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHCTX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSHLFLAGS(pub i32);
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
impl ::core::marker::Copy for MSHLFLAGS {}
impl ::core::clone::Clone for MSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MSHLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHLFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MULTI_QI {
    pub pIID: *const ::windows_core::GUID,
    pub pItf: ::core::option::Option<::windows_core::IUnknown>,
    pub hr: ::windows_core::HRESULT,
}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        Self { pIID: self.pIID, pItf: self.pItf.clone(), hr: self.hr }
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows_core::Abi for MULTI_QI {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
impl ::core::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows_core::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MachineGlobalObjectTableRegistrationToken__>()) == 0 }
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows_core::IntoParam<'a, IBindCtx>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows_core::RawPtr, szusername: ::windows_core::PCWSTR, pcheaten: *mut u32, ppmk: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MonikerCommonPrefixWith<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, IMoniker>>(pmkthis: Param0, pmkother: Param1) -> ::windows_core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerCommonPrefixWith(pmkthis: ::windows_core::RawPtr, pmkother: ::windows_core::RawPtr, ppmkcommon: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MonikerRelativePathTo<'a, Param0: ::windows_core::IntoParam<'a, IMoniker>, Param1: ::windows_core::IntoParam<'a, IMoniker>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pmksrc: Param0, pmkdest: Param1, ppmkrelpath: *mut ::core::option::Option<IMoniker>, dwreserved: Param3) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerRelativePathTo(pmksrc: ::windows_core::RawPtr, pmkdest: ::windows_core::RawPtr, ppmkrelpath: *mut ::windows_core::RawPtr, dwreserved: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        MonikerRelativePathTo(pmksrc.into_param().abi(), pmkdest.into_param().abi(), ::core::mem::transmute(ppmkrelpath), dwreserved.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PENDINGMSG(pub i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::core::marker::Copy for PENDINGMSG {}
impl ::core::clone::Clone for PENDINGMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGMSG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PENDINGMSG {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGMSG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PENDINGTYPE(pub i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::core::marker::Copy for PENDINGTYPE {}
impl ::core::clone::Clone for PENDINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PENDINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGTYPE").field(&self.0).finish()
    }
}
pub type PFNCONTEXTCALL = ::core::option::Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows_core::HRESULT>;
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows_core::GUID, lplpszprogid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        ProgIDFromCLSID(::core::mem::transmute(clsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
unsafe impl ::windows_core::Abi for QUERYCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERYCONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
impl ::core::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGCLS(pub i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::core::marker::Copy for REGCLS {}
impl ::core::clone::Clone for REGCLS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGCLS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REGCLS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGCLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGCLS").field(&self.0).finish()
    }
}
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(C)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPCOLEMESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
impl ::core::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPCOPT_PROPERTIES(pub i32);
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::core::marker::Copy for RPCOPT_PROPERTIES {}
impl ::core::clone::Clone for RPCOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::core::marker::Copy for RPCOPT_SERVER_LOCALITY_VALUES {}
impl ::core::clone::Clone for RPCOPT_SERVER_LOCALITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_SERVER_LOCALITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_SERVER_LOCALITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_SERVER_LOCALITY_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::core::marker::Copy for RPC_C_AUTHN_LEVEL {}
impl ::core::clone::Clone for RPC_C_AUTHN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RPC_C_AUTHN_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_AUTHN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_IMP_LEVEL(pub u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::core::marker::Copy for RPC_C_IMP_LEVEL {}
impl ::core::clone::Clone for RPC_C_IMP_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_IMP_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RPC_C_IMP_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_IMP_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_IMP_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
unsafe impl ::windows_core::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RemSTGMEDIUM>()) == 0 }
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
impl ::core::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
unsafe impl ::windows_core::Abi for SAFEARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
impl ::core::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
unsafe impl ::windows_core::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAYBOUND>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
impl ::core::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SChannelHookCallInfo {
    pub iid: ::windows_core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows_core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
unsafe impl ::windows_core::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SChannelHookCallInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
impl ::core::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVERCALL(pub i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::core::marker::Copy for SERVERCALL {}
impl ::core::clone::Clone for SERVERCALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVERCALL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SERVERCALL {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVERCALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERCALL").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl ::core::marker::Copy for SHORT_SIZEDARR {}
impl ::core::clone::Clone for SHORT_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHORT_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows_core::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHORT_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHORT_SIZEDARR {}
impl ::core::default::Default for SHORT_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
unsafe impl ::windows_core::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
impl ::core::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
unsafe impl ::windows_core::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
impl ::core::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: ::windows_core::PWSTR,
    pub hr: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows_core::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_SERVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::core::option::Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        Self { formatetc: self.formatetc, advf: self.advf, pAdvSink: self.pAdvSink.clone(), dwConnection: self.dwConnection }
    }
}
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
unsafe impl ::windows_core::Abi for STATDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::core::cmp::Eq for STATDATA {}
impl ::core::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STATSTG {
    pub pwcsName: ::windows_core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: ::win32_foundation::FILETIME,
    pub ctime: ::win32_foundation::FILETIME,
    pub atime: ::win32_foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows_core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl ::core::marker::Copy for STATSTG {}
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATSTG").field("pwcsName", &self.pwcsName).field("type", &self.r#type).field("cbSize", &self.cbSize).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("grfMode", &self.grfMode).field("grfLocksSupported", &self.grfLocksSupported).field("clsid", &self.clsid).field("grfStateBits", &self.grfStateBits).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows_core::Abi for STATSTG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATSTG>()) == 0 }
    }
}
impl ::core::cmp::Eq for STATSTG {}
impl ::core::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STGC(pub u32);
pub const STGC_DEFAULT: STGC = STGC(0u32);
pub const STGC_OVERWRITE: STGC = STGC(1u32);
pub const STGC_ONLYIFCURRENT: STGC = STGC(2u32);
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = STGC(4u32);
pub const STGC_CONSOLIDATE: STGC = STGC(8u32);
impl ::core::marker::Copy for STGC {}
impl ::core::clone::Clone for STGC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STGC {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGC").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STGC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STGC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STGC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STGC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STGC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::core::option::Option<::windows_core::IUnknown>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        Self { tymed: self.tymed, Anonymous: self.Anonymous.clone(), pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows_core::Abi for STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.Anonymous == other.Anonymous && self.pUnkForRelease == other.pUnkForRelease
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: ::win32_graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: ::win32_graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: ::windows_core::PWSTR,
    pub pstm: ::core::mem::ManuallyDrop<::core::option::Option<IStream>>,
    pub pstg: ::core::mem::ManuallyDrop<::core::option::Option<StructuredStorage::IStorage>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows_core::Abi for STGMEDIUM_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STGTY(pub i32);
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::core::marker::Copy for STGTY {}
impl ::core::clone::Clone for STGTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STGTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGTY").field(&self.0).finish()
    }
}
pub const STGTY_REPEAT: i32 = 256i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::core::marker::Copy for STREAM_SEEK {}
impl ::core::clone::Clone for STREAM_SEEK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_SEEK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STREAM_SEEK {
    type Abi = Self;
}
impl ::core::fmt::Debug for STREAM_SEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_SEEK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSKIND(pub i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::core::marker::Copy for SYSKIND {}
impl ::core::clone::Clone for SYSKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SYSKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSKIND").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SetErrorInfo<'a, Param1: ::windows_core::IntoParam<'a, IErrorInfo>>(dwreserved: u32, perrinfo: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetErrorInfo(dwreserved: u32, perrinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        SetErrorInfo(::core::mem::transmute(dwreserved), perrinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ShutdownType(pub i32);
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::core::marker::Copy for ShutdownType {}
impl ::core::clone::Clone for ShutdownType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ShutdownType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: ::windows_core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl ::core::marker::Copy for StorageLayout {}
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
unsafe impl ::windows_core::Abi for StorageLayout {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<StorageLayout>()) == 0 }
    }
}
impl ::core::cmp::Eq for StorageLayout {}
impl ::core::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows_core::GUID, lplpsz: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        StringFromCLSID(::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows_core::GUID, lpsz: &mut [u16]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows_core::GUID, lpsz: ::windows_core::PWSTR, cchmax: i32) -> i32;
        }
        ::core::mem::transmute(StringFromGUID2(::core::mem::transmute(rguid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows_core::GUID, lplpsz: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        StringFromIID(::core::mem::transmute(rclsid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THDTYPE(pub i32);
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::core::marker::Copy for THDTYPE {}
impl ::core::clone::Clone for THDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for THDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for THDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THDTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TLIBATTR {
    pub guid: ::windows_core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLIBATTR").field("guid", &self.guid).field("lcid", &self.lcid).field("syskind", &self.syskind).field("wMajorVerNum", &self.wMajorVerNum).field("wMinorVerNum", &self.wMinorVerNum).field("wLibFlags", &self.wLibFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for TLIBATTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TLIBATTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
impl ::core::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYMED(pub i32);
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
pub const TYMED_FILE: TYMED = TYMED(2i32);
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
pub const TYMED_GDI: TYMED = TYMED(16i32);
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
pub const TYMED_NULL: TYMED = TYMED(0i32);
impl ::core::marker::Copy for TYMED {}
impl ::core::clone::Clone for TYMED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYMED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TYMED {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYMED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYMED").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEATTR {
    pub guid: ::windows_core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: ::windows_core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for TYPEATTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEATTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for TYPEDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for TYPEDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYPEKIND(pub i32);
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::core::marker::Copy for TYPEKIND {}
impl ::core::clone::Clone for TYPEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYPEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TYPEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYPEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYSPEC(pub i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::core::marker::Copy for TYSPEC {}
impl ::core::clone::Clone for TYSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYSPEC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TYSPEC {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYSPEC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URI_CREATE_FLAGS(pub u32);
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
impl ::core::marker::Copy for URI_CREATE_FLAGS {}
impl ::core::clone::Clone for URI_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URI_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for URI_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for URI_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URI_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Uri_PROPERTY(pub i32);
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
impl ::core::marker::Copy for Uri_PROPERTY {}
impl ::core::clone::Clone for Uri_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Uri_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Uri_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for Uri_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: ::windows_core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for VARDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for VARDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        Self { Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARIANT {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union VARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: ::win32_foundation::DECIMAL,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARIANT_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        Self { vt: self.vt, wReserved1: self.wReserved1, wReserved2: self.wReserved2, wReserved3: self.wReserved3, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.vt == other.vt && self.wReserved1 == other.wReserved1 && self.wReserved2 == other.wReserved2 && self.wReserved3 == other.wReserved3 && self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>,
    pub punkVal: ::core::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
    pub pdispVal: ::core::mem::ManuallyDrop<::core::option::Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::win32_foundation::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows_core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: ::win32_foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut ::win32_foundation::DECIMAL,
    pub pcVal: ::windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::core::option::Option<super::Ole::IRecordInfo>,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        Self { pvRecord: self.pvRecord, pRecInfo: self.pRecInfo.clone() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows_core::Abi for VARIANT_0_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VARKIND(pub i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::core::marker::Copy for VARKIND {}
impl ::core::clone::Clone for VARKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VARKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARKIND").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows_core::Abi for WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
impl ::core::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: &str = "_";
#[repr(C)]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl ::core::marker::Copy for uCLSSPEC {}
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for uCLSSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC {}
impl ::core::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union uCLSSPEC_0 {
    pub clsid: ::windows_core::GUID,
    pub pFileExt: ::windows_core::PWSTR,
    pub pMimeType: ::windows_core::PWSTR,
    pub pProgId: ::windows_core::PWSTR,
    pub pFileName: ::windows_core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl ::core::marker::Copy for uCLSSPEC_0 {}
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0 {}
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: ::windows_core::PWSTR,
    pub PolicyId: ::windows_core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_0").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows_core::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
impl ::core::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows_core::GUID,
    pub PolicyId: ::windows_core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_1").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows_core::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
impl ::core::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
unsafe impl ::windows_core::Abi for userFLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
impl ::core::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::core::option::Option<::windows_core::IUnknown>,
}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        Self { pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
unsafe impl ::windows_core::Abi for userSTGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
impl ::core::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows_core::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: ::windows_core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows_core::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
