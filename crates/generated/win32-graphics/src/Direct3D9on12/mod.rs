#[repr(C)]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: ::win32_foundation::BOOL,
    pub pD3D12Device: ::core::option::Option<::windows_core::IUnknown>,
    pub ppD3D12Queues: [::core::option::Option<::windows_core::IUnknown>; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
impl ::core::clone::Clone for D3D9ON12_ARGS {
    fn clone(&self) -> Self {
        Self {
            Enable9On12: self.Enable9On12,
            pD3D12Device: self.pD3D12Device.clone(),
            ppD3D12Queues: self.ppD3D12Queues.clone(),
            NumQueues: self.NumQueues,
            NodeMask: self.NodeMask,
        }
    }
}
impl ::core::fmt::Debug for D3D9ON12_ARGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D9ON12_ARGS").field("Enable9On12", &self.Enable9On12).field("pD3D12Device", &self.pD3D12Device).field("ppD3D12Queues", &self.ppD3D12Queues).field("NumQueues", &self.NumQueues).field("NodeMask", &self.NodeMask).finish()
    }
}
unsafe impl ::windows_core::Abi for D3D9ON12_ARGS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for D3D9ON12_ARGS {
    fn eq(&self, other: &Self) -> bool {
        self.Enable9On12 == other.Enable9On12 && self.pD3D12Device == other.pD3D12Device && self.ppD3D12Queues == other.ppD3D12Queues && self.NumQueues == other.NumQueues && self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D9ON12_ARGS {}
impl ::core::default::Default for D3D9ON12_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
        }
        ::core::mem::transmute(Direct3DCreate9On12(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        Direct3DCreate9On12Ex(::core::mem::transmute(sdkversion), ::core::mem::transmute(poverridelist), ::core::mem::transmute(numoverrideentries), ::core::mem::transmute(ppoutputinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDirect3DDevice9On12(::windows_core::IUnknown);
impl IDirect3DDevice9On12 {
    pub unsafe fn GetD3D12Device(&self, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetD3D12Device)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppvdevice)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn UnwrapUnderlyingResource<'a, Param0: ::windows_core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>, Param1: ::windows_core::IntoParam<'a, super::Direct3D12::ID3D12CommandQueue>>(&self, presource: Param0, pcommandqueue: Param1, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnwrapUnderlyingResource)(::windows_core::Interface::as_raw(self), presource.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvresource12)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn ReturnUnderlyingResource<'a, Param0: ::windows_core::IntoParam<'a, super::Direct3D9::IDirect3DResource9>>(&self, presource: Param0, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReturnUnderlyingResource)(::windows_core::Interface::as_raw(self), presource.into_param().abi(), ::core::mem::transmute(numsync), ::core::mem::transmute(psignalvalues), ::core::mem::transmute(ppfences)).ok()
    }
}
impl ::core::convert::From<IDirect3DDevice9On12> for ::windows_core::IUnknown {
    fn from(value: IDirect3DDevice9On12) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice9On12> for ::windows_core::IUnknown {
    fn from(value: &IDirect3DDevice9On12) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDirect3DDevice9On12 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DDevice9On12 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9On12 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9On12 {}
impl ::core::fmt::Debug for IDirect3DDevice9On12 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9On12").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDirect3DDevice9On12 {
    type Vtable = IDirect3DDevice9On12_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7fda234_b589_4049_940d_8878977531c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9On12_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presource: ::windows_core::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    ReturnUnderlyingResource: usize,
}
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12 = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12Ex = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut ::core::option::Option<super::Direct3D9::IDirect3D9Ex>) -> ::windows_core::HRESULT>;
