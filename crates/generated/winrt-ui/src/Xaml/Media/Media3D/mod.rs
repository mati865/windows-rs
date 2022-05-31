#[repr(transparent)]
pub struct CompositeTransform3D(::windows_core::IUnknown);
impl CompositeTransform3D {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositeTransform3D, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetCenterZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RotationZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetScaleZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslateX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslateX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslateY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslateY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslateZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetTranslateZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslateZ)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CenterZProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CenterZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ScaleZProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TranslateZProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateZProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ICompositeTransform3DStatics<R, F: FnOnce(&ICompositeTransform3DStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositeTransform3D, ICompositeTransform3DStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositeTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositeTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeTransform3D {}
impl ::core::fmt::Debug for CompositeTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositeTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositeTransform3D {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.CompositeTransform3D;{8977cb01-af8d-4af5-b084-c08eb9704abe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositeTransform3D {
    type Vtable = ICompositeTransform3D_Vtbl;
    const IID: ::windows_core::GUID = <ICompositeTransform3D as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositeTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.CompositeTransform3D";
}
impl ::core::convert::From<CompositeTransform3D> for ::windows_core::IUnknown {
    fn from(value: CompositeTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows_core::IUnknown {
    fn from(value: &CompositeTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform3D> for ::windows_core::IInspectable {
    fn from(value: CompositeTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows_core::IInspectable {
    fn from(value: &CompositeTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositeTransform3D> for Transform3D {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for Transform3D {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform3D> for CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, Transform3D> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform3D> for &CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, Transform3D> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl ::core::convert::From<CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CompositeTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositeTransform3D {}
unsafe impl ::core::marker::Sync for CompositeTransform3D {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform3D(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransform3D {
    type Vtable = ICompositeTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8977cb01_af8d_4af5_b084_c08eb9704abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3D_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub CenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub TranslateZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetTranslateZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeTransform3DStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransform3DStatics {
    type Vtable = ICompositeTransform3DStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddbf4d67_2a25_48f3_9808_c51ec3d55bec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3DStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CenterZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslateZProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DHelper {
    type Vtable = IMatrix3DHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe48c10ef_9927_4c9b_8213_07775512ba04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMatrix3DHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DHelperStatics {
    type Vtable = IMatrix3DHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9264545e_e158_4e74_8899_689160bd2f8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Matrix3D) -> ::windows_core::HRESULT,
    pub Multiply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix1: Matrix3D, matrix2: Matrix3D, result__: *mut Matrix3D) -> ::windows_core::HRESULT,
    pub FromElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64, result__: *mut Matrix3D) -> ::windows_core::HRESULT,
    pub GetHasInverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Invert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: Matrix3D, result__: *mut Matrix3D) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerspectiveTransform3D(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a7b532a_30f9_40a1_96c9_c59d87f95ac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3D_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Depth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPerspectiveTransform3DStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerspectiveTransform3DStatics {
    type Vtable = IPerspectiveTransform3DStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e6f6400_620c_48c7_844d_3f0984da5b17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DepthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OffsetXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OffsetYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform3D(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransform3D {
    type Vtable = ITransform3D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae3ed43a_a9fc_4c31_86cd_56d9ca251a69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3D_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransform3DFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransform3DFactory {
    type Vtable = ITransform3DFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x052c1f7a_8d73_48cd_bbb8_d00434caae5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3DFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct Matrix3D {
    pub M11: f64,
    pub M12: f64,
    pub M13: f64,
    pub M14: f64,
    pub M21: f64,
    pub M22: f64,
    pub M23: f64,
    pub M24: f64,
    pub M31: f64,
    pub M32: f64,
    pub M33: f64,
    pub M34: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
    pub OffsetZ: f64,
    pub M44: f64,
}
impl ::core::marker::Copy for Matrix3D {}
impl ::core::clone::Clone for Matrix3D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Matrix3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix3D").field("M11", &self.M11).field("M12", &self.M12).field("M13", &self.M13).field("M14", &self.M14).field("M21", &self.M21).field("M22", &self.M22).field("M23", &self.M23).field("M24", &self.M24).field("M31", &self.M31).field("M32", &self.M32).field("M33", &self.M33).field("M34", &self.M34).field("OffsetX", &self.OffsetX).field("OffsetY", &self.OffsetY).field("OffsetZ", &self.OffsetZ).field("M44", &self.M44).finish()
    }
}
unsafe impl ::windows_core::Abi for Matrix3D {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for Matrix3D {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Media3D.Matrix3D;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Matrix3D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Matrix3D>()) == 0 }
    }
}
impl ::core::cmp::Eq for Matrix3D {}
impl ::core::default::Default for Matrix3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct Matrix3DHelper(::windows_core::IUnknown);
impl Matrix3DHelper {
    pub fn Identity() -> ::windows_core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn Multiply<'a, Param0: ::windows_core::IntoParam<'a, Matrix3D>, Param1: ::windows_core::IntoParam<'a, Matrix3D>>(matrix1: Param0, matrix2: Param1) -> ::windows_core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).Multiply)(::windows_core::Interface::as_raw(this), matrix1.into_param().abi(), matrix2.into_param().abi(), result__.as_mut_ptr()).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn FromElements(m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64) -> ::windows_core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).FromElements)(::windows_core::Interface::as_raw(this), m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, offsetx, offsety, offsetz, m44, result__.as_mut_ptr()).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn GetHasInverse<'a, Param0: ::windows_core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows_core::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetHasInverse)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetIsIdentity<'a, Param0: ::windows_core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows_core::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsIdentity)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn Invert<'a, Param0: ::windows_core::IntoParam<'a, Matrix3D>>(target: Param0) -> ::windows_core::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Matrix3D>::zeroed();
            (::windows_core::Interface::vtable(this).Invert)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<Matrix3D>(result__)
        })
    }
    pub fn IMatrix3DHelperStatics<R, F: FnOnce(&IMatrix3DHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Matrix3DHelper, IMatrix3DHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Matrix3DHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Matrix3DHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Matrix3DHelper {}
impl ::core::fmt::Debug for Matrix3DHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Matrix3DHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Matrix3DHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Matrix3DHelper;{e48c10ef-9927-4c9b-8213-07775512ba04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Matrix3DHelper {
    type Vtable = IMatrix3DHelper_Vtbl;
    const IID: ::windows_core::GUID = <IMatrix3DHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Matrix3DHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Matrix3DHelper";
}
impl ::core::convert::From<Matrix3DHelper> for ::windows_core::IUnknown {
    fn from(value: Matrix3DHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows_core::IUnknown {
    fn from(value: &Matrix3DHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Matrix3DHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Matrix3DHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Matrix3DHelper> for ::windows_core::IInspectable {
    fn from(value: Matrix3DHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows_core::IInspectable {
    fn from(value: &Matrix3DHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Matrix3DHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Matrix3DHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Matrix3DHelper {}
unsafe impl ::core::marker::Sync for Matrix3DHelper {}
#[repr(transparent)]
pub struct PerspectiveTransform3D(::windows_core::IUnknown);
impl PerspectiveTransform3D {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerspectiveTransform3D, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Depth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Depth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDepth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDepth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OffsetX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffsetX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OffsetY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffsetY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DepthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DepthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetXProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetXProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetYProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetYProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPerspectiveTransform3DStatics<R, F: FnOnce(&IPerspectiveTransform3DStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerspectiveTransform3D, IPerspectiveTransform3DStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PerspectiveTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PerspectiveTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PerspectiveTransform3D {}
impl ::core::fmt::Debug for PerspectiveTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerspectiveTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PerspectiveTransform3D {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D;{9a7b532a-30f9-40a1-96c9-c59d87f95ac3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_Vtbl;
    const IID: ::windows_core::GUID = <IPerspectiveTransform3D as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PerspectiveTransform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.PerspectiveTransform3D";
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows_core::IUnknown {
    fn from(value: PerspectiveTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows_core::IUnknown {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows_core::IInspectable {
    fn from(value: PerspectiveTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows_core::IInspectable {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for Transform3D {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for Transform3D {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform3D> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, Transform3D> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transform3D> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, Transform3D> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PerspectiveTransform3D {}
unsafe impl ::core::marker::Sync for PerspectiveTransform3D {}
#[repr(transparent)]
pub struct Transform3D(::windows_core::IUnknown);
impl Transform3D {}
impl ::core::clone::Clone for Transform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Transform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transform3D {}
impl ::core::fmt::Debug for Transform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Transform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Transform3D {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Media3D.Transform3D;{ae3ed43a-a9fc-4c31-86cd-56d9ca251a69})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Transform3D {
    type Vtable = ITransform3D_Vtbl;
    const IID: ::windows_core::GUID = <ITransform3D as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Transform3D {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Media3D.Transform3D";
}
impl ::core::convert::From<Transform3D> for ::windows_core::IUnknown {
    fn from(value: Transform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform3D> for ::windows_core::IUnknown {
    fn from(value: &Transform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform3D> for ::windows_core::IInspectable {
    fn from(value: Transform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transform3D> for ::windows_core::IInspectable {
    fn from(value: &Transform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transform3D> for super::super::DependencyObject {
    fn from(value: Transform3D) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transform3D> for super::super::DependencyObject {
    fn from(value: &Transform3D) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Transform3D {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Transform3D {}
unsafe impl ::core::marker::Sync for Transform3D {}