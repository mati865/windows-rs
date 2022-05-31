#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberFormatter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1556b49e_bad4_4b4a_900d_4407adb7c981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatWithOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::windows_core::RawPtr, numberformat: PhoneNumberFormat, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatPartialString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatStringWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberFormatterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberFormatterStatics {
    type Vtable = IPhoneNumberFormatterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ca6f931_84d9_414b_ab4e_a0552c878602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, phonenumber: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCountryCodeForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetNationalDirectDialingPrefixForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, stripnondigit: bool, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WrapWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c7ce4dd_c8b4_4ea3_9aef_b342e2c5b417);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetLengthOfGeographicalAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetNationalSignificantNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetLengthOfNationalDestinationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PredictNumberKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PredictedPhoneNumberKind) -> ::windows_core::HRESULT,
    pub GetGeographicRegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CheckNumberMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othernumber: ::windows_core::RawPtr, result__: *mut PhoneNumberMatchResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfoFactory {
    type Vtable = IPhoneNumberInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8202b964_adaa_4cff_8fcf_17e7516a28ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfoStatics {
    type Vtable = IPhoneNumberInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b3f4f6a_86a9_40e9_8649_6d61161928d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, phonenumber: *mut ::windows_core::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows_core::HRESULT,
    pub TryParseWithRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, regioncode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, phonenumber: *mut ::windows_core::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: Self = Self(0i32);
    pub const International: Self = Self(1i32);
    pub const National: Self = Self(2i32);
    pub const Rfc3966: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberFormat {}
impl ::core::clone::Clone for PhoneNumberFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneNumberFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNumberFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNumberFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhoneNumberFormatter(::windows_core::IUnknown);
impl PhoneNumberFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhoneNumberFormatter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Format<'a, Param0: ::windows_core::IntoParam<'a, PhoneNumberInfo>>(&self, number: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatWithOutputFormat<'a, Param0: ::windows_core::IntoParam<'a, PhoneNumberInfo>>(&self, number: Param0, numberformat: PhoneNumberFormat) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatWithOutputFormat)(::windows_core::Interface::as_raw(this), number.into_param().abi(), numberformat, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatPartialString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, number: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatPartialString)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, number: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatString)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatStringWithLeftToRightMarkers<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, number: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatStringWithLeftToRightMarkers)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TryCreate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(regioncode: Param0, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows_core::Result<()> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TryCreate)(::windows_core::Interface::as_raw(this), regioncode.into_param().abi(), phonenumber as *mut _ as _).ok() })
    }
    pub fn GetCountryCodeForRegion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(regioncode: Param0) -> ::windows_core::Result<i32> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetCountryCodeForRegion)(::windows_core::Interface::as_raw(this), regioncode.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn GetNationalDirectDialingPrefixForRegion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(regioncode: Param0, stripnondigit: bool) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetNationalDirectDialingPrefixForRegion)(::windows_core::Interface::as_raw(this), regioncode.into_param().abi(), stripnondigit, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn WrapWithLeftToRightMarkers<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(number: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WrapWithLeftToRightMarkers)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IPhoneNumberFormatterStatics<R, F: FnOnce(&IPhoneNumberFormatterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhoneNumberFormatter, IPhoneNumberFormatterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneNumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberFormatter {}
impl ::core::fmt::Debug for PhoneNumberFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNumberFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter;{1556b49e-bad4-4b4a-900d-4407adb7c981})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneNumberFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNumberFormatter {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter";
}
impl ::core::convert::From<PhoneNumberFormatter> for ::windows_core::IUnknown {
    fn from(value: PhoneNumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNumberFormatter> for ::windows_core::IUnknown {
    fn from(value: &PhoneNumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneNumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneNumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNumberFormatter> for ::windows_core::IInspectable {
    fn from(value: PhoneNumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNumberFormatter> for ::windows_core::IInspectable {
    fn from(value: &PhoneNumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneNumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneNumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneNumberFormatter {}
unsafe impl ::core::marker::Sync for PhoneNumberFormatter {}
#[repr(transparent)]
pub struct PhoneNumberInfo(::windows_core::IUnknown);
impl PhoneNumberInfo {
    pub fn CountryCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CountryCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetLengthOfGeographicalAreaCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetLengthOfGeographicalAreaCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GetNationalSignificantNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetNationalSignificantNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetLengthOfNationalDestinationCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetLengthOfNationalDestinationCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn PredictNumberKind(&self) -> ::windows_core::Result<PredictedPhoneNumberKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PredictedPhoneNumberKind>::zeroed();
            (::windows_core::Interface::vtable(this).PredictNumberKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PredictedPhoneNumberKind>(result__)
        }
    }
    pub fn GetGeographicRegionCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetGeographicRegionCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CheckNumberMatch<'a, Param0: ::windows_core::IntoParam<'a, PhoneNumberInfo>>(&self, othernumber: Param0) -> ::windows_core::Result<PhoneNumberMatchResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneNumberMatchResult>::zeroed();
            (::windows_core::Interface::vtable(this).CheckNumberMatch)(::windows_core::Interface::as_raw(this), othernumber.into_param().abi(), result__.as_mut_ptr()).from_abi::<PhoneNumberMatchResult>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(number: Param0) -> ::windows_core::Result<PhoneNumberInfo> {
        Self::IPhoneNumberInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<PhoneNumberInfo>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows_core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneNumberParseResult>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), phonenumber as *mut _ as _, result__.as_mut_ptr()).from_abi::<PhoneNumberParseResult>(result__)
        })
    }
    pub fn TryParseWithRegion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, regioncode: Param1, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows_core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneNumberParseResult>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseWithRegion)(::windows_core::Interface::as_raw(this), input.into_param().abi(), regioncode.into_param().abi(), phonenumber as *mut _ as _, result__.as_mut_ptr()).from_abi::<PhoneNumberParseResult>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IPhoneNumberInfoFactory<R, F: FnOnce(&IPhoneNumberInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneNumberInfoStatics<R, F: FnOnce(&IPhoneNumberInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhoneNumberInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNumberInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberInfo {}
impl ::core::fmt::Debug for PhoneNumberInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNumberInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo;{1c7ce4dd-c8b4-4ea3-9aef-b342e2c5b417})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneNumberInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNumberInfo {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo";
}
impl ::core::convert::From<PhoneNumberInfo> for ::windows_core::IUnknown {
    fn from(value: PhoneNumberInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNumberInfo> for ::windows_core::IUnknown {
    fn from(value: &PhoneNumberInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNumberInfo> for ::windows_core::IInspectable {
    fn from(value: PhoneNumberInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNumberInfo> for ::windows_core::IInspectable {
    fn from(value: &PhoneNumberInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PhoneNumberInfo> for super::super::Foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: PhoneNumberInfo) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PhoneNumberInfo> for super::super::Foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhoneNumberInfo) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IStringable> for PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IStringable> for &PhoneNumberInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhoneNumberInfo {}
unsafe impl ::core::marker::Sync for PhoneNumberInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: Self = Self(0i32);
    pub const ShortNationalSignificantNumberMatch: Self = Self(1i32);
    pub const NationalSignificantNumberMatch: Self = Self(2i32);
    pub const ExactMatch: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberMatchResult {}
impl ::core::clone::Clone for PhoneNumberMatchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberMatchResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneNumberMatchResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNumberMatchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberMatchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNumberMatchResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberMatchResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: Self = Self(0i32);
    pub const NotANumber: Self = Self(1i32);
    pub const InvalidCountryCode: Self = Self(2i32);
    pub const TooShort: Self = Self(3i32);
    pub const TooLong: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNumberParseResult {}
impl ::core::clone::Clone for PhoneNumberParseResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberParseResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhoneNumberParseResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNumberParseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberParseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneNumberParseResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberParseResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const FixedLineOrMobile: Self = Self(2i32);
    pub const TollFree: Self = Self(3i32);
    pub const PremiumRate: Self = Self(4i32);
    pub const SharedCost: Self = Self(5i32);
    pub const Voip: Self = Self(6i32);
    pub const PersonalNumber: Self = Self(7i32);
    pub const Pager: Self = Self(8i32);
    pub const UniversalAccountNumber: Self = Self(9i32);
    pub const Voicemail: Self = Self(10i32);
    pub const Unknown: Self = Self(11i32);
}
impl ::core::marker::Copy for PredictedPhoneNumberKind {}
impl ::core::clone::Clone for PredictedPhoneNumberKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PredictedPhoneNumberKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PredictedPhoneNumberKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PredictedPhoneNumberKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PredictedPhoneNumberKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PredictedPhoneNumberKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PredictedPhoneNumberKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}