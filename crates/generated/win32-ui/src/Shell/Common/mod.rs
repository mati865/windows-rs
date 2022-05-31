#[repr(C)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: ::windows_core::PCWSTR,
    pub pszSpec: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMDLG_FILTERSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMDLG_FILTERSPEC").field("pszName", &self.pszName).field("pszSpec", &self.pszSpec).finish()
    }
}
unsafe impl ::windows_core::Abi for COMDLG_FILTERSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMDLG_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMDLG_FILTERSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMDLG_FILTERSPEC {}
impl ::core::default::Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_SCALE_FACTOR(pub i32);
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(0i32);
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(100i32);
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(120i32);
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(125i32);
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(140i32);
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(150i32);
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(160i32);
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(175i32);
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(180i32);
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(200i32);
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(225i32);
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(250i32);
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(300i32);
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(350i32);
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(400i32);
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(450i32);
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(500i32);
impl ::core::marker::Copy for DEVICE_SCALE_FACTOR {}
impl ::core::clone::Clone for DEVICE_SCALE_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_SCALE_FACTOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DEVICE_SCALE_FACTOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_SCALE_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_SCALE_FACTOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IObjectArray(::windows_core::IUnknown);
impl IObjectArray {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt<T: ::windows_core::Interface>(&self, uiindex: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uiindex), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IObjectArray> for ::windows_core::IUnknown {
    fn from(value: IObjectArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectArray> for ::windows_core::IUnknown {
    fn from(value: &IObjectArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectArray {}
impl ::core::fmt::Debug for IObjectArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectArray").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectArray {
    type Vtable = IObjectArray_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectArray_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectCollection(::windows_core::IUnknown);
impl IObjectCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt<T: ::windows_core::Interface>(&self, uiindex: u32) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uiindex), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn AddObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddObject)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn AddFromArray<'a, Param0: ::windows_core::IntoParam<'a, IObjectArray>>(&self, poasource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddFromArray)(::windows_core::Interface::as_raw(self), poasource.into_param().abi()).ok()
    }
    pub unsafe fn RemoveObjectAt(&self, uiindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveObjectAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uiindex)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IObjectCollection> for ::windows_core::IUnknown {
    fn from(value: IObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectCollection> for ::windows_core::IUnknown {
    fn from(value: &IObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IObjectCollection> for IObjectArray {
    fn from(value: IObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectCollection> for IObjectArray {
    fn from(value: &IObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectArray> for IObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectArray> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectArray> for &'a IObjectCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectArray> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectCollection {}
impl ::core::fmt::Debug for IObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectCollection {
    type Vtable = IObjectCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5632b1a4_e38a_400a_928a_d4cd63230295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectCollection_Vtbl {
    pub base__: IObjectArray_Vtbl,
    pub AddObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poasource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for ITEMIDLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ITEMIDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ITEMIDLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for ITEMIDLIST {}
impl ::core::default::Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PERCEIVED(pub i32);
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = PERCEIVED(-2i32);
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = PERCEIVED(-1i32);
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = PERCEIVED(0i32);
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = PERCEIVED(1i32);
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = PERCEIVED(2i32);
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = PERCEIVED(3i32);
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = PERCEIVED(4i32);
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = PERCEIVED(5i32);
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = PERCEIVED(6i32);
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = PERCEIVED(7i32);
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = PERCEIVED(8i32);
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = PERCEIVED(9i32);
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = PERCEIVED(10i32);
pub const PERCEIVED_TYPE_LAST: PERCEIVED = PERCEIVED(10i32);
impl ::core::marker::Copy for PERCEIVED {}
impl ::core::clone::Clone for PERCEIVED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PERCEIVED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PERCEIVED {
    type Abi = Self;
}
impl ::core::fmt::Debug for PERCEIVED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERCEIVED").field(&self.0).finish()
    }
}
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHCOLSTATE(pub i32);
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = SHCOLSTATE(0i32);
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = SHCOLSTATE(1i32);
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = SHCOLSTATE(2i32);
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = SHCOLSTATE(3i32);
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = SHCOLSTATE(15i32);
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = SHCOLSTATE(16i32);
pub const SHCOLSTATE_SLOW: SHCOLSTATE = SHCOLSTATE(32i32);
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = SHCOLSTATE(64i32);
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = SHCOLSTATE(128i32);
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = SHCOLSTATE(256i32);
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = SHCOLSTATE(512i32);
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = SHCOLSTATE(1024i32);
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = SHCOLSTATE(2048i32);
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = SHCOLSTATE(65536i32);
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = SHCOLSTATE(131072i32);
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = SHCOLSTATE(262144i32);
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = SHCOLSTATE(4096i32);
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = SHCOLSTATE(8192i32);
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = SHCOLSTATE(16384i32);
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = SHCOLSTATE(61440i32);
impl ::core::marker::Copy for SHCOLSTATE {}
impl ::core::clone::Clone for SHCOLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHCOLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SHCOLSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHCOLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCOLSTATE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
impl ::core::marker::Copy for SHELLDETAILS {}
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for SHELLDETAILS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHELLDETAILS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHELLDETAILS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHELLDETAILS {}
impl ::core::default::Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl ::core::marker::Copy for SHITEMID {}
impl ::core::clone::Clone for SHITEMID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for SHITEMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHITEMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHITEMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHITEMID {}
impl ::core::default::Default for SHITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl ::core::marker::Copy for STRRET {}
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for STRRET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRRET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRRET>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRRET {}
impl ::core::default::Default for STRRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union STRRET_0 {
    pub pOleStr: ::windows_core::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
impl ::core::marker::Copy for STRRET_0 {}
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for STRRET_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRRET_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRRET_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRRET_0 {}
impl ::core::default::Default for STRRET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STRRET_TYPE(pub i32);
pub const STRRET_WSTR: STRRET_TYPE = STRRET_TYPE(0i32);
pub const STRRET_OFFSET: STRRET_TYPE = STRRET_TYPE(1i32);
pub const STRRET_CSTR: STRRET_TYPE = STRRET_TYPE(2i32);
impl ::core::marker::Copy for STRRET_TYPE {}
impl ::core::clone::Clone for STRRET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STRRET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STRRET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STRRET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRRET_TYPE").field(&self.0).finish()
    }
}