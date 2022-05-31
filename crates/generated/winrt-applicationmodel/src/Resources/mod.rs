#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Management")]
pub mod Management;
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08524908_16ef_45ad_a602_293637d7e61a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoader2 {
    type Vtable = IResourceLoader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10eb6ec6_8138_48c1_bc65_e1f14207367c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetStringForUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc33a3603_69dc_4285_a077_d5c0e47ccbe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateResourceLoaderByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf777ce1_19c8_49c2_953c_47e9227b334e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetStringForReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStringForReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics2 {
    type Vtable = IResourceLoaderStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc04141_6466_4989_9494_0b82dfc53f1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForCurrentViewWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForViewIndependentUseWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics3 {
    type Vtable = IResourceLoaderStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64609dfb_64ac_491b_8100_0e558d61c1d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics4 {
    type Vtable = IResourceLoaderStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fb36c32_6c8c_4316_962e_909539b5c259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultPriPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ResourceLoader(::windows_core::IUnknown);
impl ResourceLoader {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resource: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetStringForUri<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IResourceLoader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetStringForUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateResourceLoaderByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceLoaderByName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetStringForReference<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetStringForReference)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForCurrentViewWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentViewWithName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForViewIndependentUse() -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForViewIndependentUseWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUseWithName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::UIContext>>(context: Param0) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetDefaultPriPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefullname: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IResourceLoaderStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultPriPath)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IResourceLoaderFactory<R, F: FnOnce(&IResourceLoaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, IResourceLoaderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics<R, F: FnOnce(&IResourceLoaderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, IResourceLoaderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics2<R, F: FnOnce(&IResourceLoaderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, IResourceLoaderStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics3<R, F: FnOnce(&IResourceLoaderStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, IResourceLoaderStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics4<R, F: FnOnce(&IResourceLoaderStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ResourceLoader, IResourceLoaderStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceLoader {}
impl ::core::fmt::Debug for ResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceLoader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResourceLoader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.ResourceLoader;{08524908-16ef-45ad-a602-293637d7e61a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
    const IID: ::windows_core::GUID = <IResourceLoader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::core::convert::From<ResourceLoader> for ::windows_core::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows_core::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ResourceLoader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceLoader> for ::windows_core::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows_core::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ResourceLoader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}