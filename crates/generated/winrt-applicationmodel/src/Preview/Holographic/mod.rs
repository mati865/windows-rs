pub struct HolographicApplicationPreview;
impl HolographicApplicationPreview {
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> ::windows_core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentViewPresentedOnHolographicDisplay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<'a, Param0: ::windows_core::IntoParam<'a, super::super::Activation::IActivatedEventArgs>>(activatedeventargs: Param0) -> ::windows_core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHolographicActivation)(::windows_core::Interface::as_raw(this), activatedeventargs.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IHolographicApplicationPreviewStatics<R, F: FnOnce(&IHolographicApplicationPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicApplicationPreview, IHolographicApplicationPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for HolographicApplicationPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicKeyboardPlacementOverridePreview(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicKeyboardPlacementOverridePreview {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacementOverride)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector2>>(&self, coordinatesystem: Param0, topcenterposition: Param1, normal: Param2, maxsize: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), normal.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetPlacementOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows_core::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicKeyboardPlacementOverridePreview>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<R, F: FnOnce(&IHolographicKeyboardPlacementOverridePreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicKeyboardPlacementOverridePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicKeyboardPlacementOverridePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicKeyboardPlacementOverridePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicKeyboardPlacementOverridePreview").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for HolographicKeyboardPlacementOverridePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview;{c8a8ce3a-dfde-5a14-8d5f-182c526dd9c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicKeyboardPlacementOverridePreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows_core::IUnknown {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows_core::IUnknown {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicKeyboardPlacementOverridePreview> for ::windows_core::IInspectable {
    fn from(value: HolographicKeyboardPlacementOverridePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicKeyboardPlacementOverridePreview> for ::windows_core::IInspectable {
    fn from(value: &HolographicKeyboardPlacementOverridePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicKeyboardPlacementOverridePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicKeyboardPlacementOverridePreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicApplicationPreviewStatics {
    type Vtable = IHolographicApplicationPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe038691_2a3a_45a9_a208_7bed691919f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicApplicationPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCurrentViewPresentedOnHolographicDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub IsHolographicActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatedeventargs: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    IsHolographicActivation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IHolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, topcenterposition: ::winrt_foundation::Numerics::Vector3, normal: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, topcenterposition: ::winrt_foundation::Numerics::Vector3, normal: ::winrt_foundation::Numerics::Vector3, maxsize: ::winrt_foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverrideWithMaxSize: usize,
    #[cfg(feature = "deprecated")]
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetPlacementOverride: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    type Vtable = IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x202e6039_1ff6_5a06_aac4_a5e24fa3ec4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
