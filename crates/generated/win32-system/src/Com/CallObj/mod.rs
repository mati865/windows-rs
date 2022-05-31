#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows_core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAMEINFO")
            .field("iMethod", &self.iMethod)
            .field("fHasInValues", &self.fHasInValues)
            .field("fHasInOutValues", &self.fHasInOutValues)
            .field("fHasOutValues", &self.fHasOutValues)
            .field("fDerivesFromIDispatch", &self.fDerivesFromIDispatch)
            .field("cInInterfacesMax", &self.cInInterfacesMax)
            .field("cInOutInterfacesMax", &self.cInOutInterfacesMax)
            .field("cOutInterfacesMax", &self.cOutInterfacesMax)
            .field("cTopLevelInInterfaces", &self.cTopLevelInInterfaces)
            .field("iid", &self.iid)
            .field("cMethod", &self.cMethod)
            .field("cParams", &self.cParams)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for CALLFRAMEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALLFRAMEINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEPARAMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAMEPARAMINFO").field("fIn", &self.fIn).field("fOut", &self.fOut).field("stackOffset", &self.stackOffset).field("cbParam", &self.cbParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for CALLFRAMEPARAMINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAMEPARAMINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALLFRAMEPARAMINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLFRAME_COPY(pub i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
impl ::core::marker::Copy for CALLFRAME_COPY {}
impl ::core::clone::Clone for CALLFRAME_COPY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_COPY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLFRAME_COPY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLFRAME_COPY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_COPY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLFRAME_FREE(pub i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
impl ::core::marker::Copy for CALLFRAME_FREE {}
impl ::core::clone::Clone for CALLFRAME_FREE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_FREE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLFRAME_FREE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLFRAME_FREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_FREE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::core::option::Option<::windows_core::IUnknown>,
    pub guidTransferSyntax: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        Self {
            fIn: self.fIn,
            dwDestContext: self.dwDestContext,
            pvDestContext: self.pvDestContext,
            punkReserved: self.punkReserved.clone(),
            guidTransferSyntax: self.guidTransferSyntax,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAME_MARSHALCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAME_MARSHALCONTEXT").field("fIn", &self.fIn).field("dwDestContext", &self.dwDestContext).field("pvDestContext", &self.pvDestContext).field("punkReserved", &self.punkReserved).field("guidTransferSyntax", &self.guidTransferSyntax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows_core::Abi for CALLFRAME_MARSHALCONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAME_MARSHALCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.fIn == other.fIn && self.dwDestContext == other.dwDestContext && self.pvDestContext == other.pvDestContext && self.punkReserved == other.punkReserved && self.guidTransferSyntax == other.guidTransferSyntax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLFRAME_NULL(pub i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
impl ::core::marker::Copy for CALLFRAME_NULL {}
impl ::core::clone::Clone for CALLFRAME_NULL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_NULL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLFRAME_NULL {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLFRAME_NULL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_NULL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLFRAME_WALK(pub i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
impl ::core::marker::Copy for CALLFRAME_WALK {}
impl ::core::clone::Clone for CALLFRAME_WALK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_WALK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CALLFRAME_WALK {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLFRAME_WALK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_WALK").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn CoGetInterceptor<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(iidintercepted: *const ::windows_core::GUID, punkouter: Param1, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptor(iidintercepted: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetInterceptor(::core::mem::transmute(iidintercepted), punkouter.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::ITypeInfo>>(iidintercepted: *const ::windows_core::GUID, punkouter: Param1, typeinfo: Param2, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, typeinfo: ::windows_core::RawPtr, iid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetInterceptorFromTypeInfo(::core::mem::transmute(iidintercepted), punkouter.into_param().abi(), typeinfo.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct ICallFrame(::windows_core::IUnknown);
impl ICallFrame {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<CALLFRAMEINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<CALLFRAMEINFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CALLFRAMEINFO>(result__)
    }
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut ::windows_core::GUID, pimethod: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIIDAndMethod)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(piid), ::core::mem::transmute(pimethod)).ok()
    }
    pub unsafe fn GetNames(&self, pwszinterface: *mut ::windows_core::PWSTR, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszinterface), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackLocation(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetStackLocation)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn SetStackLocation(&self, pvstack: *const ::core::ffi::c_void) {
        (::windows_core::Interface::vtable(self).SetStackLocation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvstack))
    }
    pub unsafe fn SetReturnValue(&self, hr: ::windows_core::HRESULT) {
        (::windows_core::Interface::vtable(self).SetReturnValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr))
    }
    pub unsafe fn GetReturnValue(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetReturnValue)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> ::windows_core::Result<CALLFRAMEPARAMINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<CALLFRAMEPARAMINFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetParamInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iparam), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CALLFRAMEPARAMINFO>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const super::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParam)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iparam), ::core::mem::transmute(pvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParam(&self, iparam: u32) -> ::windows_core::Result<super::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetParam)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iparam), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::VARIANT>(result__)
    }
    pub unsafe fn Copy<'a, Param1: ::windows_core::IntoParam<'a, ICallFrameWalker>>(&self, copycontrol: CALLFRAME_COPY, pwalker: Param1) -> ::windows_core::Result<ICallFrame> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(copycontrol), pwalker.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICallFrame>(result__)
    }
    pub unsafe fn Free<'a, Param0: ::windows_core::IntoParam<'a, ICallFrame>, Param1: ::windows_core::IntoParam<'a, ICallFrameWalker>, Param2: ::windows_core::IntoParam<'a, ICallFrameWalker>, Param4: ::windows_core::IntoParam<'a, ICallFrameWalker>>(&self, pframeargsdest: Param0, pwalkerdestfree: Param1, pwalkercopy: Param2, freeflags: u32, pwalkerfree: Param4, nullflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Free)(::windows_core::Interface::as_raw(self), pframeargsdest.into_param().abi(), pwalkerdestfree.into_param().abi(), pwalkercopy.into_param().abi(), ::core::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::core::mem::transmute(nullflags)).ok()
    }
    pub unsafe fn FreeParam<'a, Param2: ::windows_core::IntoParam<'a, ICallFrameWalker>>(&self, iparam: u32, freeflags: u32, pwalkerfree: Param2, nullflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeParam)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iparam), ::core::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::core::mem::transmute(nullflags)).ok()
    }
    pub unsafe fn WalkFrame<'a, Param1: ::windows_core::IntoParam<'a, ICallFrameWalker>>(&self, walkwhat: u32, pwalker: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WalkFrame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(walkwhat), pwalker.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMarshalSizeMax)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmshlcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: &[u8], pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Marshal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmshlcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(pcbbufferused), ::core::mem::transmute(pdatarep), ::core::mem::transmute(prpcflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal(&self, pbuffer: &[u8], datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Unmarshal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMarshalData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(ibfirstrelease), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn Invoke(&self, pvreceiver: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvreceiver)).ok()
    }
}
impl ::core::convert::From<ICallFrame> for ::windows_core::IUnknown {
    fn from(value: ICallFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFrame> for ::windows_core::IUnknown {
    fn from(value: &ICallFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrame {}
impl ::core::fmt::Debug for ICallFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallFrame {
    type Vtable = ICallFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd573b4b0_894e_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrame_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInfo: usize,
    pub GetIIDAndMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID, pimethod: *mut u32) -> ::windows_core::HRESULT,
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinterface: *mut ::windows_core::PWSTR, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetStackLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub SetStackLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void),
    pub SetReturnValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT),
    pub GetReturnValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetParamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetParamInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetParam: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetParam: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: ::windows_core::RawPtr, ppframe: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframeargsdest: ::windows_core::RawPtr, pwalkerdestfree: ::windows_core::RawPtr, pwalkercopy: ::windows_core::RawPtr, freeflags: u32, pwalkerfree: ::windows_core::RawPtr, nullflags: u32) -> ::windows_core::HRESULT,
    pub FreeParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: ::windows_core::RawPtr, nullflags: u32) -> ::windows_core::HRESULT,
    pub WalkFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMarshalSizeMax: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Marshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Marshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICallFrameEvents(::windows_core::IUnknown);
impl ICallFrameEvents {
    pub unsafe fn OnCall<'a, Param0: ::windows_core::IntoParam<'a, ICallFrame>>(&self, pframe: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCall)(::windows_core::Interface::as_raw(self), pframe.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICallFrameEvents> for ::windows_core::IUnknown {
    fn from(value: ICallFrameEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFrameEvents> for ::windows_core::IUnknown {
    fn from(value: &ICallFrameEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallFrameEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallFrameEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFrameEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFrameEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrameEvents {}
impl ::core::fmt::Debug for ICallFrameEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrameEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallFrameEvents {
    type Vtable = ICallFrameEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd5e0843_fc91_11d0_97d7_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframe: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICallFrameWalker(::windows_core::IUnknown);
impl ICallFrameWalker {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWalkInterface<'a, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, iid: *const ::windows_core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: Param2, fout: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnWalkInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvinterface), fin.into_param().abi(), fout.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICallFrameWalker> for ::windows_core::IUnknown {
    fn from(value: ICallFrameWalker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFrameWalker> for ::windows_core::IUnknown {
    fn from(value: &ICallFrameWalker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallFrameWalker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallFrameWalker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFrameWalker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFrameWalker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrameWalker {}
impl ::core::fmt::Debug for ICallFrameWalker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrameWalker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallFrameWalker {
    type Vtable = ICallFrameWalker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08b23919_392d_11d2_b8a4_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameWalker_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWalkInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWalkInterface: usize,
}
#[repr(transparent)]
pub struct ICallIndirect(::windows_core::IUnknown);
impl ICallIndirect {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows_core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CallIndirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phrreturn), ::core::mem::transmute(imethod), ::core::mem::transmute(pvargs), ::core::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMethodInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(pinfo), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetStackSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: *mut ::windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(piid), ::core::mem::transmute(pfderivesfromidispatch), ::core::mem::transmute(pcmethod), ::core::mem::transmute(pwszinterface)).ok()
    }
}
impl ::core::convert::From<ICallIndirect> for ::windows_core::IUnknown {
    fn from(value: ICallIndirect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallIndirect> for ::windows_core::IUnknown {
    fn from(value: &ICallIndirect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallIndirect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallIndirect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallIndirect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallIndirect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallIndirect {}
impl ::core::fmt::Debug for ICallIndirect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallIndirect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallIndirect {
    type Vtable = ICallIndirect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd573b4b1_894e_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallIndirect_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CallIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows_core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMethodInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMethodInfo: usize,
    pub GetStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIID: usize,
}
#[repr(transparent)]
pub struct ICallInterceptor(::windows_core::IUnknown);
impl ICallInterceptor {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows_core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CallIndirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phrreturn), ::core::mem::transmute(imethod), ::core::mem::transmute(pvargs), ::core::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMethodInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(pinfo), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStackSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: *mut ::windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(piid), ::core::mem::transmute(pfderivesfromidispatch), ::core::mem::transmute(pcmethod), ::core::mem::transmute(pwszinterface)).ok()
    }
    pub unsafe fn RegisterSink<'a, Param0: ::windows_core::IntoParam<'a, ICallFrameEvents>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisteredSink(&self) -> ::windows_core::Result<ICallFrameEvents> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRegisteredSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICallFrameEvents>(result__)
    }
}
impl ::core::convert::From<ICallInterceptor> for ::windows_core::IUnknown {
    fn from(value: ICallInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallInterceptor> for ::windows_core::IUnknown {
    fn from(value: &ICallInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICallInterceptor> for ICallIndirect {
    fn from(value: ICallInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallInterceptor> for ICallIndirect {
    fn from(value: &ICallInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICallIndirect> for ICallInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ICallIndirect> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICallIndirect> for &'a ICallInterceptor {
    fn into_param(self) -> ::windows_core::Param<'a, ICallIndirect> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallInterceptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallInterceptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallInterceptor {}
impl ::core::fmt::Debug for ICallInterceptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallInterceptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallInterceptor {
    type Vtable = ICallInterceptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60c7ca75_896d_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallInterceptor_Vtbl {
    pub base__: ICallIndirect_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRegisteredSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICallUnmarshal(::windows_core::IUnknown);
impl ICallUnmarshal {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal<'a, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, imethod: u32, pbuffer: &[u8], fforcebuffercopy: Param3, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unmarshal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, fforcebuffercopy.into_param().abi(), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext), ::core::mem::transmute(pcbunmarshalled), ::core::mem::transmute(ppframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseMarshalData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imethod), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbuffer)), pbuffer.len() as _, ::core::mem::transmute(ibfirstrelease), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext)).ok()
    }
}
impl ::core::convert::From<ICallUnmarshal> for ::windows_core::IUnknown {
    fn from(value: ICallUnmarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallUnmarshal> for ::windows_core::IUnknown {
    fn from(value: &ICallUnmarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICallUnmarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICallUnmarshal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallUnmarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallUnmarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallUnmarshal {}
impl ::core::fmt::Debug for ICallUnmarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallUnmarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICallUnmarshal {
    type Vtable = ICallUnmarshal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5333b003_2e42_11d2_b89d_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallUnmarshal_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
}
#[repr(transparent)]
pub struct IInterfaceRelated(::windows_core::IUnknown);
impl IInterfaceRelated {
    pub unsafe fn SetIID(&self, iid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn GetIID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetIID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IInterfaceRelated> for ::windows_core::IUnknown {
    fn from(value: IInterfaceRelated) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInterfaceRelated> for ::windows_core::IUnknown {
    fn from(value: &IInterfaceRelated) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInterfaceRelated {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInterfaceRelated {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInterfaceRelated {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInterfaceRelated {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInterfaceRelated {}
impl ::core::fmt::Debug for IInterfaceRelated {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInterfaceRelated").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInterfaceRelated {
    type Vtable = IInterfaceRelated_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1fb5a79_7706_11d1_adba_00c04fc2adc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterfaceRelated_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}