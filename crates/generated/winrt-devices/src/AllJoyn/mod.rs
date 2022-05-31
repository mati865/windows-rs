#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynAboutData(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynAboutData {
    #[cfg(feature = "deprecated")]
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DefaultAppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultAppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultAppName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultAppName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AppNames(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DateOfManufacture(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DateOfManufacture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetDateOfManufacture<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDateOfManufacture)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DefaultDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Descriptions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Descriptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DefaultManufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultManufacturer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultManufacturer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultManufacturer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Manufacturers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Manufacturers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetModelNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModelNumber)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SoftwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetSoftwareVersion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareVersion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSupportUrl<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportUrl)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAppId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAboutData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynAboutData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynAboutData {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynAboutData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynAboutData").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynAboutData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAboutData;{e5a9bf00-1fa2-4839-93ef-f9df404890f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynAboutData {
    type Vtable = IAllJoynAboutData_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynAboutData as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynAboutData {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAboutData";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAboutData> for ::windows_core::IUnknown {
    fn from(value: AllJoynAboutData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAboutData> for ::windows_core::IUnknown {
    fn from(value: &AllJoynAboutData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynAboutData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynAboutData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAboutData> for ::windows_core::IInspectable {
    fn from(value: AllJoynAboutData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAboutData> for ::windows_core::IInspectable {
    fn from(value: &AllJoynAboutData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynAboutData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynAboutData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynAboutData {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynAboutData {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynAboutDataView(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynAboutDataView {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn AJSoftwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AJSoftwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DateOfManufacture(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DateOfManufacture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(all(feature = "Globalization", feature = "deprecated"))]
    pub fn DefaultLanguage(&self) -> ::windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn HardwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SoftwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "deprecated"))]
    pub fn SupportedLanguages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedLanguages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportUrl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeviceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Manufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Manufacturer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDataBySessionPortAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, AllJoynBusAttachment>>(uniquename: Param0, busattachment: Param1, sessionport: u16) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        Self::IAllJoynAboutDataViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDataBySessionPortAsync)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), busattachment.into_param().abi(), sessionport, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub fn GetDataBySessionPortWithLanguageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, AllJoynBusAttachment>, Param3: ::windows_core::IntoParam<'a, super::super::Globalization::Language>>(uniquename: Param0, busattachment: Param1, sessionport: u16, language: Param3) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        Self::IAllJoynAboutDataViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDataBySessionPortWithLanguageAsync)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), busattachment.into_param().abi(), sessionport, language.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynAboutDataViewStatics<R, F: FnOnce(&IAllJoynAboutDataViewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynAboutDataView, IAllJoynAboutDataViewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAboutDataView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynAboutDataView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynAboutDataView {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynAboutDataView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynAboutDataView").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynAboutDataView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAboutDataView;{6823111f-6212-4934-9c48-e19ca4984288})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynAboutDataView {
    type Vtable = IAllJoynAboutDataView_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynAboutDataView as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynAboutDataView {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAboutDataView";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAboutDataView> for ::windows_core::IUnknown {
    fn from(value: AllJoynAboutDataView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAboutDataView> for ::windows_core::IUnknown {
    fn from(value: &AllJoynAboutDataView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynAboutDataView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynAboutDataView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAboutDataView> for ::windows_core::IInspectable {
    fn from(value: AllJoynAboutDataView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAboutDataView> for ::windows_core::IInspectable {
    fn from(value: &AllJoynAboutDataView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynAboutDataView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynAboutDataView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynAboutDataView {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynAboutDataView {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynAcceptSessionJoinerEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynAcceptSessionJoinerEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SessionPort(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).SessionPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TrafficType(&self) -> ::windows_core::Result<AllJoynTrafficType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynTrafficType>::zeroed();
            (::windows_core::Interface::vtable(this).TrafficType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynTrafficType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SamePhysicalNode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SamePhysicalNode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SameNetwork(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SameNetwork)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, IAllJoynAcceptSessionJoiner>>(uniquename: Param0, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: Param4) -> ::windows_core::Result<AllJoynAcceptSessionJoinerEventArgs> {
        Self::IAllJoynAcceptSessionJoinerEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), sessionport, traffictype, proximity, acceptsessionjoiner.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynAcceptSessionJoinerEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynAcceptSessionJoinerEventArgsFactory<R, F: FnOnce(&IAllJoynAcceptSessionJoinerEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynAcceptSessionJoinerEventArgs, IAllJoynAcceptSessionJoinerEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAcceptSessionJoinerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynAcceptSessionJoinerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynAcceptSessionJoinerEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynAcceptSessionJoinerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynAcceptSessionJoinerEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynAcceptSessionJoinerEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAcceptSessionJoinerEventArgs;{4efb5365-3e8a-4257-8f10-539ce0d56c0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynAcceptSessionJoinerEventArgs {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynAcceptSessionJoinerEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynAcceptSessionJoinerEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAcceptSessionJoinerEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAcceptSessionJoinerEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynAcceptSessionJoinerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAcceptSessionJoinerEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynAcceptSessionJoinerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAcceptSessionJoinerEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynAcceptSessionJoinerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAcceptSessionJoinerEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynAcceptSessionJoinerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynAcceptSessionJoinerEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynAcceptSessionJoinerEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynAcceptSessionJoinerEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynAuthenticationCompleteEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynAuthenticationCompleteEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn AuthenticationMechanism(&self) -> ::windows_core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynAuthenticationMechanism>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationMechanism)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PeerUniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PeerUniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAuthenticationCompleteEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynAuthenticationCompleteEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynAuthenticationCompleteEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynAuthenticationCompleteEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynAuthenticationCompleteEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynAuthenticationCompleteEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynAuthenticationCompleteEventArgs;{97b4701c-15dc-4b53-b6a4-7d134300d7bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynAuthenticationCompleteEventArgs {
    type Vtable = IAllJoynAuthenticationCompleteEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynAuthenticationCompleteEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynAuthenticationCompleteEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynAuthenticationCompleteEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAuthenticationCompleteEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynAuthenticationCompleteEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAuthenticationCompleteEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynAuthenticationCompleteEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynAuthenticationCompleteEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynAuthenticationCompleteEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynAuthenticationCompleteEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynAuthenticationCompleteEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynAuthenticationCompleteEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynAuthenticationCompleteEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynAuthenticationCompleteEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AllJoynAuthenticationMechanism(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynAuthenticationMechanism {
    pub const None: Self = Self(0i32);
    pub const SrpAnonymous: Self = Self(1i32);
    pub const SrpLogon: Self = Self(2i32);
    pub const EcdheNull: Self = Self(3i32);
    pub const EcdhePsk: Self = Self(4i32);
    pub const EcdheEcdsa: Self = Self(5i32);
    pub const EcdheSpeke: Self = Self(6i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynAuthenticationMechanism {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAuthenticationMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AllJoynAuthenticationMechanism {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for AllJoynAuthenticationMechanism {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynAuthenticationMechanism {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynAuthenticationMechanism").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynAuthenticationMechanism {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynAuthenticationMechanism;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynBusAttachment(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynBusAttachment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusAttachment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn AboutData(&self) -> ::windows_core::Result<AllJoynAboutData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AboutData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynAboutData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ConnectionSpecification(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionSpecification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> ::windows_core::Result<AllJoynBusAttachmentState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynBusAttachmentState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynBusAttachmentState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, uniquename: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PingAsync)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Connect(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Connect)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Disconnect(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Disconnect)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AuthenticationMechanisms(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationMechanisms)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CredentialsRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCredentialsRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCredentialsRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CredentialsVerificationRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialsVerificationRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCredentialsVerificationRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCredentialsVerificationRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AuthenticationComplete<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationComplete)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAuthenticationComplete<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAuthenticationComplete)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetAboutDataAsync<'a, Param0: ::windows_core::IntoParam<'a, AllJoynServiceInfo>>(&self, serviceinfo: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAboutDataAsync)(::windows_core::Interface::as_raw(this), serviceinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub fn GetAboutDataWithLanguageAsync<'a, Param0: ::windows_core::IntoParam<'a, AllJoynServiceInfo>, Param1: ::windows_core::IntoParam<'a, super::super::Globalization::Language>>(&self, serviceinfo: Param0, language: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAboutDataWithLanguageAsync)(::windows_core::Interface::as_raw(this), serviceinfo.into_param().abi(), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AcceptSessionJoinerRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptSessionJoinerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAcceptSessionJoinerRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAcceptSessionJoinerRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SessionJoined<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SessionJoined)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSessionJoined<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAllJoynBusAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSessionJoined)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(connectionspecification: Param0) -> ::windows_core::Result<AllJoynBusAttachment> {
        Self::IAllJoynBusAttachmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), connectionspecification.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynBusAttachment>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GetDefault() -> ::windows_core::Result<AllJoynBusAttachment> {
        Self::IAllJoynBusAttachmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynBusAttachment>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetWatcher<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(requiredinterfaces: Param0) -> ::windows_core::Result<super::Enumeration::DeviceWatcher> {
        Self::IAllJoynBusAttachmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetWatcher)(::windows_core::Interface::as_raw(this), requiredinterfaces.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcher>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynBusAttachmentFactory<R, F: FnOnce(&IAllJoynBusAttachmentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusAttachment, IAllJoynBusAttachmentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynBusAttachmentStatics<R, F: FnOnce(&IAllJoynBusAttachmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusAttachment, IAllJoynBusAttachmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynBusAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynBusAttachment {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynBusAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynBusAttachment").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynBusAttachment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusAttachment;{f309f153-1eed-42c3-a20e-436d41fe62f6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynBusAttachment {
    type Vtable = IAllJoynBusAttachment_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynBusAttachment as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynBusAttachment {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusAttachment";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusAttachment> for ::windows_core::IUnknown {
    fn from(value: AllJoynBusAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusAttachment> for ::windows_core::IUnknown {
    fn from(value: &AllJoynBusAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynBusAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynBusAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusAttachment> for ::windows_core::IInspectable {
    fn from(value: AllJoynBusAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusAttachment> for ::windows_core::IInspectable {
    fn from(value: &AllJoynBusAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynBusAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynBusAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynBusAttachment {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynBusAttachment {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AllJoynBusAttachmentState(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynBusAttachmentState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynBusAttachmentState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusAttachmentState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AllJoynBusAttachmentState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for AllJoynBusAttachmentState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynBusAttachmentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynBusAttachmentState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynBusAttachmentState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynBusAttachmentState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynBusAttachmentStateChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynBusAttachmentStateChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> ::windows_core::Result<AllJoynBusAttachmentState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynBusAttachmentState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynBusAttachmentState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusAttachmentStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynBusAttachmentStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynBusAttachmentStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynBusAttachmentStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynBusAttachmentStateChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynBusAttachmentStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusAttachmentStateChangedEventArgs;{d82e75f4-c02a-41ec-a8d5-eab1558953aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynBusAttachmentStateChangedEventArgs {
    type Vtable = IAllJoynBusAttachmentStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynBusAttachmentStateChangedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynBusAttachmentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusAttachmentStateChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusAttachmentStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusAttachmentStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusAttachmentStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusAttachmentStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynBusAttachmentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynBusAttachmentStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynBusAttachmentStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynBusAttachmentStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynBusObject(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynBusObject {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusObject, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AddProducer<'a, Param0: ::windows_core::IntoParam<'a, IAllJoynProducer>>(&self, producer: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddProducer)(::windows_core::Interface::as_raw(this), producer.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BusAttachment(&self) -> ::windows_core::Result<AllJoynBusAttachment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BusAttachment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynBusAttachment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Session(&self) -> ::windows_core::Result<AllJoynSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynSession>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(objectpath: Param0) -> ::windows_core::Result<AllJoynBusObject> {
        Self::IAllJoynBusObjectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), objectpath.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynBusObject>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWithBusAttachment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, AllJoynBusAttachment>>(objectpath: Param0, busattachment: Param1) -> ::windows_core::Result<AllJoynBusObject> {
        Self::IAllJoynBusObjectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithBusAttachment)(::windows_core::Interface::as_raw(this), objectpath.into_param().abi(), busattachment.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynBusObject>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynBusObjectFactory<R, F: FnOnce(&IAllJoynBusObjectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusObject, IAllJoynBusObjectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynBusObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynBusObject {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynBusObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynBusObject").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynBusObject {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusObject;{e8fd825e-f73a-490c-8804-04e026643047})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynBusObject {
    type Vtable = IAllJoynBusObject_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynBusObject as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynBusObject {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusObject";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusObject> for ::windows_core::IUnknown {
    fn from(value: AllJoynBusObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusObject> for ::windows_core::IUnknown {
    fn from(value: &AllJoynBusObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynBusObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynBusObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusObject> for ::windows_core::IInspectable {
    fn from(value: AllJoynBusObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusObject> for ::windows_core::IInspectable {
    fn from(value: &AllJoynBusObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynBusObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynBusObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynBusObject {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynBusObject {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynBusObjectStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynBusObjectStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(status: i32) -> ::windows_core::Result<AllJoynBusObjectStoppedEventArgs> {
        Self::IAllJoynBusObjectStoppedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), status, result__.as_mut_ptr()).from_abi::<AllJoynBusObjectStoppedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynBusObjectStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynBusObjectStoppedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynBusObjectStoppedEventArgs, IAllJoynBusObjectStoppedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusObjectStoppedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynBusObjectStoppedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynBusObjectStoppedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynBusObjectStoppedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynBusObjectStoppedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynBusObjectStoppedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynBusObjectStoppedEventArgs;{de102115-ef8e-4d42-b93b-a2ae74519766})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynBusObjectStoppedEventArgs {
    type Vtable = IAllJoynBusObjectStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynBusObjectStoppedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynBusObjectStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynBusObjectStoppedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusObjectStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynBusObjectStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusObjectStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynBusObjectStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynBusObjectStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynBusObjectStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynBusObjectStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynBusObjectStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynBusObjectStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynBusObjectStoppedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynBusObjectStoppedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynCredentials(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynCredentials {
    #[cfg(feature = "deprecated")]
    pub fn AuthenticationMechanism(&self) -> ::windows_core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynAuthenticationMechanism>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationMechanism)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub fn Certificate(&self) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Certificate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub fn SetCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Cryptography::Certificates::Certificate>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCertificate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub fn PasswordCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PasswordCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub fn SetPasswordCredential<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPasswordCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTimeout<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynCredentials {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynCredentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynCredentials").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynCredentials {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentials;{824650f2-a190-40b1-abab-349ec244dfaa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynCredentials {
    type Vtable = IAllJoynCredentials_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynCredentials as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynCredentials {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentials";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentials> for ::windows_core::IUnknown {
    fn from(value: AllJoynCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentials> for ::windows_core::IUnknown {
    fn from(value: &AllJoynCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentials> for ::windows_core::IInspectable {
    fn from(value: AllJoynCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentials> for ::windows_core::IInspectable {
    fn from(value: &AllJoynCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynCredentials {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynCredentials {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynCredentialsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynCredentialsRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn AttemptCount(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).AttemptCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Credentials(&self) -> ::windows_core::Result<AllJoynCredentials> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Credentials)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynCredentials>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PeerUniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PeerUniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RequestedUserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedUserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynCredentialsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynCredentialsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynCredentialsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynCredentialsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynCredentialsRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynCredentialsRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentialsRequestedEventArgs;{6a87e34e-b069-4b80-9e1a-41bc837c65d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynCredentialsRequestedEventArgs {
    type Vtable = IAllJoynCredentialsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynCredentialsRequestedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynCredentialsRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentialsRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentialsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynCredentialsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentialsRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynCredentialsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentialsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynCredentialsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentialsRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynCredentialsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynCredentialsRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynCredentialsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynCredentialsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynCredentialsVerificationRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynCredentialsVerificationRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn AuthenticationMechanism(&self) -> ::windows_core::Result<AllJoynAuthenticationMechanism> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynAuthenticationMechanism>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationMechanism)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynAuthenticationMechanism>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PeerUniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PeerUniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub fn PeerCertificate(&self) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PeerCertificate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "deprecated"))]
    pub fn PeerCertificateErrorSeverity(&self) -> ::windows_core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Networking::Sockets::SocketSslErrorSeverity>::zeroed();
            (::windows_core::Interface::vtable(this).PeerCertificateErrorSeverity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub fn PeerCertificateErrors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PeerCertificateErrors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub fn PeerIntermediateCertificates(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PeerIntermediateCertificates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynCredentialsVerificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynCredentialsVerificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynCredentialsVerificationRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynCredentialsVerificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynCredentialsVerificationRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynCredentialsVerificationRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynCredentialsVerificationRequestedEventArgs;{800a7612-b805-44af-a2e1-792ab655a2d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynCredentialsVerificationRequestedEventArgs {
    type Vtable = IAllJoynCredentialsVerificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynCredentialsVerificationRequestedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynCredentialsVerificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynCredentialsVerificationRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentialsVerificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentialsVerificationRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynCredentialsVerificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynCredentialsVerificationRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynCredentialsVerificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynCredentialsVerificationRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynCredentialsVerificationRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynCredentialsVerificationRequestedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynMessageInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynMessageInfo {
    #[cfg(feature = "deprecated")]
    pub fn SenderUniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SenderUniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(senderuniquename: Param0) -> ::windows_core::Result<AllJoynMessageInfo> {
        Self::IAllJoynMessageInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), senderuniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynMessageInfo>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynMessageInfoFactory<R, F: FnOnce(&IAllJoynMessageInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynMessageInfo, IAllJoynMessageInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynMessageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynMessageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynMessageInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynMessageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynMessageInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynMessageInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynMessageInfo;{ff2b0127-2c12-4859-aa3a-c74461ee814c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynMessageInfo {
    type Vtable = IAllJoynMessageInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynMessageInfo as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynMessageInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynMessageInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynMessageInfo> for ::windows_core::IUnknown {
    fn from(value: AllJoynMessageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynMessageInfo> for ::windows_core::IUnknown {
    fn from(value: &AllJoynMessageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynMessageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynMessageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynMessageInfo> for ::windows_core::IInspectable {
    fn from(value: AllJoynMessageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynMessageInfo> for ::windows_core::IInspectable {
    fn from(value: &AllJoynMessageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynMessageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynMessageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynMessageInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynMessageInfo {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynProducerStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynProducerStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(status: i32) -> ::windows_core::Result<AllJoynProducerStoppedEventArgs> {
        Self::IAllJoynProducerStoppedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), status, result__.as_mut_ptr()).from_abi::<AllJoynProducerStoppedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynProducerStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynProducerStoppedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynProducerStoppedEventArgs, IAllJoynProducerStoppedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynProducerStoppedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynProducerStoppedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynProducerStoppedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynProducerStoppedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynProducerStoppedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynProducerStoppedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynProducerStoppedEventArgs;{51309770-4937-492d-8080-236439987ceb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynProducerStoppedEventArgs {
    type Vtable = IAllJoynProducerStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynProducerStoppedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynProducerStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynProducerStoppedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynProducerStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynProducerStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynProducerStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynProducerStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynProducerStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynProducerStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynProducerStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynProducerStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynProducerStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynProducerStoppedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynProducerStoppedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynServiceInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynServiceInfo {
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ObjectPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ObjectPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SessionPort(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).SessionPort)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uniquename: Param0, objectpath: Param1, sessionport: u16) -> ::windows_core::Result<AllJoynServiceInfo> {
        Self::IAllJoynServiceInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), objectpath.into_param().abi(), sessionport, result__.as_mut_ptr()).from_abi::<AllJoynServiceInfo>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>> {
        Self::IAllJoynServiceInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynServiceInfoFactory<R, F: FnOnce(&IAllJoynServiceInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynServiceInfo, IAllJoynServiceInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynServiceInfoStatics<R, F: FnOnce(&IAllJoynServiceInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynServiceInfo, IAllJoynServiceInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynServiceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynServiceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynServiceInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynServiceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynServiceInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynServiceInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynServiceInfo;{4cbe8209-b93e-4182-999b-ddd000f9c575})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynServiceInfo {
    type Vtable = IAllJoynServiceInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynServiceInfo as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynServiceInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynServiceInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynServiceInfo> for ::windows_core::IUnknown {
    fn from(value: AllJoynServiceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynServiceInfo> for ::windows_core::IUnknown {
    fn from(value: &AllJoynServiceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynServiceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynServiceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynServiceInfo> for ::windows_core::IInspectable {
    fn from(value: AllJoynServiceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynServiceInfo> for ::windows_core::IInspectable {
    fn from(value: &AllJoynServiceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynServiceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynServiceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynServiceInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynServiceInfo {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynServiceInfoRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynServiceInfoRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uniquename: Param0) -> ::windows_core::Result<AllJoynServiceInfoRemovedEventArgs> {
        Self::IAllJoynServiceInfoRemovedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynServiceInfoRemovedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynServiceInfoRemovedEventArgsFactory<R, F: FnOnce(&IAllJoynServiceInfoRemovedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynServiceInfoRemovedEventArgs, IAllJoynServiceInfoRemovedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynServiceInfoRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynServiceInfoRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynServiceInfoRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynServiceInfoRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynServiceInfoRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynServiceInfoRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynServiceInfoRemovedEventArgs;{3057a95f-1d3f-41f3-8969-e32792627396})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynServiceInfoRemovedEventArgs {
    type Vtable = IAllJoynServiceInfoRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynServiceInfoRemovedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynServiceInfoRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynServiceInfoRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynServiceInfoRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynServiceInfoRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynServiceInfoRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynServiceInfoRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynServiceInfoRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynServiceInfoRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynServiceInfoRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynServiceInfoRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynServiceInfoRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynServiceInfoRemovedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynServiceInfoRemovedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSession(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynSession {
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMemberAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, uniquename: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveMemberAsync)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MemberAdded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MemberAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMemberAdded<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMemberAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MemberRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MemberRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMemberRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMemberRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Lost<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Lost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveLost<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetFromServiceInfoAsync<'a, Param0: ::windows_core::IntoParam<'a, AllJoynServiceInfo>>(serviceinfo: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>> {
        Self::IAllJoynSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromServiceInfoAsync)(::windows_core::Interface::as_raw(this), serviceinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynSession>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetFromServiceInfoAndBusAttachmentAsync<'a, Param0: ::windows_core::IntoParam<'a, AllJoynServiceInfo>, Param1: ::windows_core::IntoParam<'a, AllJoynBusAttachment>>(serviceinfo: Param0, busattachment: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>> {
        Self::IAllJoynSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromServiceInfoAndBusAttachmentAsync)(::windows_core::Interface::as_raw(this), serviceinfo.into_param().abi(), busattachment.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AllJoynSession>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynSessionStatics<R, F: FnOnce(&IAllJoynSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynSession, IAllJoynSessionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynSession {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSession").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSession;{e8d11b0c-c0d4-406c-88a9-a93efa85d4b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynSession {
    type Vtable = IAllJoynSession_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynSession as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynSession {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSession";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSession> for ::windows_core::IUnknown {
    fn from(value: AllJoynSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSession> for ::windows_core::IUnknown {
    fn from(value: &AllJoynSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSession> for ::windows_core::IInspectable {
    fn from(value: AllJoynSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSession> for ::windows_core::IInspectable {
    fn from(value: &AllJoynSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynSession {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynSession {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSessionJoinedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynSessionJoinedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Session(&self) -> ::windows_core::Result<AllJoynSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynSession>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, AllJoynSession>>(session: Param0) -> ::windows_core::Result<AllJoynSessionJoinedEventArgs> {
        Self::IAllJoynSessionJoinedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), session.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynSessionJoinedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynSessionJoinedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionJoinedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynSessionJoinedEventArgs, IAllJoynSessionJoinedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionJoinedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynSessionJoinedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynSessionJoinedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSessionJoinedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSessionJoinedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSessionJoinedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionJoinedEventArgs;{9e9f5bd0-b5d7-47c5-8dab-b040cc192871})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynSessionJoinedEventArgs {
    type Vtable = IAllJoynSessionJoinedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynSessionJoinedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynSessionJoinedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionJoinedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionJoinedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynSessionJoinedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionJoinedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynSessionJoinedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionJoinedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynSessionJoinedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionJoinedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynSessionJoinedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynSessionJoinedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynSessionJoinedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynSessionJoinedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSessionLostEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynSessionLostEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Reason(&self) -> ::windows_core::Result<AllJoynSessionLostReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AllJoynSessionLostReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AllJoynSessionLostReason>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(reason: AllJoynSessionLostReason) -> ::windows_core::Result<AllJoynSessionLostEventArgs> {
        Self::IAllJoynSessionLostEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<AllJoynSessionLostEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynSessionLostEventArgsFactory<R, F: FnOnce(&IAllJoynSessionLostEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynSessionLostEventArgs, IAllJoynSessionLostEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionLostEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynSessionLostEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynSessionLostEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSessionLostEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSessionLostEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSessionLostEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionLostEventArgs;{e766a48a-8bb8-4954-ae67-d2fa43d1f96b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynSessionLostEventArgs {
    type Vtable = IAllJoynSessionLostEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynSessionLostEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynSessionLostEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionLostEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionLostEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynSessionLostEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionLostEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynSessionLostEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionLostEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynSessionLostEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionLostEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynSessionLostEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynSessionLostEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynSessionLostEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynSessionLostEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AllJoynSessionLostReason(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynSessionLostReason {
    pub const None: Self = Self(0i32);
    pub const ProducerLeftSession: Self = Self(1i32);
    pub const ProducerClosedAbruptly: Self = Self(2i32);
    pub const RemovedByProducer: Self = Self(3i32);
    pub const LinkTimeout: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynSessionLostReason {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionLostReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AllJoynSessionLostReason {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for AllJoynSessionLostReason {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSessionLostReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSessionLostReason").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSessionLostReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynSessionLostReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSessionMemberAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynSessionMemberAddedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uniquename: Param0) -> ::windows_core::Result<AllJoynSessionMemberAddedEventArgs> {
        Self::IAllJoynSessionMemberAddedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynSessionMemberAddedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynSessionMemberAddedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionMemberAddedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynSessionMemberAddedEventArgs, IAllJoynSessionMemberAddedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionMemberAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynSessionMemberAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynSessionMemberAddedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSessionMemberAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSessionMemberAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSessionMemberAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionMemberAddedEventArgs;{49a2798a-0dd1-46c1-9cd6-27190e503a5e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynSessionMemberAddedEventArgs {
    type Vtable = IAllJoynSessionMemberAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynSessionMemberAddedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynSessionMemberAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionMemberAddedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionMemberAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynSessionMemberAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionMemberAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynSessionMemberAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionMemberAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynSessionMemberAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionMemberAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynSessionMemberAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynSessionMemberAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynSessionMemberAddedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynSessionMemberAddedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSessionMemberRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynSessionMemberRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn UniqueName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UniqueName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uniquename: Param0) -> ::windows_core::Result<AllJoynSessionMemberRemovedEventArgs> {
        Self::IAllJoynSessionMemberRemovedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uniquename.into_param().abi(), result__.as_mut_ptr()).from_abi::<AllJoynSessionMemberRemovedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynSessionMemberRemovedEventArgsFactory<R, F: FnOnce(&IAllJoynSessionMemberRemovedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynSessionMemberRemovedEventArgs, IAllJoynSessionMemberRemovedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionMemberRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynSessionMemberRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynSessionMemberRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynSessionMemberRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynSessionMemberRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynSessionMemberRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynSessionMemberRemovedEventArgs;{409a219f-aa4a-4893-b430-baa1b63c6219})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynSessionMemberRemovedEventArgs {
    type Vtable = IAllJoynSessionMemberRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynSessionMemberRemovedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynSessionMemberRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynSessionMemberRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionMemberRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynSessionMemberRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionMemberRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynSessionMemberRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynSessionMemberRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynSessionMemberRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynSessionMemberRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynSessionMemberRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynSessionMemberRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynSessionMemberRemovedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynSessionMemberRemovedEventArgs {}
#[cfg(feature = "deprecated")]
pub struct AllJoynStatus;
#[cfg(feature = "deprecated")]
impl AllJoynStatus {
    #[cfg(feature = "deprecated")]
    pub fn Ok() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Ok)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Fail() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Fail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn OperationTimedOut() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).OperationTimedOut)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn OtherEndClosed() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).OtherEndClosed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ConnectionRefused() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionRefused)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn AuthenticationFailed() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationFailed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn AuthenticationRejectedByUser() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationRejectedByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SslConnectFailed() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SslConnectFailed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SslIdentityVerificationFailed() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SslIdentityVerificationFailed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InsufficientSecurity() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InsufficientSecurity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument1() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument2() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument3() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument4() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument5() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument6() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument6)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument7() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn InvalidArgument8() -> ::windows_core::Result<i32> {
        Self::IAllJoynStatusStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).InvalidArgument8)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynStatusStatics<R, F: FnOnce(&IAllJoynStatusStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynStatus, IAllJoynStatusStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynStatus {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynStatus";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AllJoynTrafficType(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynTrafficType {
    pub const Unknown: Self = Self(0i32);
    pub const Messages: Self = Self(1i32);
    pub const RawUnreliable: Self = Self(2i32);
    pub const RawReliable: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynTrafficType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynTrafficType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AllJoynTrafficType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for AllJoynTrafficType {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynTrafficType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynTrafficType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynTrafficType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.AllJoyn.AllJoynTrafficType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynWatcherStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl AllJoynWatcherStoppedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(status: i32) -> ::windows_core::Result<AllJoynWatcherStoppedEventArgs> {
        Self::IAllJoynWatcherStoppedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), status, result__.as_mut_ptr()).from_abi::<AllJoynWatcherStoppedEventArgs>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn IAllJoynWatcherStoppedEventArgsFactory<R, F: FnOnce(&IAllJoynWatcherStoppedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AllJoynWatcherStoppedEventArgs, IAllJoynWatcherStoppedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for AllJoynWatcherStoppedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for AllJoynWatcherStoppedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AllJoynWatcherStoppedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AllJoynWatcherStoppedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for AllJoynWatcherStoppedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.AllJoyn.AllJoynWatcherStoppedEventArgs;{c9fca03b-701d-4aa8-97dd-a2bb0a8f5fa3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for AllJoynWatcherStoppedEventArgs {
    type Vtable = IAllJoynWatcherStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAllJoynWatcherStoppedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for AllJoynWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.AllJoynWatcherStoppedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynWatcherStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AllJoynWatcherStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynWatcherStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AllJoynWatcherStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<AllJoynWatcherStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AllJoynWatcherStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&AllJoynWatcherStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AllJoynWatcherStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AllJoynWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for AllJoynWatcherStoppedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for AllJoynWatcherStoppedEventArgs {}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAboutData(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAboutData {
    type Vtable = IAllJoynAboutData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5a9bf00_1fa2_4839_93ef_f9df404890f7);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultAppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultAppName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultAppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultAppName: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AppNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AppNames: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DateOfManufacture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DateOfManufacture: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetDateOfManufacture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetDateOfManufacture: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultDescription: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultDescription: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Descriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Descriptions: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultManufacturer: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultManufacturer: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Manufacturers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Manufacturers: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SetModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SoftwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoftwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub SetSoftwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSoftwareVersion: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SupportUrl: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSupportUrl: usize,
    #[cfg(feature = "deprecated")]
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppId: usize,
    #[cfg(feature = "deprecated")]
    pub SetAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAppId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAboutDataView(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAboutDataView {
    type Vtable = IAllJoynAboutDataView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6823111f_6212_4934_9c48_e19ca4984288);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutDataView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(feature = "deprecated")]
    pub AJSoftwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AJSoftwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppId: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DateOfManufacture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DateOfManufacture: usize,
    #[cfg(all(feature = "Globalization", feature = "deprecated"))]
    pub DefaultLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Globalization", feature = "deprecated")))]
    DefaultLanguage: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub HardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HardwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SoftwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoftwareVersion: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "deprecated"))]
    pub SupportedLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization", feature = "deprecated")))]
    SupportedLanguages: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SupportUrl: usize,
    #[cfg(feature = "deprecated")]
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppName: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceName: usize,
    #[cfg(feature = "deprecated")]
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Manufacturer: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAboutDataViewStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAboutDataViewStatics {
    type Vtable = IAllJoynAboutDataViewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57edb688_0c5e_416e_88b5_39b32d25c47d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAboutDataViewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDataBySessionPortAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, busattachment: ::windows_core::RawPtr, sessionport: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDataBySessionPortAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub GetDataBySessionPortWithLanguageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, busattachment: ::windows_core::RawPtr, sessionport: u16, language: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization", feature = "deprecated")))]
    GetDataBySessionPortWithLanguageAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoiner(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IAllJoynAcceptSessionJoiner {
    #[cfg(feature = "deprecated")]
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IAllJoynAcceptSessionJoiner> for ::windows_core::IUnknown {
    fn from(value: IAllJoynAcceptSessionJoiner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IAllJoynAcceptSessionJoiner> for ::windows_core::IUnknown {
    fn from(value: &IAllJoynAcceptSessionJoiner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IAllJoynAcceptSessionJoiner> for ::windows_core::IInspectable {
    fn from(value: IAllJoynAcceptSessionJoiner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IAllJoynAcceptSessionJoiner> for ::windows_core::IInspectable {
    fn from(value: &IAllJoynAcceptSessionJoiner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAllJoynAcceptSessionJoiner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IAllJoynAcceptSessionJoiner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IAllJoynAcceptSessionJoiner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IAllJoynAcceptSessionJoiner {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IAllJoynAcceptSessionJoiner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAllJoynAcceptSessionJoiner").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IAllJoynAcceptSessionJoiner {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4da817d2-cd1d-4023-a7c4-16def89c28df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAcceptSessionJoiner {
    type Vtable = IAllJoynAcceptSessionJoiner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4da817d2_cd1d_4023_a7c4_16def89c28df);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoiner_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAcceptSessionJoinerEventArgs {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4efb5365_3e8a_4257_8f10_539ce0d56c0f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub SessionPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SessionPort: usize,
    #[cfg(feature = "deprecated")]
    pub TrafficType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynTrafficType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrafficType: usize,
    #[cfg(feature = "deprecated")]
    pub SamePhysicalNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SamePhysicalNode: usize,
    #[cfg(feature = "deprecated")]
    pub SameNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SameNetwork: usize,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAcceptSessionJoinerEventArgsFactory {
    type Vtable = IAllJoynAcceptSessionJoinerEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4435bc0_6145_429e_84db_d5bfe772b14f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynAuthenticationCompleteEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynAuthenticationCompleteEventArgs {
    type Vtable = IAllJoynAuthenticationCompleteEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97b4701c_15dc_4b53_b6a4_7d134300d7bf);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynAuthenticationCompleteEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Succeeded: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusAttachment(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusAttachment {
    type Vtable = IAllJoynBusAttachment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf309f153_1eed_42c3_a20e_436d41fe62f6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AboutData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AboutData: usize,
    #[cfg(feature = "deprecated")]
    pub ConnectionSpecification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConnectionSpecification: usize,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynBusAttachmentState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PingAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Connect: usize,
    #[cfg(feature = "deprecated")]
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Disconnect: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStateChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AuthenticationMechanisms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AuthenticationMechanisms: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CredentialsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CredentialsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCredentialsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCredentialsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CredentialsVerificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CredentialsVerificationRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCredentialsVerificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCredentialsVerificationRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AuthenticationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AuthenticationComplete: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAuthenticationComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAuthenticationComplete: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusAttachment2(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusAttachment2 {
    type Vtable = IAllJoynBusAttachment2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3474cb1e_2368_43b2_b43e_6a3ac1278d98);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachment2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetAboutDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetAboutDataAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub GetAboutDataWithLanguageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceinfo: ::windows_core::RawPtr, language: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization", feature = "deprecated")))]
    GetAboutDataWithLanguageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AcceptSessionJoinerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AcceptSessionJoinerRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAcceptSessionJoinerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAcceptSessionJoinerRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SessionJoined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SessionJoined: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSessionJoined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSessionJoined: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusAttachmentFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusAttachmentFactory {
    type Vtable = IAllJoynBusAttachmentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x642ef1a4_ad85_4ddf_90ae_604452b22288);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionspecification: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusAttachmentStateChangedEventArgs {
    type Vtable = IAllJoynBusAttachmentStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd82e75f4_c02a_41ec_a8d5_eab1558953aa);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynBusAttachmentState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusAttachmentStatics {
    type Vtable = IAllJoynBusAttachmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x839d4d3d_1051_40d7_872a_8d0141115b1f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusAttachmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredinterfaces: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "deprecated")))]
    GetWatcher: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusObject(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusObject {
    type Vtable = IAllJoynBusObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8fd825e_f73a_490c_8804_04e026643047);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObject_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
    #[cfg(feature = "deprecated")]
    pub AddProducer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, producer: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddProducer: usize,
    #[cfg(feature = "deprecated")]
    pub BusAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BusAttachment: usize,
    #[cfg(feature = "deprecated")]
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Session: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusObjectFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusObjectFactory {
    type Vtable = IAllJoynBusObjectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c2f9f0b_8e02_4f9c_ac27_ea6dad5d3b50);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
    #[cfg(feature = "deprecated")]
    pub CreateWithBusAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, busattachment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWithBusAttachment: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusObjectStoppedEventArgs {
    type Vtable = IAllJoynBusObjectStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde102115_ef8e_4d42_b93b_a2ae74519766);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynBusObjectStoppedEventArgsFactory {
    type Vtable = IAllJoynBusObjectStoppedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b22fd48_d0a3_4255_953a_4772b4028073);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynCredentials(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynCredentials {
    type Vtable = IAllJoynCredentials_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x824650f2_a190_40b1_abab_349ec244dfaa);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentials_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub Certificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    Certificate: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub SetCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    SetCertificate: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub PasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "deprecated")))]
    PasswordCredential: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub SetPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "deprecated")))]
    SetPasswordCredential: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timeout: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimeout: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynCredentialsRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynCredentialsRequestedEventArgs {
    type Vtable = IAllJoynCredentialsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a87e34e_b069_4b80_9e1a_41bc837c65d2);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentialsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AttemptCount: usize,
    #[cfg(feature = "deprecated")]
    pub Credentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Credentials: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub RequestedUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestedUserName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynCredentialsVerificationRequestedEventArgs {
    type Vtable = IAllJoynCredentialsVerificationRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x800a7612_b805_44af_a2e1_792ab655a2d0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerCertificate: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "deprecated"))]
    pub PeerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "deprecated")))]
    PeerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerIntermediateCertificates: usize,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynMessageInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynMessageInfo {
    type Vtable = IAllJoynMessageInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff2b0127_2c12_4859_aa3a_c74461ee814c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynMessageInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub SenderUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SenderUniqueName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynMessageInfoFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynMessageInfoFactory {
    type Vtable = IAllJoynMessageInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34664c2a_8289_43d4_b4a8_3f4de359f043);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynMessageInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, senderuniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynProducer(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IAllJoynProducer {
    #[cfg(feature = "deprecated")]
    pub fn SetBusObject<'a, Param0: ::windows_core::IntoParam<'a, AllJoynBusObject>>(&self, busobject: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBusObject)(::windows_core::Interface::as_raw(this), busobject.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IAllJoynProducer> for ::windows_core::IUnknown {
    fn from(value: IAllJoynProducer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IAllJoynProducer> for ::windows_core::IUnknown {
    fn from(value: &IAllJoynProducer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAllJoynProducer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAllJoynProducer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IAllJoynProducer> for ::windows_core::IInspectable {
    fn from(value: IAllJoynProducer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IAllJoynProducer> for ::windows_core::IInspectable {
    fn from(value: &IAllJoynProducer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAllJoynProducer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAllJoynProducer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IAllJoynProducer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IAllJoynProducer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IAllJoynProducer {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IAllJoynProducer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAllJoynProducer").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IAllJoynProducer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9d084679-469b-495a-a710-ac50f123069f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynProducer {
    type Vtable = IAllJoynProducer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d084679_469b_495a_a710_ac50f123069f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub SetBusObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busobject: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBusObject: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynProducerStoppedEventArgs {
    type Vtable = IAllJoynProducerStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51309770_4937_492d_8080_236439987ceb);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynProducerStoppedEventArgsFactory {
    type Vtable = IAllJoynProducerStoppedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56529961_b219_4d6e_9f78_fa3f99fa8fe5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynProducerStoppedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynServiceInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynServiceInfo {
    type Vtable = IAllJoynServiceInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cbe8209_b93e_4182_999b_ddd000f9c575);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub ObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ObjectPath: usize,
    #[cfg(feature = "deprecated")]
    pub SessionPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SessionPort: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynServiceInfoFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynServiceInfoFactory {
    type Vtable = IAllJoynServiceInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7581dabd_fe03_4f4b_94a4_f02fdcbd11b8);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, objectpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionport: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynServiceInfoRemovedEventArgs {
    type Vtable = IAllJoynServiceInfoRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3057a95f_1d3f_41f3_8969_e32792627396);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynServiceInfoRemovedEventArgsFactory {
    type Vtable = IAllJoynServiceInfoRemovedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0dbf8627_9aff_4955_9227_6953baf41569);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynServiceInfoStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynServiceInfoStatics {
    type Vtable = IAllJoynServiceInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5678570a_603a_49fc_b750_0ef13609213c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynServiceInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSession(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSession {
    type Vtable = IAllJoynSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8d11b0c_c0d4_406c_88a9_a93efa85d4b1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MemberAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MemberAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MemberRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MemberRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Lost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Lost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLost: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionJoinedEventArgs {
    type Vtable = IAllJoynSessionJoinedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e9f5bd0_b5d7_47c5_8dab_b040cc192871);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Session: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionJoinedEventArgsFactory {
    type Vtable = IAllJoynSessionJoinedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6824d689_d6cb_4d9e_a09e_35806870b17f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionJoinedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionLostEventArgs {
    type Vtable = IAllJoynSessionLostEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe766a48a_8bb8_4954_ae67_d2fa43d1f96b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AllJoynSessionLostReason) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reason: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionLostEventArgsFactory {
    type Vtable = IAllJoynSessionLostEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13bbfd32_d2f4_49c9_980e_2805e13586b1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionLostEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AllJoynSessionLostReason, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionMemberAddedEventArgs {
    type Vtable = IAllJoynSessionMemberAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49a2798a_0dd1_46c1_9cd6_27190e503a5e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionMemberAddedEventArgsFactory {
    type Vtable = IAllJoynSessionMemberAddedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x341de352_1d33_40a1_a1d3_e5777020e1f1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionMemberRemovedEventArgs {
    type Vtable = IAllJoynSessionMemberRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x409a219f_aa4a_4893_b430_baa1b63c6219);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionMemberRemovedEventArgsFactory {
    type Vtable = IAllJoynSessionMemberRemovedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4d355e8_42b8_4b67_b757_d0cfcad59280);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynSessionStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynSessionStatics {
    type Vtable = IAllJoynSessionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e05d604_a06c_46d4_b46c_0b0b54105b44);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFromServiceInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFromServiceInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFromServiceInfoAndBusAttachmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceinfo: ::windows_core::RawPtr, busattachment: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFromServiceInfoAndBusAttachmentAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynStatusStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynStatusStatics {
    type Vtable = IAllJoynStatusStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0b7a17e_0d29_4da9_8ac6_54c554bedbc5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynStatusStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Ok: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Ok: usize,
    #[cfg(feature = "deprecated")]
    pub Fail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Fail: usize,
    #[cfg(feature = "deprecated")]
    pub OperationTimedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OperationTimedOut: usize,
    #[cfg(feature = "deprecated")]
    pub OtherEndClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OtherEndClosed: usize,
    #[cfg(feature = "deprecated")]
    pub ConnectionRefused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConnectionRefused: usize,
    #[cfg(feature = "deprecated")]
    pub AuthenticationFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationFailed: usize,
    #[cfg(feature = "deprecated")]
    pub AuthenticationRejectedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationRejectedByUser: usize,
    #[cfg(feature = "deprecated")]
    pub SslConnectFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SslConnectFailed: usize,
    #[cfg(feature = "deprecated")]
    pub SslIdentityVerificationFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SslIdentityVerificationFailed: usize,
    #[cfg(feature = "deprecated")]
    pub InsufficientSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InsufficientSecurity: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument1: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument2: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument3: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument4: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument5: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument6: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument7: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument8: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynWatcherStoppedEventArgs {
    type Vtable = IAllJoynWatcherStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9fca03b_701d_4aa8_97dd_a2bb0a8f5fa3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgsFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IAllJoynWatcherStoppedEventArgsFactory {
    type Vtable = IAllJoynWatcherStoppedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x878fa5a8_2d50_47e1_904a_20bf0d48c782);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAllJoynWatcherStoppedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}