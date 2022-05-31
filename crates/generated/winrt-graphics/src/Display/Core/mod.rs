#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayColorSpace {}
impl ::core::clone::Clone for HdmiDisplayColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayColorSpace {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdmiDisplayColorSpace {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayColorSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayColorSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayColorSpace {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayColorSpace;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct HdmiDisplayHdr2086Metadata {
    pub RedPrimaryX: u16,
    pub RedPrimaryY: u16,
    pub GreenPrimaryX: u16,
    pub GreenPrimaryY: u16,
    pub BluePrimaryX: u16,
    pub BluePrimaryY: u16,
    pub WhitePointX: u16,
    pub WhitePointY: u16,
    pub MaxMasteringLuminance: u16,
    pub MinMasteringLuminance: u16,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for HdmiDisplayHdr2086Metadata {}
impl ::core::clone::Clone for HdmiDisplayHdr2086Metadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HdmiDisplayHdr2086Metadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HdmiDisplayHdr2086Metadata")
            .field("RedPrimaryX", &self.RedPrimaryX)
            .field("RedPrimaryY", &self.RedPrimaryY)
            .field("GreenPrimaryX", &self.GreenPrimaryX)
            .field("GreenPrimaryY", &self.GreenPrimaryY)
            .field("BluePrimaryX", &self.BluePrimaryX)
            .field("BluePrimaryY", &self.BluePrimaryY)
            .field("WhitePointX", &self.WhitePointX)
            .field("WhitePointY", &self.WhitePointY)
            .field("MaxMasteringLuminance", &self.MaxMasteringLuminance)
            .field("MinMasteringLuminance", &self.MinMasteringLuminance)
            .field("MaxContentLightLevel", &self.MaxContentLightLevel)
            .field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for HdmiDisplayHdr2086Metadata {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayHdr2086Metadata {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayHdr2086Metadata {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HdmiDisplayHdr2086Metadata>()) == 0 }
    }
}
impl ::core::cmp::Eq for HdmiDisplayHdr2086Metadata {}
impl ::core::default::Default for HdmiDisplayHdr2086Metadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayHdrOption {}
impl ::core::clone::Clone for HdmiDisplayHdrOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayHdrOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdmiDisplayHdrOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayHdrOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayHdrOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayHdrOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayHdrOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HdmiDisplayInformation(::windows_core::IUnknown);
impl HdmiDisplayInformation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedDisplayModes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HdmiDisplayMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedDisplayModes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HdmiDisplayMode>>(result__)
        }
    }
    pub fn GetCurrentDisplayMode(&self) -> ::windows_core::Result<HdmiDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDisplayMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HdmiDisplayMode>(result__)
        }
    }
    pub fn SetDefaultDisplayModeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetDefaultDisplayModeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RequestSetCurrentDisplayModeAsync<'a, Param0: ::windows_core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeAsync)(::windows_core::Interface::as_raw(this), mode.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestSetCurrentDisplayModeWithHdrAsync<'a, Param0: ::windows_core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAsync)(::windows_core::Interface::as_raw(this), mode.into_param().abi(), hdroption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync<'a, Param0: ::windows_core::IntoParam<'a, HdmiDisplayMode>, Param2: ::windows_core::IntoParam<'a, HdmiDisplayHdr2086Metadata>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption, hdrmetadata: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(::windows_core::Interface::as_raw(this), mode.into_param().abi(), hdroption, hdrmetadata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DisplayModesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<HdmiDisplayInformation, ::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayModesChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisplayModesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisplayModesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<HdmiDisplayInformation> {
        Self::IHdmiDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HdmiDisplayInformation>(result__)
        })
    }
    pub fn IHdmiDisplayInformationStatics<R, F: FnOnce(&IHdmiDisplayInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HdmiDisplayInformation, IHdmiDisplayInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HdmiDisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayInformation {}
impl ::core::fmt::Debug for HdmiDisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayInformation;{130b3c0a-f565-476e-abd5-ea05aee74c69})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
    const IID: ::windows_core::GUID = <IHdmiDisplayInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayInformation";
}
impl ::core::convert::From<HdmiDisplayInformation> for ::windows_core::IUnknown {
    fn from(value: HdmiDisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayInformation> for ::windows_core::IUnknown {
    fn from(value: &HdmiDisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HdmiDisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HdmiDisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HdmiDisplayInformation> for ::windows_core::IInspectable {
    fn from(value: HdmiDisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayInformation> for ::windows_core::IInspectable {
    fn from(value: &HdmiDisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HdmiDisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HdmiDisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HdmiDisplayInformation {}
unsafe impl ::core::marker::Sync for HdmiDisplayInformation {}
#[repr(transparent)]
pub struct HdmiDisplayMode(::windows_core::IUnknown);
impl HdmiDisplayMode {
    pub fn ResolutionWidthInRawPixels(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionWidthInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ResolutionHeightInRawPixels(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionHeightInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn RefreshRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RefreshRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn StereoEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StereoEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn BitsPerPixel(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).BitsPerPixel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEqual)(::windows_core::Interface::as_raw(this), mode.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ColorSpace(&self) -> ::windows_core::Result<HdmiDisplayColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HdmiDisplayColorSpace>::zeroed();
            (::windows_core::Interface::vtable(this).ColorSpace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HdmiDisplayColorSpace>(result__)
        }
    }
    pub fn PixelEncoding(&self) -> ::windows_core::Result<HdmiDisplayPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HdmiDisplayPixelEncoding>::zeroed();
            (::windows_core::Interface::vtable(this).PixelEncoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HdmiDisplayPixelEncoding>(result__)
        }
    }
    pub fn IsSdrLuminanceSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSdrLuminanceSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSmpte2084Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSmpte2084Supported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Is2086MetadataSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Is2086MetadataSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDolbyVisionLowLatencySupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHdmiDisplayMode2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDolbyVisionLowLatencySupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HdmiDisplayMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayMode {}
impl ::core::fmt::Debug for HdmiDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayMode;{0c06d5ad-1b90-4f51-9981-ef5a1c0ddf66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
    const IID: ::windows_core::GUID = <IHdmiDisplayMode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayMode";
}
impl ::core::convert::From<HdmiDisplayMode> for ::windows_core::IUnknown {
    fn from(value: HdmiDisplayMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayMode> for ::windows_core::IUnknown {
    fn from(value: &HdmiDisplayMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HdmiDisplayMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HdmiDisplayMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HdmiDisplayMode> for ::windows_core::IInspectable {
    fn from(value: HdmiDisplayMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayMode> for ::windows_core::IInspectable {
    fn from(value: &HdmiDisplayMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HdmiDisplayMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HdmiDisplayMode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HdmiDisplayMode {}
unsafe impl ::core::marker::Sync for HdmiDisplayMode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayPixelEncoding {}
impl ::core::clone::Clone for HdmiDisplayPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayPixelEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdmiDisplayPixelEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayPixelEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayPixelEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdmiDisplayPixelEncoding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x130b3c0a_f565_476e_abd5_ea05aee74c69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedDisplayModes: usize,
    pub GetCurrentDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeWithHdrAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows_core::RawPtr, hdroption: HdmiDisplayHdrOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows_core::RawPtr, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdmiDisplayInformationStatics {
    type Vtable = IHdmiDisplayInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ce6b260_f42a_4a15_914c_7b8e2a5a65df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c06d5ad_1b90_4f51_9981_ef5a1c0ddf66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ResolutionWidthInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ResolutionHeightInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayColorSpace) -> ::windows_core::HRESULT,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayPixelEncoding) -> ::windows_core::HRESULT,
    pub IsSdrLuminanceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSmpte2084Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Is2086MetadataSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdmiDisplayMode2 {
    type Vtable = IHdmiDisplayMode2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07cd4e9f_4b3c_42b8_84e7_895368718af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDolbyVisionLowLatencySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
