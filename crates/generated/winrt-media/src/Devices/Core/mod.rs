#[repr(transparent)]
pub struct CameraIntrinsics(::windows_core::IUnknown);
impl CameraIntrinsics {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FocalLength(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).FocalLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PrincipalPoint(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).PrincipalPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RadialDistortion(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).RadialDistortion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TangentialDistortion(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).TangentialDistortion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn ImageWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ImageWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ImageHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ImageHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectOntoFrame<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, coordinate: Param0) -> ::windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectOntoFrame)(::windows_core::Interface::as_raw(this), coordinate.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectAtUnitDepth<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, pixelcoordinate: Param0) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).UnprojectAtUnitDepth)(::windows_core::Interface::as_raw(this), pixelcoordinate.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectManyOntoFrame(&self, coordinates: &[super::super::super::Foundation::Numerics::Vector3], results: &mut [super::super::super::Foundation::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProjectManyOntoFrame)(::windows_core::Interface::as_raw(this), coordinates.len() as u32, ::core::mem::transmute(coordinates.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Numerics::Vector2]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnprojectPixelsAtUnitDepth)(::windows_core::Interface::as_raw(this), pixelcoordinates.len() as u32, ::core::mem::transmute(pixelcoordinates.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UndistortedProjectionTransform(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &::windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Matrix4x4>::zeroed();
            (::windows_core::Interface::vtable(this).UndistortedProjectionTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DistortPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, input: Param0) -> ::windows_core::Result<super::super::super::Foundation::Point> {
        let this = &::windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).DistortPoint)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).DistortPoints)(::windows_core::Interface::as_raw(this), inputs.len() as u32, ::core::mem::transmute(inputs.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, input: Param0) -> ::windows_core::Result<super::super::super::Foundation::Point> {
        let this = &::windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).UndistortPoint)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UndistortPoints)(::windows_core::Interface::as_raw(this), inputs.len() as u32, ::core::mem::transmute(inputs.as_ptr()), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param3: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(focallength: Param0, principalpoint: Param1, radialdistortion: Param2, tangentialdistortion: Param3, imagewidth: u32, imageheight: u32) -> ::windows_core::Result<CameraIntrinsics> {
        Self::ICameraIntrinsicsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), focallength.into_param().abi(), principalpoint.into_param().abi(), radialdistortion.into_param().abi(), tangentialdistortion.into_param().abi(), imagewidth, imageheight, result__.as_mut_ptr()).from_abi::<CameraIntrinsics>(result__)
        })
    }
    pub fn ICameraIntrinsicsFactory<R, F: FnOnce(&ICameraIntrinsicsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CameraIntrinsics, ICameraIntrinsicsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CameraIntrinsics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraIntrinsics {}
impl ::core::fmt::Debug for CameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraIntrinsics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraIntrinsics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.CameraIntrinsics;{0aa6ed32-6589-49da-afde-594270ca0aac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CameraIntrinsics {
    type Vtable = ICameraIntrinsics_Vtbl;
    const IID: ::windows_core::GUID = <ICameraIntrinsics as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CameraIntrinsics {
    const NAME: &'static str = "Windows.Media.Devices.Core.CameraIntrinsics";
}
impl ::core::convert::From<CameraIntrinsics> for ::windows_core::IUnknown {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows_core::IUnknown {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraIntrinsics> for ::windows_core::IInspectable {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows_core::IInspectable {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraIntrinsics {}
unsafe impl ::core::marker::Sync for CameraIntrinsics {}
#[repr(transparent)]
pub struct DepthCorrelatedCoordinateMapper(::windows_core::IUnknown);
impl DepthCorrelatedCoordinateMapper {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, sourcepoint: Param0, targetcoordinatesystem: Param1) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).UnprojectPoint)(::windows_core::Interface::as_raw(this), sourcepoint.into_param().abi(), targetcoordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoints<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: Param1, results: &mut [super::super::super::Foundation::Numerics::Vector3]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnprojectPoints)(::windows_core::Interface::as_raw(this), sourcepoints.len() as u32, ::core::mem::transmute(sourcepoints.as_ptr()), targetcoordinatesystem.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoint<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param2: ::windows_core::IntoParam<'a, CameraIntrinsics>>(&self, sourcepoint: Param0, targetcoordinatesystem: Param1, targetcameraintrinsics: Param2) -> ::windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).MapPoint)(::windows_core::Interface::as_raw(this), sourcepoint.into_param().abi(), targetcoordinatesystem.into_param().abi(), targetcameraintrinsics.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoints<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param2: ::windows_core::IntoParam<'a, CameraIntrinsics>>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: Param1, targetcameraintrinsics: Param2, results: &mut [super::super::super::Foundation::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MapPoints)(::windows_core::Interface::as_raw(this), sourcepoints.len() as u32, ::core::mem::transmute(sourcepoints.as_ptr()), targetcoordinatesystem.into_param().abi(), targetcameraintrinsics.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
}
impl ::core::clone::Clone for DepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthCorrelatedCoordinateMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthCorrelatedCoordinateMapper {}
impl ::core::fmt::Debug for DepthCorrelatedCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DepthCorrelatedCoordinateMapper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DepthCorrelatedCoordinateMapper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper;{f95d89fb-8af0-4cb0-926d-696866e5046a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows_core::GUID = <IDepthCorrelatedCoordinateMapper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows_core::IUnknown {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows_core::IUnknown {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows_core::IInspectable {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows_core::IInspectable {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DepthCorrelatedCoordinateMapper) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DepthCorrelatedCoordinateMapper) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DepthCorrelatedCoordinateMapper {}
unsafe impl ::core::marker::Sync for DepthCorrelatedCoordinateMapper {}
#[repr(transparent)]
pub struct FrameControlCapabilities(::windows_core::IUnknown);
impl FrameControlCapabilities {
    pub fn Exposure(&self) -> ::windows_core::Result<FrameExposureCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCapabilities>(result__)
        }
    }
    pub fn ExposureCompensation(&self) -> ::windows_core::Result<FrameExposureCompensationCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCompensationCapabilities>(result__)
        }
    }
    pub fn IsoSpeed(&self) -> ::windows_core::Result<FrameIsoSpeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsoSpeed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameIsoSpeedCapabilities>(result__)
        }
    }
    pub fn Focus(&self) -> ::windows_core::Result<FrameFocusCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFocusCapabilities>(result__)
        }
    }
    pub fn PhotoConfirmationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoConfirmationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Flash(&self) -> ::windows_core::Result<FrameFlashCapabilities> {
        let this = &::windows_core::Interface::cast::<IFrameControlCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Flash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameControlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameControlCapabilities {}
impl ::core::fmt::Debug for FrameControlCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameControlCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameControlCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameControlCapabilities;{a8ffae60-4e9e-4377-a789-e24c4ae7e544})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameControlCapabilities {
    type Vtable = IFrameControlCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameControlCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameControlCapabilities";
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameControlCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameControlCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameControlCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameControlCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameController(::windows_core::IUnknown);
impl FrameController {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FrameController, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ExposureControl(&self) -> ::windows_core::Result<FrameExposureControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureControl>(result__)
        }
    }
    pub fn ExposureCompensationControl(&self) -> ::windows_core::Result<FrameExposureCompensationControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensationControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCompensationControl>(result__)
        }
    }
    pub fn IsoSpeedControl(&self) -> ::windows_core::Result<FrameIsoSpeedControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsoSpeedControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameIsoSpeedControl>(result__)
        }
    }
    pub fn FocusControl(&self) -> ::windows_core::Result<FrameFocusControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFocusControl>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationEnabled(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhotoConfirmationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPhotoConfirmationEnabled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotoConfirmationEnabled)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FlashControl(&self) -> ::windows_core::Result<FrameFlashControl> {
        let this = &::windows_core::Interface::cast::<IFrameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlashControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashControl>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameController {}
impl ::core::fmt::Debug for FrameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameController;{c16459d9-baef-4052-9177-48aff2af7522})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameController {
    type Vtable = IFrameController_Vtbl;
    const IID: ::windows_core::GUID = <IFrameController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameController {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameController";
}
impl ::core::convert::From<FrameController> for ::windows_core::IUnknown {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows_core::IUnknown {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameController> for ::windows_core::IInspectable {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows_core::IInspectable {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FrameController {}
unsafe impl ::core::marker::Sync for FrameController {}
#[repr(transparent)]
pub struct FrameExposureCapabilities(::windows_core::IUnknown);
impl FrameExposureCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCapabilities {}
impl ::core::fmt::Debug for FrameExposureCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameExposureCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCapabilities;{bdbe9ce3-3985-4e72-97c2-0590d61307a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameExposureCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameExposureCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCapabilities";
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameExposureCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameExposureCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameExposureCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameExposureCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameExposureCompensationCapabilities(::windows_core::IUnknown);
impl FrameExposureCompensationCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationCapabilities {}
impl ::core::fmt::Debug for FrameExposureCompensationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCompensationCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameExposureCompensationCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationCapabilities;{b988a823-8065-41ee-b04f-722265954500})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameExposureCompensationCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameExposureCompensationCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameExposureCompensationCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameExposureCompensationControl(::windows_core::IUnknown);
impl FrameExposureCompensationControl {
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IReference<f32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationControl {}
impl ::core::fmt::Debug for FrameExposureCompensationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCompensationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameExposureCompensationControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationControl;{e95896c9-f7f9-48ca-8591-a26531cb1578})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControl_Vtbl;
    const IID: ::windows_core::GUID = <IFrameExposureCompensationControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationControl";
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows_core::IUnknown {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows_core::IUnknown {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameExposureCompensationControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameExposureCompensationControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows_core::IInspectable {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows_core::IInspectable {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameExposureCompensationControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameExposureCompensationControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameExposureControl(::windows_core::IUnknown);
impl FrameExposureControl {
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuto)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureControl {}
impl ::core::fmt::Debug for FrameExposureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameExposureControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureControl;{b1605a61-ffaf-4752-b621-f5b6f117f432})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameExposureControl {
    type Vtable = IFrameExposureControl_Vtbl;
    const IID: ::windows_core::GUID = <IFrameExposureControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureControl";
}
impl ::core::convert::From<FrameExposureControl> for ::windows_core::IUnknown {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows_core::IUnknown {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameExposureControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameExposureControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameExposureControl> for ::windows_core::IInspectable {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows_core::IInspectable {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameExposureControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameExposureControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameFlashCapabilities(::windows_core::IUnknown);
impl FrameFlashCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RedEyeReductionSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PowerSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFlashCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashCapabilities {}
impl ::core::fmt::Debug for FrameFlashCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameFlashCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashCapabilities;{bb9341a2-5ebe-4f62-8223-0e2b05bfbbd0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameFlashCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameFlashCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashCapabilities";
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameFlashCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameFlashCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameFlashCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameFlashCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameFlashControl(::windows_core::IUnknown);
impl FrameFlashControl {
    pub fn Mode(&self) -> ::windows_core::Result<FrameFlashMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FrameFlashMode>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashMode>(result__)
        }
    }
    pub fn SetMode(&self, value: FrameFlashMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuto)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RedEyeReduction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRedEyeReduction)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PowerPercent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPowerPercent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FrameFlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashControl {}
impl ::core::fmt::Debug for FrameFlashControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameFlashControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashControl;{75d5f6c7-bd45-4fab-9375-45ac04b332c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameFlashControl {
    type Vtable = IFrameFlashControl_Vtbl;
    const IID: ::windows_core::GUID = <IFrameFlashControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashControl";
}
impl ::core::convert::From<FrameFlashControl> for ::windows_core::IUnknown {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows_core::IUnknown {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameFlashControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameFlashControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFlashControl> for ::windows_core::IInspectable {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows_core::IInspectable {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameFlashControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameFlashControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl ::core::marker::Copy for FrameFlashMode {}
impl ::core::clone::Clone for FrameFlashMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FrameFlashMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FrameFlashMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FrameFlashMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameFlashMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.Core.FrameFlashMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FrameFocusCapabilities(::windows_core::IUnknown);
impl FrameFocusCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFocusCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusCapabilities {}
impl ::core::fmt::Debug for FrameFocusCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFocusCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameFocusCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusCapabilities;{7b25cd58-01c0-4065-9c40-c1a721425c1a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameFocusCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameFocusCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusCapabilities";
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameFocusCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameFocusCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameFocusCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameFocusCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameFocusControl(::windows_core::IUnknown);
impl FrameFocusControl {
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameFocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusControl {}
impl ::core::fmt::Debug for FrameFocusControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFocusControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameFocusControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusControl;{272df1d0-d912-4214-a67b-e38a8d48d8c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameFocusControl {
    type Vtable = IFrameFocusControl_Vtbl;
    const IID: ::windows_core::GUID = <IFrameFocusControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusControl";
}
impl ::core::convert::From<FrameFocusControl> for ::windows_core::IUnknown {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows_core::IUnknown {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameFocusControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameFocusControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameFocusControl> for ::windows_core::IInspectable {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows_core::IInspectable {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameFocusControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameFocusControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameIsoSpeedCapabilities(::windows_core::IUnknown);
impl FrameIsoSpeedCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedCapabilities {}
impl ::core::fmt::Debug for FrameIsoSpeedCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameIsoSpeedCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameIsoSpeedCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedCapabilities;{16bdff61-6df6-4ac9-b92a-9f6ecd1ad2fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IFrameIsoSpeedCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameIsoSpeedCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows_core::IUnknown {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows_core::IUnknown {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows_core::IInspectable {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows_core::IInspectable {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameIsoSpeedCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct FrameIsoSpeedControl(::windows_core::IUnknown);
impl FrameIsoSpeedControl {
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuto)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedControl {}
impl ::core::fmt::Debug for FrameIsoSpeedControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameIsoSpeedControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FrameIsoSpeedControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedControl;{1a03efed-786a-4c75-a557-7ab9a85f588c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControl_Vtbl;
    const IID: ::windows_core::GUID = <IFrameIsoSpeedControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FrameIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedControl";
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows_core::IUnknown {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows_core::IUnknown {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FrameIsoSpeedControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FrameIsoSpeedControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows_core::IInspectable {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows_core::IInspectable {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FrameIsoSpeedControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FrameIsoSpeedControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraIntrinsics {
    type Vtable = ICameraIntrinsics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0aa6ed32_6589_49da_afde_594270ca0aac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FocalLength: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PrincipalPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RadialDistortion: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TangentialDistortion: usize,
    pub ImageWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ImageHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectOntoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinate: super::super::super::Foundation::Numerics::Vector3, result__: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectAtUnitDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectAtUnitDepth: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectManyOntoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinates_array_size: u32, coordinates: *const super::super::super::Foundation::Numerics::Vector3, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectManyOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectPixelsAtUnitDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelCoordinates_array_size: u32, pixelcoordinates: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectPixelsAtUnitDepth: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraIntrinsics2 {
    type Vtable = ICameraIntrinsics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cdaa447_0798_4b4d_839f_c5ec414db27a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub UndistortedProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UndistortedProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoints: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsicsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraIntrinsicsFactory {
    type Vtable = ICameraIntrinsicsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0ddc486_2132_4a34_a659_9bfe2a055712);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsicsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthCorrelatedCoordinateMapper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf95d89fb_8af0_4cb0_926d_696866e5046a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthCorrelatedCoordinateMapper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows_core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoints: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows_core::RawPtr, targetcameraintrinsics: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoint: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows_core::RawPtr, targetcameraintrinsics: ::windows_core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameControlCapabilities {
    type Vtable = IFrameControlCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8ffae60_4e9e_4377_a789_e24c4ae7e544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PhotoConfirmationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameControlCapabilities2 {
    type Vtable = IFrameControlCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce9b0464_4730_440f_bd3e_efe8a8f230a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Flash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameController {
    type Vtable = IFrameController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc16459d9_baef_4052_9177_48aff2af7522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExposureControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub SetPhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPhotoConfirmationEnabled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameController2 {
    type Vtable = IFrameController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00d3bc75_d87c_485b_8a09_5c358568b427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FlashControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdbe9ce3_3985_4e72_97c2_0590d61307a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb988a823_8065_41ee_b04f_722265954500);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe95896c9_f7f9_48ca_8591_a26531cb1578);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameExposureControl {
    type Vtable = IFrameExposureControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1605a61_ffaf_4752_b621_f5b6f117f432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb9341a2_5ebe_4f62_8223_0e2b05bfbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameFlashControl {
    type Vtable = IFrameFlashControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d5f6c7_bd45_4fab_9375_45ac04b332c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FrameFlashMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FrameFlashMode) -> ::windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b25cd58_01c0_4065_9c40_c1a721425c1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameFocusControl {
    type Vtable = IFrameFocusControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x272df1d0_d912_4214_a67b_e38a8d48d8c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16bdff61_6df6_4ac9_b92a_9f6ecd1ad2fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a03efed_786a_4c75_a557_7ab9a85f588c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fbff880_ed8c_43fd_a7c3_b35809e4229a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub FrameCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFrameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFrameControllers: usize,
}
#[repr(transparent)]
pub struct VariablePhotoSequenceController(::windows_core::IUnknown);
impl VariablePhotoSequenceController {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPhotosPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn PhotosPerSecondLimit(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosPerSecondLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotosPerSecondLimit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, Param0: ::windows_core::IntoParam<'a, super::super::MediaProperties::IMediaEncodingProperties>>(&self, captureproperties: Param0) -> ::windows_core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(::windows_core::Interface::as_raw(this), captureproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows_core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentFrameRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn FrameCapabilities(&self) -> ::windows_core::Result<FrameControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameControlCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFrameControllers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFrameControllers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<FrameController>>(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoSequenceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceController {}
impl ::core::fmt::Debug for VariablePhotoSequenceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoSequenceController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VariablePhotoSequenceController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.VariablePhotoSequenceController;{7fbff880-ed8c-43fd-a7c3-b35809e4229a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceController_Vtbl;
    const IID: ::windows_core::GUID = <IVariablePhotoSequenceController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VariablePhotoSequenceController {
    const NAME: &'static str = "Windows.Media.Devices.Core.VariablePhotoSequenceController";
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows_core::IUnknown {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows_core::IUnknown {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VariablePhotoSequenceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VariablePhotoSequenceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows_core::IInspectable {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows_core::IInspectable {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VariablePhotoSequenceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VariablePhotoSequenceController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}