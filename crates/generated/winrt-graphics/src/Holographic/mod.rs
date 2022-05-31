#[repr(C)]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for HolographicAdapterId {}
impl ::core::clone::Clone for HolographicAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HolographicAdapterId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
unsafe impl ::windows_core::Abi for HolographicAdapterId {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for HolographicAdapterId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicAdapterId;u4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HolographicAdapterId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicAdapterId>()) == 0 }
    }
}
impl ::core::cmp::Eq for HolographicAdapterId {}
impl ::core::default::Default for HolographicAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct HolographicCamera(::windows_core::IUnknown);
impl HolographicCamera {
    #[cfg(feature = "Foundation")]
    pub fn RenderTargetSize(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).RenderTargetSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn ViewportScaleFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportScaleFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetViewportScaleFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewportScaleFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStereo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNearPlaneDistance(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNearPlaneDistance)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetFarPlaneDistance(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFarPlaneDistance)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftViewportParameters(&self) -> ::windows_core::Result<HolographicCameraViewportParameters> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LeftViewportParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    pub fn RightViewportParameters(&self) -> ::windows_core::Result<HolographicCameraViewportParameters> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RightViewportParameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    pub fn Display(&self) -> ::windows_core::Result<HolographicDisplay> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Display)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        }
    }
    pub fn IsPrimaryLayerEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPrimaryLayerEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPrimaryLayerEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxQuadLayerCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxQuadLayerCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn QuadLayers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QuadLayers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>(result__)
        }
    }
    pub fn CanOverrideViewport(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanOverrideViewport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHardwareContentProtectionSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHardwareContentProtectionSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHardwareContentProtectionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHardwareContentProtectionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHardwareContentProtectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewConfiguration(&self) -> ::windows_core::Result<HolographicViewConfiguration> {
        let this = &::windows_core::Interface::cast::<IHolographicCamera6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewConfiguration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicCamera {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCamera {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCamera {}
impl ::core::fmt::Debug for HolographicCamera {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCamera").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicCamera {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCamera;{e4e98445-9bed-4980-9ba0-e87680d1cb74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicCamera {
    type Vtable = IHolographicCamera_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicCamera as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCamera";
}
impl ::core::convert::From<HolographicCamera> for ::windows_core::IUnknown {
    fn from(value: HolographicCamera) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCamera> for ::windows_core::IUnknown {
    fn from(value: &HolographicCamera) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicCamera {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicCamera {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicCamera> for ::windows_core::IInspectable {
    fn from(value: HolographicCamera) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCamera> for ::windows_core::IInspectable {
    fn from(value: &HolographicCamera) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicCamera {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicCamera {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicCamera {}
unsafe impl ::core::marker::Sync for HolographicCamera {}
#[repr(transparent)]
pub struct HolographicCameraPose(::windows_core::IUnknown);
impl HolographicCameraPose {
    pub fn HolographicCamera(&self) -> ::windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HolographicCamera)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Viewport(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).Viewport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetViewTransform<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<super::super::Foundation::IReference<HolographicStereoTransform>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetViewTransform)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<HolographicStereoTransform>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectionTransform(&self) -> ::windows_core::Result<HolographicStereoTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicStereoTransform>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicStereoTransform>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetCullingFrustum<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetCullingFrustum)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetVisibleFrustum<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetVisibleFrustum)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    pub fn NearPlaneDistance(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NearPlaneDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn FarPlaneDistance(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FarPlaneDistance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn OverrideViewTransform<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, HolographicStereoTransform>>(&self, coordinatesystem: Param0, coordinatesystemtoviewtransform: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OverrideViewTransform)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), coordinatesystemtoviewtransform.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OverrideProjectionTransform<'a, Param0: ::windows_core::IntoParam<'a, HolographicStereoTransform>>(&self, projectiontransform: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OverrideProjectionTransform)(::windows_core::Interface::as_raw(this), projectiontransform.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OverrideViewport<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Rect>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Rect>>(&self, leftviewport: Param0, rightviewport: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OverrideViewport)(::windows_core::Interface::as_raw(this), leftviewport.into_param().abi(), rightviewport.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for HolographicCameraPose {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraPose {}
impl ::core::fmt::Debug for HolographicCameraPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraPose").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicCameraPose {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraPose;{0d7d7e30-12de-45bd-912b-c7f6561599d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicCameraPose {
    type Vtable = IHolographicCameraPose_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicCameraPose as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraPose";
}
impl ::core::convert::From<HolographicCameraPose> for ::windows_core::IUnknown {
    fn from(value: HolographicCameraPose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraPose> for ::windows_core::IUnknown {
    fn from(value: &HolographicCameraPose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicCameraPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicCameraPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicCameraPose> for ::windows_core::IInspectable {
    fn from(value: HolographicCameraPose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraPose> for ::windows_core::IInspectable {
    fn from(value: &HolographicCameraPose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicCameraPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicCameraPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicCameraPose {}
unsafe impl ::core::marker::Sync for HolographicCameraPose {}
#[repr(transparent)]
pub struct HolographicCameraRenderingParameters(::windows_core::IUnknown);
impl HolographicCameraRenderingParameters {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, position: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusPoint)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormal<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, position: Param1, normal: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusPointWithNormal)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi(), normal.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormalLinearVelocity<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param3: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, position: Param1, normal: Param2, linearvelocity: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusPointWithNormalLinearVelocity)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi(), normal.into_param().abi(), linearvelocity.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows_core::Result<super::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11BackBuffer(&self) -> ::windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11BackBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    pub fn ReprojectionMode(&self) -> ::windows_core::Result<HolographicReprojectionMode> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicReprojectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).ReprojectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicReprojectionMode>(result__)
        }
    }
    pub fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReprojectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CommitDirect3D11DepthBuffer<'a, Param0: ::windows_core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CommitDirect3D11DepthBuffer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsContentProtectionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentProtectionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsContentProtectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DepthReprojectionMethod(&self) -> ::windows_core::Result<HolographicDepthReprojectionMethod> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicDepthReprojectionMethod>::zeroed();
            (::windows_core::Interface::vtable(this).DepthReprojectionMethod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDepthReprojectionMethod>(result__)
        }
    }
    pub fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDepthReprojectionMethod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for HolographicCameraRenderingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraRenderingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraRenderingParameters {}
impl ::core::fmt::Debug for HolographicCameraRenderingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraRenderingParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicCameraRenderingParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraRenderingParameters;{8eac2ed1-5bf4-4e16-8236-ae0800c11d0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicCameraRenderingParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
}
impl ::core::convert::From<HolographicCameraRenderingParameters> for ::windows_core::IUnknown {
    fn from(value: HolographicCameraRenderingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraRenderingParameters> for ::windows_core::IUnknown {
    fn from(value: &HolographicCameraRenderingParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicCameraRenderingParameters> for ::windows_core::IInspectable {
    fn from(value: HolographicCameraRenderingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraRenderingParameters> for ::windows_core::IInspectable {
    fn from(value: &HolographicCameraRenderingParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicCameraRenderingParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraRenderingParameters {}
#[repr(transparent)]
pub struct HolographicCameraViewportParameters(::windows_core::IUnknown);
impl HolographicCameraViewportParameters {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HiddenAreaMesh(&self) -> ::windows_core::Result<::windows_core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<super::super::Foundation::Numerics::Vector2>>::zeroed();
            (::windows_core::Interface::vtable(this).HiddenAreaMesh)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VisibleAreaMesh(&self) -> ::windows_core::Result<::windows_core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<super::super::Foundation::Numerics::Vector2>>::zeroed();
            (::windows_core::Interface::vtable(this).VisibleAreaMesh)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for HolographicCameraViewportParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicCameraViewportParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicCameraViewportParameters {}
impl ::core::fmt::Debug for HolographicCameraViewportParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicCameraViewportParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicCameraViewportParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraViewportParameters;{80cdf3f7-842a-41e1-93ed-5692ab1fbb10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicCameraViewportParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraViewportParameters";
}
impl ::core::convert::From<HolographicCameraViewportParameters> for ::windows_core::IUnknown {
    fn from(value: HolographicCameraViewportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraViewportParameters> for ::windows_core::IUnknown {
    fn from(value: &HolographicCameraViewportParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicCameraViewportParameters> for ::windows_core::IInspectable {
    fn from(value: HolographicCameraViewportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicCameraViewportParameters> for ::windows_core::IInspectable {
    fn from(value: &HolographicCameraViewportParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicCameraViewportParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraViewportParameters {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicDepthReprojectionMethod {}
impl ::core::clone::Clone for HolographicDepthReprojectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicDepthReprojectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicDepthReprojectionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicDepthReprojectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicDepthReprojectionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicDepthReprojectionMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicDepthReprojectionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HolographicDisplay(::windows_core::IUnknown);
impl HolographicDisplay {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxViewportSize(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).MaxViewportSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn IsStereo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStereo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOpaque(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpaque)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AdapterId(&self) -> ::windows_core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicAdapterId>::zeroed();
            (::windows_core::Interface::vtable(this).AdapterId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn SpatialLocator(&self) -> ::windows_core::Result<super::super::Perception::Spatial::SpatialLocator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialLocator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Perception::Spatial::SpatialLocator>(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IHolographicDisplay2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RefreshRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> ::windows_core::Result<HolographicViewConfiguration> {
        let this = &::windows_core::Interface::cast::<IHolographicDisplay3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetViewConfiguration)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<HolographicDisplay> {
        Self::IHolographicDisplayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        })
    }
    pub fn IHolographicDisplayStatics<R, F: FnOnce(&IHolographicDisplayStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicDisplay, IHolographicDisplayStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicDisplay {}
impl ::core::fmt::Debug for HolographicDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicDisplay {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicDisplay;{9acea414-1d9f-4090-a388-90c06f6eae9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicDisplay {
    type Vtable = IHolographicDisplay_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicDisplay as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicDisplay";
}
impl ::core::convert::From<HolographicDisplay> for ::windows_core::IUnknown {
    fn from(value: HolographicDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicDisplay> for ::windows_core::IUnknown {
    fn from(value: &HolographicDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicDisplay> for ::windows_core::IInspectable {
    fn from(value: HolographicDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicDisplay> for ::windows_core::IInspectable {
    fn from(value: &HolographicDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicDisplay {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicDisplay {}
unsafe impl ::core::marker::Sync for HolographicDisplay {}
#[repr(transparent)]
pub struct HolographicFrame(::windows_core::IUnknown);
impl HolographicFrame {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedCameras(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddedCameras)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemovedCameras(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovedCameras)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    pub fn GetRenderingParameters<'a, Param0: ::windows_core::IntoParam<'a, HolographicCameraPose>>(&self, camerapose: Param0) -> ::windows_core::Result<HolographicCameraRenderingParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRenderingParameters)(::windows_core::Interface::as_raw(this), camerapose.into_param().abi(), result__.as_mut_ptr()).from_abi::<HolographicCameraRenderingParameters>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn CurrentPrediction(&self) -> ::windows_core::Result<HolographicFramePrediction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPrediction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFramePrediction>(result__)
        }
    }
    pub fn UpdateCurrentPrediction(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateCurrentPrediction)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PresentUsingCurrentPrediction(&self) -> ::windows_core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicFramePresentResult>::zeroed();
            (::windows_core::Interface::vtable(this).PresentUsingCurrentPrediction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    pub fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> ::windows_core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicFramePresentResult>::zeroed();
            (::windows_core::Interface::vtable(this).PresentUsingCurrentPredictionWithBehavior)(::windows_core::Interface::as_raw(this), waitbehavior, result__.as_mut_ptr()).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    pub fn WaitForFrameToFinish(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WaitForFrameToFinish)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetQuadLayerUpdateParameters<'a, Param0: ::windows_core::IntoParam<'a, HolographicQuadLayer>>(&self, layer: Param0) -> ::windows_core::Result<HolographicQuadLayerUpdateParameters> {
        let this = &::windows_core::Interface::cast::<IHolographicFrame2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetQuadLayerUpdateParameters)(::windows_core::Interface::as_raw(this), layer.into_param().abi(), result__.as_mut_ptr()).from_abi::<HolographicQuadLayerUpdateParameters>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<HolographicFrameId> {
        let this = &::windows_core::Interface::cast::<IHolographicFrame3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicFrameId>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameId>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrame {}
impl ::core::fmt::Debug for HolographicFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrame;{c6988eb6-a8b9-3054-a6eb-d624b6536375})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicFrame {
    type Vtable = IHolographicFrame_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrame";
}
impl ::core::convert::From<HolographicFrame> for ::windows_core::IUnknown {
    fn from(value: HolographicFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrame> for ::windows_core::IUnknown {
    fn from(value: &HolographicFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicFrame> for ::windows_core::IInspectable {
    fn from(value: HolographicFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrame> for ::windows_core::IInspectable {
    fn from(value: &HolographicFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicFrame {}
unsafe impl ::core::marker::Sync for HolographicFrame {}
#[repr(C)]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl ::core::marker::Copy for HolographicFrameId {}
impl ::core::clone::Clone for HolographicFrameId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HolographicFrameId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicFrameId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for HolographicFrameId {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for HolographicFrameId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicFrameId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HolographicFrameId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicFrameId>()) == 0 }
    }
}
impl ::core::cmp::Eq for HolographicFrameId {}
impl ::core::default::Default for HolographicFrameId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct HolographicFramePrediction(::windows_core::IUnknown);
impl HolographicFramePrediction {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CameraPoses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CameraPoses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFramePrediction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFramePrediction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFramePrediction {}
impl ::core::fmt::Debug for HolographicFramePrediction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePrediction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFramePrediction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePrediction;{520f4de1-5c0a-4e79-a81e-6abe02bb2739})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFramePrediction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePrediction";
}
impl ::core::convert::From<HolographicFramePrediction> for ::windows_core::IUnknown {
    fn from(value: HolographicFramePrediction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFramePrediction> for ::windows_core::IUnknown {
    fn from(value: &HolographicFramePrediction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFramePrediction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFramePrediction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicFramePrediction> for ::windows_core::IInspectable {
    fn from(value: HolographicFramePrediction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFramePrediction> for ::windows_core::IInspectable {
    fn from(value: &HolographicFramePrediction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFramePrediction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFramePrediction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicFramePrediction {}
unsafe impl ::core::marker::Sync for HolographicFramePrediction {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentResult {}
impl ::core::clone::Clone for HolographicFramePresentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicFramePresentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicFramePresentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicFramePresentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFramePresentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentWaitBehavior {}
impl ::core::clone::Clone for HolographicFramePresentWaitBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicFramePresentWaitBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicFramePresentWaitBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicFramePresentWaitBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentWaitBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFramePresentWaitBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicFramePresentationMonitor(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationMonitor {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ReadReports(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicFramePresentationMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicFramePresentationMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicFramePresentationMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentationMonitor").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for HolographicFramePresentationMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationMonitor;{ca87256c-6fae-428e-bb83-25dfee51136b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for HolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFramePresentationMonitor as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for HolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicFramePresentationMonitor> for ::windows_core::IUnknown {
    fn from(value: HolographicFramePresentationMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicFramePresentationMonitor> for ::windows_core::IUnknown {
    fn from(value: &HolographicFramePresentationMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicFramePresentationMonitor> for ::windows_core::IInspectable {
    fn from(value: HolographicFramePresentationMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicFramePresentationMonitor> for ::windows_core::IInspectable {
    fn from(value: &HolographicFramePresentationMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HolographicFramePresentationMonitor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HolographicFramePresentationMonitor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HolographicFramePresentationReport(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationReport {
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CompositorGpuDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).CompositorGpuDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AppGpuDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AppGpuDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AppGpuOverrun(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AppGpuOverrun)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn MissedPresentationOpportunityCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MissedPresentationOpportunityCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PresentationCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PresentationCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HolographicFramePresentationReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for HolographicFramePresentationReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for HolographicFramePresentationReport {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for HolographicFramePresentationReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFramePresentationReport").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for HolographicFramePresentationReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationReport;{80baf614-f2f4-4c8a-8de3-065c78f6d5de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for HolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFramePresentationReport as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for HolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationReport";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicFramePresentationReport> for ::windows_core::IUnknown {
    fn from(value: HolographicFramePresentationReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicFramePresentationReport> for ::windows_core::IUnknown {
    fn from(value: &HolographicFramePresentationReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFramePresentationReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFramePresentationReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<HolographicFramePresentationReport> for ::windows_core::IInspectable {
    fn from(value: HolographicFramePresentationReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&HolographicFramePresentationReport> for ::windows_core::IInspectable {
    fn from(value: &HolographicFramePresentationReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFramePresentationReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFramePresentationReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for HolographicFramePresentationReport {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for HolographicFramePresentationReport {}
#[repr(transparent)]
pub struct HolographicFrameRenderingReport(::windows_core::IUnknown);
impl HolographicFrameRenderingReport {
    pub fn FrameId(&self) -> ::windows_core::Result<HolographicFrameId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicFrameId>::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameId>(result__)
        }
    }
    pub fn MissedLatchCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MissedLatchCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeFrameReadyTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeFrameReadyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeActualGpuFinishTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeActualGpuFinishTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTargetLatchTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTargetLatchTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameRenderingReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameRenderingReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameRenderingReport {}
impl ::core::fmt::Debug for HolographicFrameRenderingReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameRenderingReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFrameRenderingReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameRenderingReport;{05f32de4-e384-51b3-b934-f0d3a0f78606})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFrameRenderingReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameRenderingReport";
}
impl ::core::convert::From<HolographicFrameRenderingReport> for ::windows_core::IUnknown {
    fn from(value: HolographicFrameRenderingReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameRenderingReport> for ::windows_core::IUnknown {
    fn from(value: &HolographicFrameRenderingReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicFrameRenderingReport> for ::windows_core::IInspectable {
    fn from(value: HolographicFrameRenderingReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameRenderingReport> for ::windows_core::IInspectable {
    fn from(value: &HolographicFrameRenderingReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicFrameRenderingReport {}
unsafe impl ::core::marker::Sync for HolographicFrameRenderingReport {}
#[repr(transparent)]
pub struct HolographicFrameScanoutMonitor(::windows_core::IUnknown);
impl HolographicFrameScanoutMonitor {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameScanoutMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameScanoutMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameScanoutMonitor {}
impl ::core::fmt::Debug for HolographicFrameScanoutMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameScanoutMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFrameScanoutMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutMonitor;{7e83efa9-843c-5401-8095-9bc1b8b08638})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFrameScanoutMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
}
impl ::core::convert::From<HolographicFrameScanoutMonitor> for ::windows_core::IUnknown {
    fn from(value: HolographicFrameScanoutMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameScanoutMonitor> for ::windows_core::IUnknown {
    fn from(value: &HolographicFrameScanoutMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicFrameScanoutMonitor> for ::windows_core::IInspectable {
    fn from(value: HolographicFrameScanoutMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameScanoutMonitor> for ::windows_core::IInspectable {
    fn from(value: &HolographicFrameScanoutMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HolographicFrameScanoutMonitor) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HolographicFrameScanoutMonitor) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HolographicFrameScanoutMonitor {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutMonitor {}
#[repr(transparent)]
pub struct HolographicFrameScanoutReport(::windows_core::IUnknown);
impl HolographicFrameScanoutReport {
    pub fn RenderingReport(&self) -> ::windows_core::Result<HolographicFrameRenderingReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenderingReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrameRenderingReport>(result__)
        }
    }
    pub fn MissedScanoutCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MissedScanoutCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeLatchTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeLatchTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeScanoutStartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeScanoutStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativePhotonTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativePhotonTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicFrameScanoutReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicFrameScanoutReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicFrameScanoutReport {}
impl ::core::fmt::Debug for HolographicFrameScanoutReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicFrameScanoutReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicFrameScanoutReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutReport;{0ebbe606-03a0-5ca0-b46e-bba068d7233f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicFrameScanoutReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutReport";
}
impl ::core::convert::From<HolographicFrameScanoutReport> for ::windows_core::IUnknown {
    fn from(value: HolographicFrameScanoutReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameScanoutReport> for ::windows_core::IUnknown {
    fn from(value: &HolographicFrameScanoutReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicFrameScanoutReport> for ::windows_core::IInspectable {
    fn from(value: HolographicFrameScanoutReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicFrameScanoutReport> for ::windows_core::IInspectable {
    fn from(value: &HolographicFrameScanoutReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicFrameScanoutReport {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutReport {}
#[repr(transparent)]
pub struct HolographicQuadLayer(::windows_core::IUnknown);
impl HolographicQuadLayer {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> ::windows_core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).PixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Size>>(size: Param0) -> ::windows_core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), size.into_param().abi(), result__.as_mut_ptr()).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub fn CreateWithPixelFormat<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Size>>(size: Param0, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows_core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPixelFormat)(::windows_core::Interface::as_raw(this), size.into_param().abi(), pixelformat, result__.as_mut_ptr()).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    pub fn IHolographicQuadLayerFactory<R, F: FnOnce(&IHolographicQuadLayerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicQuadLayer, IHolographicQuadLayerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicQuadLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicQuadLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicQuadLayer {}
impl ::core::fmt::Debug for HolographicQuadLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicQuadLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicQuadLayer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayer;{903460c9-c9d9-5d5c-41ac-a2d5ab0fd331})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicQuadLayer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayer";
}
impl ::core::convert::From<HolographicQuadLayer> for ::windows_core::IUnknown {
    fn from(value: HolographicQuadLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicQuadLayer> for ::windows_core::IUnknown {
    fn from(value: &HolographicQuadLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicQuadLayer> for ::windows_core::IInspectable {
    fn from(value: HolographicQuadLayer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicQuadLayer> for ::windows_core::IInspectable {
    fn from(value: &HolographicQuadLayer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HolographicQuadLayer) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HolographicQuadLayer) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicQuadLayer {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HolographicQuadLayer {}
unsafe impl ::core::marker::Sync for HolographicQuadLayer {}
#[repr(transparent)]
pub struct HolographicQuadLayerUpdateParameters(::windows_core::IUnknown);
impl HolographicQuadLayerUpdateParameters {
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContent(&self) -> ::windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireBufferToUpdateContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateViewport<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateViewport)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateContentProtectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateExtents<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateExtents)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UpdateLocationWithStationaryMode<'a, Param0: ::windows_core::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, position: Param1, orientation: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateLocationWithStationaryMode)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateLocationWithDisplayRelativeMode<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, position: Param0, orientation: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateLocationWithDisplayRelativeMode)(::windows_core::Interface::as_raw(this), position.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    pub fn CanAcquireWithHardwareProtection(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanAcquireWithHardwareProtection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows_core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireBufferToUpdateContentWithHardwareProtection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicQuadLayerUpdateParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicQuadLayerUpdateParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicQuadLayerUpdateParameters {}
impl ::core::fmt::Debug for HolographicQuadLayerUpdateParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicQuadLayerUpdateParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicQuadLayerUpdateParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters;{2b0ea3b0-798d-5bca-55c2-2c0c762ebb08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicQuadLayerUpdateParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
}
impl ::core::convert::From<HolographicQuadLayerUpdateParameters> for ::windows_core::IUnknown {
    fn from(value: HolographicQuadLayerUpdateParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicQuadLayerUpdateParameters> for ::windows_core::IUnknown {
    fn from(value: &HolographicQuadLayerUpdateParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicQuadLayerUpdateParameters> for ::windows_core::IInspectable {
    fn from(value: HolographicQuadLayerUpdateParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicQuadLayerUpdateParameters> for ::windows_core::IInspectable {
    fn from(value: &HolographicQuadLayerUpdateParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicQuadLayerUpdateParameters {}
unsafe impl ::core::marker::Sync for HolographicQuadLayerUpdateParameters {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicReprojectionMode {}
impl ::core::clone::Clone for HolographicReprojectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicReprojectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicReprojectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicReprojectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicReprojectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicReprojectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicReprojectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HolographicSpace(::windows_core::IUnknown);
impl HolographicSpace {
    pub fn PrimaryAdapterId(&self) -> ::windows_core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicAdapterId>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryAdapterId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn SetDirect3D11Device<'a, Param0: ::windows_core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDirect3D11Device)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CameraAdded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraAdded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraAdded)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CameraRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraRemoved)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn CreateNextFrame(&self) -> ::windows_core::Result<HolographicFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNextFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicFrame>(result__)
        }
    }
    pub fn UserPresence(&self) -> ::windows_core::Result<HolographicSpaceUserPresence> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicSpaceUserPresence>::zeroed();
            (::windows_core::Interface::vtable(this).UserPresence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicSpaceUserPresence>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UserPresenceChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserPresenceChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserPresenceChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserPresenceChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn WaitForNextFrameReady(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).WaitForNextFrameReady)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WaitForNextFrameReadyWithHeadStart<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, requestedheadstartduration: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).WaitForNextFrameReadyWithHeadStart)(::windows_core::Interface::as_raw(this), requestedheadstartduration.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows_core::Result<HolographicFramePresentationMonitor> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFramePresentationMonitor)(::windows_core::Interface::as_raw(this), maxqueuedreports, result__.as_mut_ptr()).from_abi::<HolographicFramePresentationMonitor>(result__)
        }
    }
    pub fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> ::windows_core::Result<HolographicFrameScanoutMonitor> {
        let this = &::windows_core::Interface::cast::<IHolographicSpace3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameScanoutMonitor)(::windows_core::Interface::as_raw(this), maxqueuedreports, result__.as_mut_ptr()).from_abi::<HolographicFrameScanoutMonitor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn CreateForCoreWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::UI::Core::CoreWindow>>(window: Param0) -> ::windows_core::Result<HolographicSpace> {
        Self::IHolographicSpaceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForCoreWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi(), result__.as_mut_ptr()).from_abi::<HolographicSpace>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsAvailable() -> ::windows_core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn IsAvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsAvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IHolographicSpaceStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveIsAvailableChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IsConfigured() -> ::windows_core::Result<bool> {
        Self::IHolographicSpaceStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConfigured)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IHolographicSpaceStatics<R, F: FnOnce(&IHolographicSpaceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicSpace, IHolographicSpaceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHolographicSpaceStatics2<R, F: FnOnce(&IHolographicSpaceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicSpace, IHolographicSpaceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHolographicSpaceStatics3<R, F: FnOnce(&IHolographicSpaceStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicSpace, IHolographicSpaceStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicSpace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpace {}
impl ::core::fmt::Debug for HolographicSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicSpace {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpace;{4380dba6-5e78-434f-807c-3433d1efe8b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicSpace {
    type Vtable = IHolographicSpace_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicSpace as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpace";
}
impl ::core::convert::From<HolographicSpace> for ::windows_core::IUnknown {
    fn from(value: HolographicSpace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpace> for ::windows_core::IUnknown {
    fn from(value: &HolographicSpace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicSpace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicSpace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicSpace> for ::windows_core::IInspectable {
    fn from(value: HolographicSpace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpace> for ::windows_core::IInspectable {
    fn from(value: &HolographicSpace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicSpace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicSpace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicSpace {}
unsafe impl ::core::marker::Sync for HolographicSpace {}
#[repr(transparent)]
pub struct HolographicSpaceCameraAddedEventArgs(::windows_core::IUnknown);
impl HolographicSpaceCameraAddedEventArgs {
    pub fn Camera(&self) -> ::windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Camera)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicSpaceCameraAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpaceCameraAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpaceCameraAddedEventArgs {}
impl ::core::fmt::Debug for HolographicSpaceCameraAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceCameraAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicSpaceCameraAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs;{58f1da35-bbb3-3c8f-993d-6c80e7feb99f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicSpaceCameraAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
}
impl ::core::convert::From<HolographicSpaceCameraAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HolographicSpaceCameraAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpaceCameraAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HolographicSpaceCameraAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicSpaceCameraAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HolographicSpaceCameraAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpaceCameraAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HolographicSpaceCameraAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicSpaceCameraAddedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraAddedEventArgs {}
#[repr(transparent)]
pub struct HolographicSpaceCameraRemovedEventArgs(::windows_core::IUnknown);
impl HolographicSpaceCameraRemovedEventArgs {
    pub fn Camera(&self) -> ::windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Camera)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicCamera>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicSpaceCameraRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicSpaceCameraRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicSpaceCameraRemovedEventArgs {}
impl ::core::fmt::Debug for HolographicSpaceCameraRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceCameraRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicSpaceCameraRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs;{805444a8-f2ae-322e-8da9-836a0a95a4c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicSpaceCameraRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicSpaceCameraRemovedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
}
impl ::core::convert::From<HolographicSpaceCameraRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HolographicSpaceCameraRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpaceCameraRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HolographicSpaceCameraRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicSpaceCameraRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HolographicSpaceCameraRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicSpaceCameraRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HolographicSpaceCameraRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicSpaceCameraRemovedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraRemovedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicSpaceUserPresence {}
impl ::core::clone::Clone for HolographicSpaceUserPresence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicSpaceUserPresence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicSpaceUserPresence {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicSpaceUserPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicSpaceUserPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicSpaceUserPresence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicSpaceUserPresence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HolographicStereoTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for HolographicStereoTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HolographicStereoTransform").field("Left", &self.Left).field("Right", &self.Right).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for HolographicStereoTransform {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for HolographicStereoTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicStereoTransform;struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for HolographicStereoTransform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HolographicStereoTransform>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for HolographicStereoTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct HolographicViewConfiguration(::windows_core::IUnknown);
impl HolographicViewConfiguration {
    #[cfg(feature = "Foundation")]
    pub fn NativeRenderTargetSize(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).NativeRenderTargetSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RenderTargetSize(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).RenderTargetSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestRenderTargetSize<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Size>>(&self, size: Param0) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRenderTargetSize)(::windows_core::Interface::as_raw(this), size.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedPixelFormats(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPixelFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> ::windows_core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).PixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPixelFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStereo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RefreshRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<HolographicViewConfigurationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HolographicViewConfigurationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicViewConfigurationKind>(result__)
        }
    }
    pub fn Display(&self) -> ::windows_core::Result<HolographicDisplay> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Display)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicDisplay>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDepthReprojectionMethods(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>> {
        let this = &::windows_core::Interface::cast::<IHolographicViewConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDepthReprojectionMethods)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>(result__)
        }
    }
}
impl ::core::clone::Clone for HolographicViewConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicViewConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicViewConfiguration {}
impl ::core::fmt::Debug for HolographicViewConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicViewConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicViewConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicViewConfiguration;{5c1de6e6-67e9-5004-b02c-67a3a122b576})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicViewConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicViewConfiguration";
}
impl ::core::convert::From<HolographicViewConfiguration> for ::windows_core::IUnknown {
    fn from(value: HolographicViewConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicViewConfiguration> for ::windows_core::IUnknown {
    fn from(value: &HolographicViewConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicViewConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicViewConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicViewConfiguration> for ::windows_core::IInspectable {
    fn from(value: HolographicViewConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicViewConfiguration> for ::windows_core::IInspectable {
    fn from(value: &HolographicViewConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicViewConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicViewConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicViewConfiguration {}
unsafe impl ::core::marker::Sync for HolographicViewConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicViewConfigurationKind {}
impl ::core::clone::Clone for HolographicViewConfigurationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HolographicViewConfigurationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HolographicViewConfigurationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for HolographicViewConfigurationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicViewConfigurationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicViewConfigurationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicViewConfigurationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera {
    type Vtable = IHolographicCamera_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4e98445_9bed_4980_9ba0_e87680d1cb74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    pub ViewportScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetViewportScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetNearPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SetFarPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera2 {
    type Vtable = IHolographicCamera2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb55b9f1a_ba8c_4f84_ad79_2e7e1e2450f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LeftViewportParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RightViewportParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera3 {
    type Vtable = IHolographicCamera3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45aa4fb3_7b59_524e_4a3f_4a6ad6650477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPrimaryLayerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MaxQuadLayerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub QuadLayers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QuadLayers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera4 {
    type Vtable = IHolographicCamera4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a2531d6_4723_4f39_a9a5_9d05181d9b44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanOverrideViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera5 {
    type Vtable = IHolographicCamera5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x229706f2_628d_4ef5_9c08_a63fdd7787c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsHardwareContentProtectionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHardwareContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCamera6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCamera6 {
    type Vtable = IHolographicCamera6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0209194f_632d_5154_ab52_0b5d15b12505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ViewConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraPose(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraPose {
    type Vtable = IHolographicCameraPose_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d7d7e30_12de_45bd_912b_c7f6561599d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HolographicCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Viewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Viewport: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicStereoTransform) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectionTransform: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetCullingFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetCullingFrustum: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetVisibleFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetVisibleFrustum: usize,
    pub NearPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub FarPlaneDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraPose2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraPose2 {
    type Vtable = IHolographicCameraPose2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x232be073_5d2d_4560_814e_2697c4fce16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub OverrideViewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    OverrideViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub OverrideProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectiontransform: HolographicStereoTransform) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OverrideProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub OverrideViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverrideViewport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eac2ed1_5bf4_4e16_8236_ae0800c11d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormal: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormalLinearVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormalLinearVelocity: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11BackBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11BackBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParameters2 {
    type Vtable = IHolographicCameraRenderingParameters2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x261270e3_b696_4634_94d6_be0681643599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReprojectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicReprojectionMode) -> ::windows_core::HRESULT,
    pub SetReprojectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HolographicReprojectionMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CommitDirect3D11DepthBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CommitDirect3D11DepthBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParameters3 {
    type Vtable = IHolographicCameraRenderingParameters3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1aa513f_136d_4b06_b9d4_e4b914cd0683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParameters4 {
    type Vtable = IHolographicCameraRenderingParameters4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0878fa4c_e163_57dc_82b7_c406ab3e0537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DepthReprojectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicDepthReprojectionMethod) -> ::windows_core::HRESULT,
    pub SetDepthReprojectionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HolographicDepthReprojectionMethod) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicCameraViewportParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80cdf3f7_842a_41e1_93ed_5692ab1fbb10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraViewportParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub HiddenAreaMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HiddenAreaMesh: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub VisibleAreaMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VisibleAreaMesh: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicDisplay {
    type Vtable = IHolographicDisplay_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9acea414_1d9f_4090_a388_90c06f6eae9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxViewportSize: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsOpaque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub SpatialLocator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    SpatialLocator: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicDisplay2 {
    type Vtable = IHolographicDisplay2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75ac3f82_e755_436c_8d96_4d32d131473e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplay3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicDisplay3 {
    type Vtable = IHolographicDisplay3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc4c6ac6_6480_5008_b29e_157d77c843f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetViewConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: HolographicViewConfigurationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicDisplayStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicDisplayStatics {
    type Vtable = IHolographicDisplayStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb374983_e7b0_4841_8355_3ae5b536e9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplayStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrame {
    type Vtable = IHolographicFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6988eb6_a8b9_3054_a6eb_d624b6536375);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedCameras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedCameras: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedCameras: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedCameras: usize,
    pub GetRenderingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, camerapose: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub CurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateCurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PresentUsingCurrentPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFramePresentResult) -> ::windows_core::HRESULT,
    pub PresentUsingCurrentPredictionWithBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows_core::HRESULT,
    pub WaitForFrameToFinish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrame2 {
    type Vtable = IHolographicFrame2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x283f37bf_3bf2_5e91_6633_870574e6f217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetQuadLayerUpdateParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrame3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrame3 {
    type Vtable = IHolographicFrame3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5e964c9_8a27_55d3_9f98_94530d369052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFramePrediction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520f4de1_5c0a_4e79_a81e_6abe02bb2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePrediction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CameraPoses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CameraPoses: usize,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicFramePresentationMonitor(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IHolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca87256c_6fae_428e_bb83_25dfee51136b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ReadReports: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IHolographicFramePresentationReport(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IHolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80baf614_f2f4_4c8a_8de3_065c78f6d5de);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CompositorGpuDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CompositorGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AppGpuOverrun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AppGpuOverrun: usize,
    #[cfg(feature = "deprecated")]
    pub MissedPresentationOpportunityCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MissedPresentationOpportunityCount: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameRenderingReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05f32de4_e384_51b3_b934_f0d3a0f78606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameRenderingReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows_core::HRESULT,
    pub MissedLatchCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeFrameReadyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeFrameReadyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeActualGpuFinishTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeActualGpuFinishTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetLatchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetLatchTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameScanoutMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e83efa9_843c_5401_8095_9bc1b8b08638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicFrameScanoutReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ebbe606_03a0_5ca0_b46e_bba068d7233f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RenderingReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MissedScanoutCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeLatchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeLatchTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeScanoutStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeScanoutStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativePhotonTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativePhotonTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x903460c9_c9d9_5d5c_41ac_a2d5ab0fd331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicQuadLayerFactory {
    type Vtable = IHolographicQuadLayerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa67538f3_5a14_5a10_489a_455065b37b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub CreateWithPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    CreateWithPixelFormat: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b0ea3b0_798d_5bca_55c2_2c0c762ebb08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContent: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateViewport: usize,
    pub UpdateContentProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateExtents: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UpdateLocationWithStationaryMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UpdateLocationWithStationaryMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateLocationWithDisplayRelativeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateLocationWithDisplayRelativeMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicQuadLayerUpdateParameters2 {
    type Vtable = IHolographicQuadLayerUpdateParameters2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f33d32d_82c1_46c1_8980_3cb70d98182b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanAcquireWithHardwareProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContentWithHardwareProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContentWithHardwareProtection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpace {
    type Vtable = IHolographicSpace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4380dba6_5e78_434f_807c_3433d1efe8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrimaryAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub SetDirect3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    SetDirect3D11Device: usize,
    #[cfg(feature = "Foundation")]
    pub CameraAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraAdded: usize,
    #[cfg(feature = "Foundation")]
    pub CameraRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraRemoved: usize,
    pub CreateNextFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpace2 {
    type Vtable = IHolographicSpace2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f81a9a8_b7ff_4883_9827_7d677287ea70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserPresence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicSpaceUserPresence) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserPresenceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserPresenceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserPresenceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserPresenceChanged: usize,
    pub WaitForNextFrameReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WaitForNextFrameReadyWithHeadStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForNextFrameReadyWithHeadStart: usize,
    #[cfg(feature = "deprecated")]
    pub CreateFramePresentationMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFramePresentationMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpace3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpace3 {
    type Vtable = IHolographicSpace3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf1733d1_f224_587e_8d71_1e8fc8f07b1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFrameScanoutMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceCameraAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58f1da35_bbb3_3c8f_993d_6c80e7feb99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceCameraRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x805444a8_f2ae_322e_8da9_836a0a95a4c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Camera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpaceStatics {
    type Vtable = IHolographicSpaceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x364e6064_c8f2_3ba1_8391_66b8489e67fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpaceStatics2 {
    type Vtable = IHolographicSpaceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e777088_75fc_48af_8758_0652f6f07c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsAvailableChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicSpaceStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicSpaceStatics3 {
    type Vtable = IHolographicSpaceStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b00de3d_b1a3_4dfe_8e79_fec5909e6df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsConfigured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicViewConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c1de6e6_67e9_5004_b02c_67a3a122b576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub NativeRenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NativeRenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenderTargetSize: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRenderTargetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRenderTargetSize: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedPixelFormats: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HolographicViewConfigurationKind) -> ::windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicViewConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicViewConfiguration2 {
    type Vtable = IHolographicViewConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe241756e_e0d0_5019_9af5_1b165bc2f54e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDepthReprojectionMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDepthReprojectionMethods: usize,
}