#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: Self = Self(0i32);
    pub const White: Self = Self(1i32);
    pub const Black: Self = Self(2i32);
    pub const Red: Self = Self(3i32);
    pub const Green: Self = Self(4i32);
    pub const Blue: Self = Self(5i32);
    pub const Yellow: Self = Self(6i32);
    pub const Magenta: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
}
impl ::core::marker::Copy for ClosedCaptionColor {}
impl ::core::clone::Clone for ClosedCaptionColor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionColor {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClosedCaptionColor {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionColor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionColor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Raised: Self = Self(2i32);
    pub const Depressed: Self = Self(3i32);
    pub const Uniform: Self = Self(4i32);
    pub const DropShadow: Self = Self(5i32);
}
impl ::core::marker::Copy for ClosedCaptionEdgeEffect {}
impl ::core::clone::Clone for ClosedCaptionEdgeEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionEdgeEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClosedCaptionEdgeEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionEdgeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionEdgeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: Self = Self(0i32);
    pub const OneHundredPercent: Self = Self(1i32);
    pub const SeventyFivePercent: Self = Self(2i32);
    pub const TwentyFivePercent: Self = Self(3i32);
    pub const ZeroPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionOpacity {}
impl ::core::clone::Clone for ClosedCaptionOpacity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionOpacity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClosedCaptionOpacity {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionOpacity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionOpacity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct ClosedCaptionProperties;
impl ClosedCaptionProperties {
    pub fn FontColor() -> ::windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionColor>::zeroed();
            (::windows_core::Interface::vtable(this).FontColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor() -> ::windows_core::Result<::winrt_ui::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).ComputedFontColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        })
    }
    pub fn FontOpacity() -> ::windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionOpacity>::zeroed();
            (::windows_core::Interface::vtable(this).FontOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn FontSize() -> ::windows_core::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionSize>::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionSize>(result__)
        })
    }
    pub fn FontStyle() -> ::windows_core::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionStyle>::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionStyle>(result__)
        })
    }
    pub fn FontEffect() -> ::windows_core::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionEdgeEffect>::zeroed();
            (::windows_core::Interface::vtable(this).FontEffect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionEdgeEffect>(result__)
        })
    }
    pub fn BackgroundColor() -> ::windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionColor>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor() -> ::windows_core::Result<::winrt_ui::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).ComputedBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        })
    }
    pub fn BackgroundOpacity() -> ::windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionOpacity>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn RegionColor() -> ::windows_core::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionColor>::zeroed();
            (::windows_core::Interface::vtable(this).RegionColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor() -> ::windows_core::Result<::winrt_ui::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).ComputedRegionColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        })
    }
    pub fn RegionOpacity() -> ::windows_core::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClosedCaptionOpacity>::zeroed();
            (::windows_core::Interface::vtable(this).RegionOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: Self = Self(0i32);
    pub const FiftyPercent: Self = Self(1i32);
    pub const OneHundredPercent: Self = Self(2i32);
    pub const OneHundredFiftyPercent: Self = Self(3i32);
    pub const TwoHundredPercent: Self = Self(4i32);
}
impl ::core::marker::Copy for ClosedCaptionSize {}
impl ::core::clone::Clone for ClosedCaptionSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClosedCaptionSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionSize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: Self = Self(0i32);
    pub const MonospacedWithSerifs: Self = Self(1i32);
    pub const ProportionalWithSerifs: Self = Self(2i32);
    pub const MonospacedWithoutSerifs: Self = Self(3i32);
    pub const ProportionalWithoutSerifs: Self = Self(4i32);
    pub const Casual: Self = Self(5i32);
    pub const Cursive: Self = Self(6i32);
    pub const SmallCapitals: Self = Self(7i32);
}
impl ::core::marker::Copy for ClosedCaptionStyle {}
impl ::core::clone::Clone for ClosedCaptionStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClosedCaptionStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClosedCaptionStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClosedCaptionStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosedCaptionStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClosedCaptionPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClosedCaptionPropertiesStatics {
    type Vtable = IClosedCaptionPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10aa1f84_cc30_4141_b503_5272289e0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedFontColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedFontColor: usize,
    pub FontOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionSize) -> ::windows_core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionStyle) -> ::windows_core::HRESULT,
    pub FontEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionEdgeEffect) -> ::windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedBackgroundColor: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows_core::HRESULT,
    pub RegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionColor) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ComputedRegionColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ComputedRegionColor: usize,
    pub RegionOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClosedCaptionOpacity) -> ::windows_core::HRESULT,
}
