#[repr(transparent)]
pub struct IIsolatedEnvironmentInterop(::windows_core::IUnknown);
impl IIsolatedEnvironmentInterop {
    pub unsafe fn GetHostHwndInterop<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, containerhwnd: Param0) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).GetHostHwndInterop)(::windows_core::Interface::as_raw(self), containerhwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IIsolatedEnvironmentInterop> for ::windows_core::IUnknown {
    fn from(value: IIsolatedEnvironmentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIsolatedEnvironmentInterop> for ::windows_core::IUnknown {
    fn from(value: &IIsolatedEnvironmentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIsolatedEnvironmentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIsolatedEnvironmentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedEnvironmentInterop {}
impl ::core::fmt::Debug for IIsolatedEnvironmentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedEnvironmentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerhwnd: ::win32_foundation::HWND, hosthwnd: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
}
