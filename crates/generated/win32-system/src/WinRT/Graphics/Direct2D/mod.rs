#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(0i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(1i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(2i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(3i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(4i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(5i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(6i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(7i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(8i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(9i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(10i32);
impl ::core::marker::Copy for GRAPHICS_EFFECT_PROPERTY_MAPPING {}
impl ::core::clone::Clone for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRAPHICS_EFFECT_PROPERTY_MAPPING").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IGeometrySource2DInterop(::windows_core::IUnknown);
impl IGeometrySource2DInterop {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn GetGeometry(&self) -> ::windows_core::Result<::win32_graphics::Direct2D::ID2D1Geometry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetGeometry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct2D::ID2D1Geometry>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TryGetGeometryUsingFactory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct2D::ID2D1Factory>>(&self, factory: Param0) -> ::windows_core::Result<::win32_graphics::Direct2D::ID2D1Geometry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).TryGetGeometryUsingFactory)(::windows_core::Interface::as_raw(self), factory.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct2D::ID2D1Geometry>(result__)
    }
}
impl ::core::convert::From<IGeometrySource2DInterop> for ::windows_core::IUnknown {
    fn from(value: IGeometrySource2DInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeometrySource2DInterop> for ::windows_core::IUnknown {
    fn from(value: &IGeometrySource2DInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGeometrySource2DInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGeometrySource2DInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGeometrySource2DInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGeometrySource2DInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeometrySource2DInterop {}
impl ::core::fmt::Debug for IGeometrySource2DInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeometrySource2DInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGeometrySource2DInterop {
    type Vtable = IGeometrySource2DInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2DInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub GetGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    GetGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub TryGetGeometryUsingFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows_core::RawPtr, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    TryGetGeometryUsingFactory: usize,
}
#[repr(transparent)]
pub struct IGraphicsEffectD2D1Interop(::windows_core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    pub unsafe fn GetEffectId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetEffectId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetNamedPropertyMapping<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamedPropertyMapping)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(mapping)).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperty(&self, index: u32) -> ::windows_core::Result<::winrt_foundation::IPropertyValue> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_foundation::IPropertyValue>(result__)
    }
    #[cfg(feature = "Graphics_Effects")]
    pub unsafe fn GetSource(&self, index: u32) -> ::windows_core::Result<::winrt_graphics::Effects::IGraphicsEffectSource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_graphics::Effects::IGraphicsEffectSource>(result__)
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGraphicsEffectD2D1Interop> for ::windows_core::IUnknown {
    fn from(value: IGraphicsEffectD2D1Interop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffectD2D1Interop> for ::windows_core::IUnknown {
    fn from(value: &IGraphicsEffectD2D1Interop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGraphicsEffectD2D1Interop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffectD2D1Interop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectD2D1Interop {}
impl ::core::fmt::Debug for IGraphicsEffectD2D1Interop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectD2D1Interop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGraphicsEffectD2D1Interop {
    type Vtable = IGraphicsEffectD2D1Interop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fc57384_a068_44d7_a331_30982fcf7177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1Interop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetEffectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::HRESULT,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Effects")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, source: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    GetSource: usize,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
}
