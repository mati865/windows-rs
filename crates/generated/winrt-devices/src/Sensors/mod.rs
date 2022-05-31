#[cfg(feature = "Custom")]
pub mod Custom;
#[repr(transparent)]
pub struct Accelerometer(::windows_core::IUnknown);
impl Accelerometer {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccelerometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Shaken<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Shaken)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShaken<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShaken)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows_core::Result<AccelerometerReadingType> {
        let this = &::windows_core::Interface::cast::<IAccelerometer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccelerometerReadingType>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccelerometerReadingType>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<AccelerometerDataThreshold> {
        let this = &::windows_core::Interface::cast::<IAccelerometer5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccelerometerDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAccelerometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Accelerometer>(result__)
        })
    }
    pub fn GetDefaultWithAccelerometerReadingType(readingtype: AccelerometerReadingType) -> ::windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultWithAccelerometerReadingType)(::windows_core::Interface::as_raw(this), readingtype, result__.as_mut_ptr()).from_abi::<Accelerometer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Accelerometer>>(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: AccelerometerReadingType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), readingtype, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IAccelerometerStatics<R, F: FnOnce(&IAccelerometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Accelerometer, IAccelerometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccelerometerStatics2<R, F: FnOnce(&IAccelerometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Accelerometer, IAccelerometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccelerometerStatics3<R, F: FnOnce(&IAccelerometerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Accelerometer, IAccelerometerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Accelerometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Accelerometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Accelerometer {}
impl ::core::fmt::Debug for Accelerometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Accelerometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Accelerometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Accelerometer;{df184548-2711-4da7-8098-4b82205d3c7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Accelerometer {
    type Vtable = IAccelerometer_Vtbl;
    const IID: ::windows_core::GUID = <IAccelerometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Accelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Accelerometer";
}
impl ::core::convert::From<Accelerometer> for ::windows_core::IUnknown {
    fn from(value: Accelerometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Accelerometer> for ::windows_core::IUnknown {
    fn from(value: &Accelerometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Accelerometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Accelerometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Accelerometer> for ::windows_core::IInspectable {
    fn from(value: Accelerometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Accelerometer> for ::windows_core::IInspectable {
    fn from(value: &Accelerometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Accelerometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Accelerometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Accelerometer {}
unsafe impl ::core::marker::Sync for Accelerometer {}
#[repr(transparent)]
pub struct AccelerometerDataThreshold(::windows_core::IUnknown);
impl AccelerometerDataThreshold {
    pub fn XAxisInGForce(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).XAxisInGForce)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetXAxisInGForce(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXAxisInGForce)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInGForce(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).YAxisInGForce)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetYAxisInGForce(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYAxisInGForce)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInGForce(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ZAxisInGForce)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetZAxisInGForce(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZAxisInGForce)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AccelerometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccelerometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerDataThreshold {}
impl ::core::fmt::Debug for AccelerometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccelerometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerDataThreshold;{f92c1b68-6320-5577-879e-9942621c3dd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <IAccelerometerDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerDataThreshold";
}
impl ::core::convert::From<AccelerometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: AccelerometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &AccelerometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccelerometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccelerometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccelerometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: AccelerometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &AccelerometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccelerometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccelerometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccelerometerDataThreshold {}
unsafe impl ::core::marker::Sync for AccelerometerDataThreshold {}
#[repr(transparent)]
pub struct AccelerometerReading(::windows_core::IUnknown);
impl AccelerometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AccelerationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AccelerationX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AccelerationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AccelerationY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AccelerationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AccelerationZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for AccelerometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccelerometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerReading {}
impl ::core::fmt::Debug for AccelerometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccelerometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReading;{b9fe7acb-d351-40af-8bb6-7aa9ae641fb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccelerometerReading {
    type Vtable = IAccelerometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IAccelerometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReading";
}
impl ::core::convert::From<AccelerometerReading> for ::windows_core::IUnknown {
    fn from(value: AccelerometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerReading> for ::windows_core::IUnknown {
    fn from(value: &AccelerometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccelerometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccelerometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccelerometerReading> for ::windows_core::IInspectable {
    fn from(value: AccelerometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerReading> for ::windows_core::IInspectable {
    fn from(value: &AccelerometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccelerometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccelerometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccelerometerReading {}
unsafe impl ::core::marker::Sync for AccelerometerReading {}
#[repr(transparent)]
pub struct AccelerometerReadingChangedEventArgs(::windows_core::IUnknown);
impl AccelerometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccelerometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for AccelerometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccelerometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for AccelerometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccelerometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs;{0095c65b-b6ac-475a-9f44-8b32d35a3f25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAccelerometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs";
}
impl ::core::convert::From<AccelerometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AccelerometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AccelerometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccelerometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AccelerometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AccelerometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccelerometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccelerometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl ::core::marker::Copy for AccelerometerReadingType {}
impl ::core::clone::Clone for AccelerometerReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AccelerometerReadingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AccelerometerReadingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AccelerometerReadingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerReadingType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccelerometerReadingType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.AccelerometerReadingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AccelerometerShakenEventArgs(::windows_core::IUnknown);
impl AccelerometerShakenEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for AccelerometerShakenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccelerometerShakenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccelerometerShakenEventArgs {}
impl ::core::fmt::Debug for AccelerometerShakenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccelerometerShakenEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccelerometerShakenEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AccelerometerShakenEventArgs;{95ff01d1-4a28-4f35-98e8-8178aae4084a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAccelerometerShakenEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerShakenEventArgs";
}
impl ::core::convert::From<AccelerometerShakenEventArgs> for ::windows_core::IUnknown {
    fn from(value: AccelerometerShakenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerShakenEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AccelerometerShakenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccelerometerShakenEventArgs> for ::windows_core::IInspectable {
    fn from(value: AccelerometerShakenEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccelerometerShakenEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AccelerometerShakenEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccelerometerShakenEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccelerometerShakenEventArgs {}
unsafe impl ::core::marker::Sync for AccelerometerShakenEventArgs {}
#[repr(transparent)]
pub struct ActivitySensor(::windows_core::IUnknown);
impl ActivitySensor {
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReadingAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedActivities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubscribedActivities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<ActivityType>>(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PowerInMilliwatts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedActivities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedActivities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ActivityType>>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensor>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ActivitySensor>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(fromtime: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemHistoryAsync)(::windows_core::Interface::as_raw(this), fromtime.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>>(fromtime: Param0, duration: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(::windows_core::Interface::as_raw(this), fromtime.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>(result__)
        })
    }
    pub fn IActivitySensorStatics<R, F: FnOnce(&IActivitySensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ActivitySensor, IActivitySensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ActivitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensor {}
impl ::core::fmt::Debug for ActivitySensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensor;{cd7a630c-fb5f-48eb-b09b-a2708d1c61ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivitySensor {
    type Vtable = IActivitySensor_Vtbl;
    const IID: ::windows_core::GUID = <IActivitySensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensor";
}
impl ::core::convert::From<ActivitySensor> for ::windows_core::IUnknown {
    fn from(value: ActivitySensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensor> for ::windows_core::IUnknown {
    fn from(value: &ActivitySensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensor> for ::windows_core::IInspectable {
    fn from(value: ActivitySensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensor> for ::windows_core::IInspectable {
    fn from(value: &ActivitySensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivitySensor {}
unsafe impl ::core::marker::Sync for ActivitySensor {}
#[repr(transparent)]
pub struct ActivitySensorReading(::windows_core::IUnknown);
impl ActivitySensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Activity(&self) -> ::windows_core::Result<ActivityType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivityType>::zeroed();
            (::windows_core::Interface::vtable(this).Activity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivityType>(result__)
        }
    }
    pub fn Confidence(&self) -> ::windows_core::Result<ActivitySensorReadingConfidence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivitySensorReadingConfidence>::zeroed();
            (::windows_core::Interface::vtable(this).Confidence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivitySensorReadingConfidence>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReading {}
impl ::core::fmt::Debug for ActivitySensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensorReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReading;{85125a96-1472-40a2-b2ae-e1ef29226c78})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivitySensorReading {
    type Vtable = IActivitySensorReading_Vtbl;
    const IID: ::windows_core::GUID = <IActivitySensorReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReading";
}
impl ::core::convert::From<ActivitySensorReading> for ::windows_core::IUnknown {
    fn from(value: ActivitySensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReading> for ::windows_core::IUnknown {
    fn from(value: &ActivitySensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensorReading> for ::windows_core::IInspectable {
    fn from(value: ActivitySensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReading> for ::windows_core::IInspectable {
    fn from(value: &ActivitySensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReading {}
unsafe impl ::core::marker::Sync for ActivitySensorReading {}
#[repr(transparent)]
pub struct ActivitySensorReadingChangeReport(::windows_core::IUnknown);
impl ActivitySensorReadingChangeReport {
    pub fn Reading(&self) -> ::windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivitySensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivitySensorReadingChangeReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReadingChangeReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReadingChangeReport {}
impl ::core::fmt::Debug for ActivitySensorReadingChangeReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingChangeReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensorReadingChangeReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangeReport;{4f3c2915-d93b-47bd-960a-f20fb2f322b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_Vtbl;
    const IID: ::windows_core::GUID = <IActivitySensorReadingChangeReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangeReport";
}
impl ::core::convert::From<ActivitySensorReadingChangeReport> for ::windows_core::IUnknown {
    fn from(value: ActivitySensorReadingChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangeReport> for ::windows_core::IUnknown {
    fn from(value: &ActivitySensorReadingChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensorReadingChangeReport> for ::windows_core::IInspectable {
    fn from(value: ActivitySensorReadingChangeReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangeReport> for ::windows_core::IInspectable {
    fn from(value: &ActivitySensorReadingChangeReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivitySensorReadingChangeReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReadingChangeReport {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangeReport {}
#[repr(transparent)]
pub struct ActivitySensorReadingChangedEventArgs(::windows_core::IUnknown);
impl ActivitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivitySensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for ActivitySensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs;{de386717-aeb6-4ec7-946a-d9cc19b951ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IActivitySensorReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs";
}
impl ::core::convert::From<ActivitySensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ActivitySensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ActivitySensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ActivitySensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ActivitySensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ActivitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for ActivitySensorReadingConfidence {}
impl ::core::clone::Clone for ActivitySensorReadingConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivitySensorReadingConfidence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivitySensorReadingConfidence {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivitySensorReadingConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingConfidence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensorReadingConfidence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivitySensorReadingConfidence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ActivitySensorTriggerDetails(::windows_core::IUnknown);
impl ActivitySensorTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivitySensorTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivitySensorTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorTriggerDetails {}
impl ::core::fmt::Debug for ActivitySensorTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivitySensorTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ActivitySensorTriggerDetails;{2c9e6612-b9ca-4677-b263-243297f79d3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IActivitySensorTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorTriggerDetails";
}
impl ::core::convert::From<ActivitySensorTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: ActivitySensorTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &ActivitySensorTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivitySensorTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: ActivitySensorTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivitySensorTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &ActivitySensorTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivitySensorTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivitySensorTriggerDetails {}
unsafe impl ::core::marker::Sync for ActivitySensorTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Stationary: Self = Self(2i32);
    pub const Fidgeting: Self = Self(3i32);
    pub const Walking: Self = Self(4i32);
    pub const Running: Self = Self(5i32);
    pub const InVehicle: Self = Self(6i32);
    pub const Biking: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivityType {}
impl ::core::clone::Clone for ActivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivityType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivityType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivityType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivityType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivityType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Altimeter(::windows_core::IUnknown);
impl Altimeter {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltimeterReading>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Altimeter> {
        Self::IAltimeterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Altimeter>(result__)
        })
    }
    pub fn IAltimeterStatics<R, F: FnOnce(&IAltimeterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Altimeter, IAltimeterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Altimeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Altimeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Altimeter {}
impl ::core::fmt::Debug for Altimeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Altimeter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Altimeter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Altimeter;{72f057fd-8f04-49f1-b4a7-f4e363b701a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Altimeter {
    type Vtable = IAltimeter_Vtbl;
    const IID: ::windows_core::GUID = <IAltimeter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Altimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.Altimeter";
}
impl ::core::convert::From<Altimeter> for ::windows_core::IUnknown {
    fn from(value: Altimeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Altimeter> for ::windows_core::IUnknown {
    fn from(value: &Altimeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Altimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Altimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Altimeter> for ::windows_core::IInspectable {
    fn from(value: Altimeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Altimeter> for ::windows_core::IInspectable {
    fn from(value: &Altimeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Altimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Altimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Altimeter {}
unsafe impl ::core::marker::Sync for Altimeter {}
#[repr(transparent)]
pub struct AltimeterReading(::windows_core::IUnknown);
impl AltimeterReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AltitudeChangeInMeters(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeChangeInMeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for AltimeterReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AltimeterReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AltimeterReading {}
impl ::core::fmt::Debug for AltimeterReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltimeterReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AltimeterReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReading;{fbe8ef73-7f5e-48c8-aa1a-f1f3befc1144})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AltimeterReading {
    type Vtable = IAltimeterReading_Vtbl;
    const IID: ::windows_core::GUID = <IAltimeterReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReading";
}
impl ::core::convert::From<AltimeterReading> for ::windows_core::IUnknown {
    fn from(value: AltimeterReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AltimeterReading> for ::windows_core::IUnknown {
    fn from(value: &AltimeterReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AltimeterReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AltimeterReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AltimeterReading> for ::windows_core::IInspectable {
    fn from(value: AltimeterReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AltimeterReading> for ::windows_core::IInspectable {
    fn from(value: &AltimeterReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AltimeterReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AltimeterReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AltimeterReading {}
unsafe impl ::core::marker::Sync for AltimeterReading {}
#[repr(transparent)]
pub struct AltimeterReadingChangedEventArgs(::windows_core::IUnknown);
impl AltimeterReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltimeterReading>(result__)
        }
    }
}
impl ::core::clone::Clone for AltimeterReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AltimeterReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AltimeterReadingChangedEventArgs {}
impl ::core::fmt::Debug for AltimeterReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltimeterReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AltimeterReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.AltimeterReadingChangedEventArgs;{7069d077-446d-47f7-998c-ebc23b45e4a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAltimeterReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReadingChangedEventArgs";
}
impl ::core::convert::From<AltimeterReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AltimeterReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AltimeterReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AltimeterReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AltimeterReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AltimeterReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AltimeterReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AltimeterReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AltimeterReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AltimeterReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for AltimeterReadingChangedEventArgs {}
#[repr(transparent)]
pub struct Barometer(::windows_core::IUnknown);
impl Barometer {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarometerReading>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<BarometerDataThreshold> {
        let this = &::windows_core::Interface::cast::<IBarometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarometerDataThreshold>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Barometer> {
        Self::IBarometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Barometer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Barometer>> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Barometer>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IBarometerStatics<R, F: FnOnce(&IBarometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Barometer, IBarometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarometerStatics2<R, F: FnOnce(&IBarometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Barometer, IBarometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Barometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Barometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Barometer {}
impl ::core::fmt::Debug for Barometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Barometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Barometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Barometer;{934475a8-78bf-452f-b017-f0209ce6dab4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Barometer {
    type Vtable = IBarometer_Vtbl;
    const IID: ::windows_core::GUID = <IBarometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Barometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Barometer";
}
impl ::core::convert::From<Barometer> for ::windows_core::IUnknown {
    fn from(value: Barometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Barometer> for ::windows_core::IUnknown {
    fn from(value: &Barometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Barometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Barometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Barometer> for ::windows_core::IInspectable {
    fn from(value: Barometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Barometer> for ::windows_core::IInspectable {
    fn from(value: &Barometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Barometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Barometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Barometer {}
unsafe impl ::core::marker::Sync for Barometer {}
#[repr(transparent)]
pub struct BarometerDataThreshold(::windows_core::IUnknown);
impl BarometerDataThreshold {
    pub fn Hectopascals(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Hectopascals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetHectopascals(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHectopascals)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for BarometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerDataThreshold {}
impl ::core::fmt::Debug for BarometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerDataThreshold;{076b952c-cb62-5a90-a0d1-f85e4a936394})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <IBarometerDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerDataThreshold";
}
impl ::core::convert::From<BarometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: BarometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &BarometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: BarometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &BarometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarometerDataThreshold {}
unsafe impl ::core::marker::Sync for BarometerDataThreshold {}
#[repr(transparent)]
pub struct BarometerReading(::windows_core::IUnknown);
impl BarometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn StationPressureInHectopascals(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).StationPressureInHectopascals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for BarometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerReading {}
impl ::core::fmt::Debug for BarometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReading;{f5b9d2e6-1df6-4a1a-a7ad-321d4f5db247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarometerReading {
    type Vtable = IBarometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IBarometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReading";
}
impl ::core::convert::From<BarometerReading> for ::windows_core::IUnknown {
    fn from(value: BarometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerReading> for ::windows_core::IUnknown {
    fn from(value: &BarometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarometerReading> for ::windows_core::IInspectable {
    fn from(value: BarometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerReading> for ::windows_core::IInspectable {
    fn from(value: &BarometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarometerReading {}
unsafe impl ::core::marker::Sync for BarometerReading {}
#[repr(transparent)]
pub struct BarometerReadingChangedEventArgs(::windows_core::IUnknown);
impl BarometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for BarometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for BarometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.BarometerReadingChangedEventArgs;{3d84945f-037b-404f-9bbb-6232d69543c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReadingChangedEventArgs";
}
impl ::core::convert::From<BarometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for BarometerReadingChangedEventArgs {}
#[repr(transparent)]
pub struct Compass(::windows_core::IUnknown);
impl Compass {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CompassReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<CompassDataThreshold> {
        let this = &::windows_core::Interface::cast::<ICompass4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CompassDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ICompassDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Compass> {
        Self::ICompassStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Compass>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Compass>> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Compass>>(result__)
        })
    }
    pub fn ICompassStatics<R, F: FnOnce(&ICompassStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Compass, ICompassStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICompassStatics2<R, F: FnOnce(&ICompassStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Compass, ICompassStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Compass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Compass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Compass {}
impl ::core::fmt::Debug for Compass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Compass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Compass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Compass;{292ffa94-1b45-403c-ba06-b106dba69a64})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Compass {
    type Vtable = ICompass_Vtbl;
    const IID: ::windows_core::GUID = <ICompass as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Compass {
    const NAME: &'static str = "Windows.Devices.Sensors.Compass";
}
impl ::core::convert::From<Compass> for ::windows_core::IUnknown {
    fn from(value: Compass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Compass> for ::windows_core::IUnknown {
    fn from(value: &Compass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Compass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Compass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Compass> for ::windows_core::IInspectable {
    fn from(value: Compass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Compass> for ::windows_core::IInspectable {
    fn from(value: &Compass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Compass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Compass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Compass {}
unsafe impl ::core::marker::Sync for Compass {}
#[repr(transparent)]
pub struct CompassDataThreshold(::windows_core::IUnknown);
impl CompassDataThreshold {
    pub fn Degrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Degrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CompassDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompassDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassDataThreshold {}
impl ::core::fmt::Debug for CompassDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompassDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassDataThreshold;{d15b52b3-d39d-5ec8-b2e4-f193e6ab34ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompassDataThreshold {
    type Vtable = ICompassDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <ICompassDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassDataThreshold";
}
impl ::core::convert::From<CompassDataThreshold> for ::windows_core::IUnknown {
    fn from(value: CompassDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &CompassDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompassDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompassDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompassDataThreshold> for ::windows_core::IInspectable {
    fn from(value: CompassDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &CompassDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompassDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompassDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompassDataThreshold {}
unsafe impl ::core::marker::Sync for CompassDataThreshold {}
#[repr(transparent)]
pub struct CompassReading(::windows_core::IUnknown);
impl CompassReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn HeadingMagneticNorth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HeadingMagneticNorth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadingTrueNorth(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeadingTrueNorth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn HeadingAccuracy(&self) -> ::windows_core::Result<MagnetometerAccuracy> {
        let this = &::windows_core::Interface::cast::<ICompassReadingHeadingAccuracy>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagnetometerAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).HeadingAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
}
impl ::core::clone::Clone for CompassReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompassReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassReading {}
impl ::core::fmt::Debug for CompassReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompassReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReading;{82911128-513d-4dc9-b781-5eedfbf02d0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompassReading {
    type Vtable = ICompassReading_Vtbl;
    const IID: ::windows_core::GUID = <ICompassReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReading";
}
impl ::core::convert::From<CompassReading> for ::windows_core::IUnknown {
    fn from(value: CompassReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassReading> for ::windows_core::IUnknown {
    fn from(value: &CompassReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompassReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompassReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompassReading> for ::windows_core::IInspectable {
    fn from(value: CompassReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassReading> for ::windows_core::IInspectable {
    fn from(value: &CompassReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompassReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompassReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompassReading {}
unsafe impl ::core::marker::Sync for CompassReading {}
#[repr(transparent)]
pub struct CompassReadingChangedEventArgs(::windows_core::IUnknown);
impl CompassReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CompassReading>(result__)
        }
    }
}
impl ::core::clone::Clone for CompassReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompassReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompassReadingChangedEventArgs {}
impl ::core::fmt::Debug for CompassReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompassReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompassReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.CompassReadingChangedEventArgs;{8f1549b0-e8bc-4c7e-b009-4e41df137072})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICompassReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReadingChangedEventArgs";
}
impl ::core::convert::From<CompassReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CompassReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CompassReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompassReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CompassReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompassReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CompassReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompassReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompassReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for CompassReadingChangedEventArgs {}
#[repr(transparent)]
pub struct Gyrometer(::windows_core::IUnknown);
impl Gyrometer {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GyrometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<GyrometerDataThreshold> {
        let this = &::windows_core::Interface::cast::<IGyrometer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GyrometerDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IGyrometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Gyrometer> {
        Self::IGyrometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Gyrometer>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Gyrometer>>(result__)
        })
    }
    pub fn IGyrometerStatics<R, F: FnOnce(&IGyrometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Gyrometer, IGyrometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGyrometerStatics2<R, F: FnOnce(&IGyrometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Gyrometer, IGyrometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Gyrometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Gyrometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Gyrometer {}
impl ::core::fmt::Debug for Gyrometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Gyrometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Gyrometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Gyrometer;{fdb9a9c4-84b1-4ca2-9763-9b589506c70c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Gyrometer {
    type Vtable = IGyrometer_Vtbl;
    const IID: ::windows_core::GUID = <IGyrometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Gyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Gyrometer";
}
impl ::core::convert::From<Gyrometer> for ::windows_core::IUnknown {
    fn from(value: Gyrometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gyrometer> for ::windows_core::IUnknown {
    fn from(value: &Gyrometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Gyrometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Gyrometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Gyrometer> for ::windows_core::IInspectable {
    fn from(value: Gyrometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Gyrometer> for ::windows_core::IInspectable {
    fn from(value: &Gyrometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Gyrometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Gyrometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Gyrometer {}
unsafe impl ::core::marker::Sync for Gyrometer {}
#[repr(transparent)]
pub struct GyrometerDataThreshold(::windows_core::IUnknown);
impl GyrometerDataThreshold {
    pub fn XAxisInDegreesPerSecond(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).XAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetXAxisInDegreesPerSecond(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInDegreesPerSecond(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).YAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetYAxisInDegreesPerSecond(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInDegreesPerSecond(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ZAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetZAxisInDegreesPerSecond(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZAxisInDegreesPerSecond)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for GyrometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GyrometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerDataThreshold {}
impl ::core::fmt::Debug for GyrometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GyrometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerDataThreshold;{8648b31e-6e52-5259-bbad-242a69dc38c8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <IGyrometerDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerDataThreshold";
}
impl ::core::convert::From<GyrometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: GyrometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &GyrometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GyrometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GyrometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GyrometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: GyrometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &GyrometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GyrometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GyrometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GyrometerDataThreshold {}
unsafe impl ::core::marker::Sync for GyrometerDataThreshold {}
#[repr(transparent)]
pub struct GyrometerReading(::windows_core::IUnknown);
impl GyrometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AngularVelocityX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngularVelocityX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AngularVelocityY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngularVelocityY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AngularVelocityZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngularVelocityZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for GyrometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GyrometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerReading {}
impl ::core::fmt::Debug for GyrometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GyrometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReading;{b3d6de5c-1ee4-456f-9de7-e2493b5c8e03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GyrometerReading {
    type Vtable = IGyrometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IGyrometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReading";
}
impl ::core::convert::From<GyrometerReading> for ::windows_core::IUnknown {
    fn from(value: GyrometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerReading> for ::windows_core::IUnknown {
    fn from(value: &GyrometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GyrometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GyrometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GyrometerReading> for ::windows_core::IInspectable {
    fn from(value: GyrometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerReading> for ::windows_core::IInspectable {
    fn from(value: &GyrometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GyrometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GyrometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GyrometerReading {}
unsafe impl ::core::marker::Sync for GyrometerReading {}
#[repr(transparent)]
pub struct GyrometerReadingChangedEventArgs(::windows_core::IUnknown);
impl GyrometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GyrometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for GyrometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GyrometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GyrometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for GyrometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GyrometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GyrometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.GyrometerReadingChangedEventArgs;{0fdf1895-6f9e-42ce-8d58-388c0ab8356d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGyrometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReadingChangedEventArgs";
}
impl ::core::convert::From<GyrometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GyrometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GyrometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GyrometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GyrometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GyrometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GyrometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GyrometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GyrometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for GyrometerReadingChangedEventArgs {}
#[repr(transparent)]
pub struct HingeAngleReading(::windows_core::IUnknown);
impl HingeAngleReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn AngleInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).AngleInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for HingeAngleReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HingeAngleReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleReading {}
impl ::core::fmt::Debug for HingeAngleReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HingeAngleReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleReading;{a3cd45b9-1bf1-4f65-a704-e2da04f182c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HingeAngleReading {
    type Vtable = IHingeAngleReading_Vtbl;
    const IID: ::windows_core::GUID = <IHingeAngleReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleReading";
}
impl ::core::convert::From<HingeAngleReading> for ::windows_core::IUnknown {
    fn from(value: HingeAngleReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleReading> for ::windows_core::IUnknown {
    fn from(value: &HingeAngleReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HingeAngleReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HingeAngleReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HingeAngleReading> for ::windows_core::IInspectable {
    fn from(value: HingeAngleReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleReading> for ::windows_core::IInspectable {
    fn from(value: &HingeAngleReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HingeAngleReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HingeAngleReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HingeAngleReading {}
unsafe impl ::core::marker::Sync for HingeAngleReading {}
#[repr(transparent)]
pub struct HingeAngleSensor(::windows_core::IUnknown);
impl HingeAngleSensor {
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentReadingAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReadingAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleReading>>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MinReportThresholdInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MinReportThresholdInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ReportThresholdInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThresholdInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReportThresholdInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportThresholdInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRelatedToAdjacentPanelsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(firstpanelid: Param0, secondpanelid: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRelatedToAdjacentPanelsAsync)(::windows_core::Interface::as_raw(this), firstpanelid.into_param().abi(), secondpanelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>(result__)
        })
    }
    pub fn IHingeAngleSensorStatics<R, F: FnOnce(&IHingeAngleSensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HingeAngleSensor, IHingeAngleSensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HingeAngleSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HingeAngleSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleSensor {}
impl ::core::fmt::Debug for HingeAngleSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleSensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HingeAngleSensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensor;{e9d3be02-bfdf-437f-8c29-88c77393d309})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HingeAngleSensor {
    type Vtable = IHingeAngleSensor_Vtbl;
    const IID: ::windows_core::GUID = <IHingeAngleSensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensor";
}
impl ::core::convert::From<HingeAngleSensor> for ::windows_core::IUnknown {
    fn from(value: HingeAngleSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleSensor> for ::windows_core::IUnknown {
    fn from(value: &HingeAngleSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HingeAngleSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HingeAngleSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HingeAngleSensor> for ::windows_core::IInspectable {
    fn from(value: HingeAngleSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleSensor> for ::windows_core::IInspectable {
    fn from(value: &HingeAngleSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HingeAngleSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HingeAngleSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HingeAngleSensor {}
unsafe impl ::core::marker::Sync for HingeAngleSensor {}
#[repr(transparent)]
pub struct HingeAngleSensorReadingChangedEventArgs(::windows_core::IUnknown);
impl HingeAngleSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<HingeAngleReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HingeAngleReading>(result__)
        }
    }
}
impl ::core::clone::Clone for HingeAngleSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HingeAngleSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HingeAngleSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for HingeAngleSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HingeAngleSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs;{24d9558b-fad0-42b8-a854-78923049a1ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHingeAngleSensorReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs";
}
impl ::core::convert::From<HingeAngleSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HingeAngleSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HingeAngleSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HingeAngleSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HingeAngleSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HingeAngleSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HingeAngleSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HingeAngleSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HingeAngleSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for HingeAngleSensorReadingChangedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometer {
    type Vtable = IAccelerometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf184548_2711_4da7_8098_4b82205d3c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Shaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Shaken: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShaken: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometer2 {
    type Vtable = IAccelerometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8f092ee_4964_401a_b602_220d7153c60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometer3 {
    type Vtable = IAccelerometer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87e0022a_ed80_49eb_bf8a_a4ea31e5cd84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometer4 {
    type Vtable = IAccelerometer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d373c4f_42d3_45b2_8144_ab7fb665eb59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccelerometerReadingType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometer5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometer5 {
    type Vtable = IAccelerometer5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e7e7021_def4_53a6_af43_806fd538edf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometer5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf92c1b68_6320_5577_879e_9942621c3dd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub XAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetXAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub YAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetYAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ZAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetZAxisInGForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerDeviceId {
    type Vtable = IAccelerometerDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7eac64a9_97d5_446d_ab5a_917df9b96a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerReading {
    type Vtable = IAccelerometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9fe7acb_d351_40af_8bb6_7aa9ae641fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AccelerationX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AccelerationY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AccelerationZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerReading2 {
    type Vtable = IAccelerometerReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a864aa2_15ae_4a40_be55_db58d7de7389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0095c65b_b6ac_475a_9f44_8b32d35a3f25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerShakenEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerShakenEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerStatics {
    type Vtable = IAccelerometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5e28b74_5a87_4a2d_becc_0f906ea061dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerStatics2 {
    type Vtable = IAccelerometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4c4842f_d86b_4685_b2d7_3396f798d57b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccelerometerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccelerometerStatics3 {
    type Vtable = IAccelerometerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9de218cf_455d_4cf3_8200_70e1410340f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccelerometerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: AccelerometerReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensor {
    type Vtable = IActivitySensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd7a630c_fb5f_48eb_b09b_a2708d1c61ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedActivities: usize,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorReading {
    type Vtable = IActivitySensorReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85125a96_1472_40a2_b2ae_e1ef29226c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Activity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivityType) -> ::windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivitySensorReadingConfidence) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReadingChangeReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f3c2915_d93b_47bd_960a_f20fb2f322b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangeReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorStatics {
    type Vtable = IActivitySensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa71e0e9d_ee8b_45d1_b25b_08cc0df92ab6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c9e6612_b9ca_4677_b263_243297f79d3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeter {
    type Vtable = IAltimeter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72f057fd_8f04_49f1_b4a7_f4e363b701a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeter2 {
    type Vtable = IAltimeter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9471bf9_2add_48f5_9f08_3d0c7660d938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeterReading {
    type Vtable = IAltimeterReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbe8ef73_7f5e_48c8_aa1a_f1f3befc1144);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AltitudeChangeInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeterReading2 {
    type Vtable = IAltimeterReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543a1bd9_6d0b_42b2_bd69_bc8fae0f782c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7069d077_446d_47f7_998c_ebc23b45e4a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAltimeterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAltimeterStatics {
    type Vtable = IAltimeterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9eb4d7c3_e5ac_47ce_8eef_d3718168c01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAltimeterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometer {
    type Vtable = IBarometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x934475a8_78bf_452f_b017_f0209ce6dab4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometer2 {
    type Vtable = IBarometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32bcc418_3eeb_4d04_9574_7633a8781f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometer3 {
    type Vtable = IBarometer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e35f0ea_02b5_5a04_b03d_822084863a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x076b952c_cb62_5a90_a0d1_f85e4a936394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Hectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerReading {
    type Vtable = IBarometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub StationPressureInHectopascals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerReading2 {
    type Vtable = IBarometerReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85a244eb_90c5_4875_891c_3865b4c357e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d84945f_037b_404f_9bbb_6232d69543c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerStatics {
    type Vtable = IBarometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x286b270a_02e3_4f86_84fc_fdd892b5940f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarometerStatics2 {
    type Vtable = IBarometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc6b1e7_95ff_44ac_878e_d65c8308c34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompass {
    type Vtable = ICompass_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x292ffa94_1b45_403c_ba06_b106dba69a64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompass2 {
    type Vtable = ICompass2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36f26d09_c7d7_434f_b461_979ddfc2322f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompass3 {
    type Vtable = ICompass3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa424801b_c5ea_4d45_a0ec_4b791f041a89);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompass4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompass4 {
    type Vtable = ICompass4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x291e7f11_ec32_5dcc_bfcb_0bb39eba5774);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompass4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassDataThreshold {
    type Vtable = ICompassDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Degrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassDeviceId {
    type Vtable = ICompassDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd181ca29_b085_4b1d_870a_4ff57ba74fd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassReading {
    type Vtable = ICompassReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82911128_513d_4dc9_b781_5eedfbf02d0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub HeadingMagneticNorth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeadingTrueNorth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadingTrueNorth: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassReading2 {
    type Vtable = ICompassReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb13a661e_51bb_4a12_bedd_ad47ff87d2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassReadingHeadingAccuracy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassReadingHeadingAccuracy {
    type Vtable = ICompassReadingHeadingAccuracy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe761354e_8911_40f7_9e16_6ecc7daec5de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassReadingHeadingAccuracy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeadingAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassStatics {
    type Vtable = ICompassStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9abc97df_56ec_4c25_b54d_40a68bb5b269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompassStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompassStatics2 {
    type Vtable = ICompassStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ace0ead_3baa_4990_9ce4_be0913754ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompassStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometer {
    type Vtable = IGyrometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdb9a9c4_84b1_4ca2_9763_9b589506c70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometer2 {
    type Vtable = IGyrometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63df2443_8ce8_41c3_ac44_8698810b557f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometer3 {
    type Vtable = IGyrometer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d6f88d5_8fbc_4484_914b_528adfd947b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometer4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometer4 {
    type Vtable = IGyrometer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0628a60c_4c4b_5096_94e6_c356df68bef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometer4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8648b31e_6e52_5259_bbad_242a69dc38c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub XAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetXAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub YAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetYAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetZAxisInDegreesPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerDeviceId {
    type Vtable = IGyrometerDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ee5e978_89a2_4275_9e95_7126f4708760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerReading {
    type Vtable = IGyrometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3d6de5c_1ee4_456f_9de7_e2493b5c8e03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngularVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AngularVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AngularVelocityZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerReading2 {
    type Vtable = IGyrometerReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16afe13c_2b89_44bb_822b_d1e1556ff09b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fdf1895_6f9e_42ce_8d58_388c0ab8356d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerStatics {
    type Vtable = IGyrometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83b6e7c9_e49d_4b39_86e6_cd554be4c5c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGyrometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGyrometerStatics2 {
    type Vtable = IGyrometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef83f7a1_d700_4204_9613_79c6b161df4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGyrometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHingeAngleReading {
    type Vtable = IHingeAngleReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub AngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHingeAngleSensor {
    type Vtable = IHingeAngleSensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9d3be02_bfdf_437f_8c29_88c77393d309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentReadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentReadingAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MinReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReportThresholdInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24d9558b_fad0_42b8_a854_78923049a1ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHingeAngleSensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHingeAngleSensorStatics {
    type Vtable = IHingeAngleSensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7b63910_fbb1_4123_89ce_4ea34eb0dfca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHingeAngleSensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRelatedToAdjacentPanelsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstpanelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, secondpanelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRelatedToAdjacentPanelsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometer {
    type Vtable = IInclinometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometer2 {
    type Vtable = IInclinometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x029f3393_28b2_45f8_bb16_61e86a7fae6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometer3 {
    type Vtable = IInclinometer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a095004_d765_4384_a3d7_0283f3abe6ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometer4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometer4 {
    type Vtable = IInclinometer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43852618_8fca_548e_bbf5_5c50412b6aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometer4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf80a4783_7bfe_545e_bb60_a0ebc47bd2fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PitchInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPitchInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub RollInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRollInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetYawInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerDeviceId {
    type Vtable = IInclinometerDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01e91982_41ff_4406_ae83_62210ff16fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerReading {
    type Vtable = IInclinometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f44f055_b6f6_497f_b127_1a775e501458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub PitchDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RollDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub YawDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerReading2 {
    type Vtable = IInclinometerReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f164781_e90b_4658_8915_0103e08a805a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ae91dc1_e7eb_4938_8511_ae0d6b440438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerReadingYawAccuracy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerReadingYawAccuracy {
    type Vtable = IInclinometerReadingYawAccuracy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb453e880_1fe3_4986_a257_e6ece2723949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerReadingYawAccuracy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerStatics {
    type Vtable = IInclinometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf22ec551_9c30_453a_8b49_3c3eeb33cb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerStatics2 {
    type Vtable = IInclinometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x043f9775_6a1e_499c_86e0_638c1a864b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerStatics3 {
    type Vtable = IInclinometerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd9a4280_b91a_4829_9392_abc0b6bdf2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInclinometerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInclinometerStatics4 {
    type Vtable = IInclinometerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8ba96f9_6e85_4a83_aed0_d7cdcc9856c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInclinometerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensor {
    type Vtable = ILightSensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf84c0718_0c54_47ae_922e_789f57fb03a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensor2 {
    type Vtable = ILightSensor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x486b24e8_a94c_4090_8f48_09f782a9f7d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensor3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensor3 {
    type Vtable = ILightSensor3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4876d0ff_9f4c_5f72_adbd_a3471b063c00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensor3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb160afd1_878f_5492_9f2c_33dc3ae584a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LuxPercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorDeviceId {
    type Vtable = ILightSensorDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fee49f8_0afb_4f51_87f0_6c26375ce94f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorReading {
    type Vtable = ILightSensorReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffdf6300_227c_4d2b_b302_fc0142485c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IlluminanceInLux: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorReading2 {
    type Vtable = ILightSensorReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7512185_44a3_44c9_8190_9ef6de0a8a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3a2f4cf_258b_420c_b8ab_8edd601ecf50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorStatics {
    type Vtable = ILightSensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45db8c84_c3a8_471e_9a53_6457fad87c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILightSensorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILightSensorStatics2 {
    type Vtable = ILightSensorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ec0a650_ddc6_40ab_ace3_ec3359d42c51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILightSensorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometer {
    type Vtable = IMagnetometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometer2 {
    type Vtable = IMagnetometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4656c85_26f6_444b_a9e2_a23f966cd368);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometer3 {
    type Vtable = IMagnetometer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe93db7c_a625_48ef_acf7_fac104832671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometer4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometer4 {
    type Vtable = IMagnetometer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfb17901_3e0f_508f_b24b_f2bb75015f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometer4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerDataThreshold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd177cb01_9063_5fa5_b596_b445e9dc3401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub XAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetXAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub YAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetYAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub ZAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetZAxisMicroteslas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerDeviceId {
    type Vtable = IMagnetometerDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58b498c2_7e4b_404c_9fc5_5de8b40ebae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerReading {
    type Vtable = IMagnetometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c2cc40d_ebfd_4e5c_bb11_afc29b3cae61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub MagneticFieldX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MagneticFieldY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MagneticFieldZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub DirectionalAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerReading2 {
    type Vtable = IMagnetometerReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4c95c61_61d9_404b_a328_066f177a1409);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerStatics {
    type Vtable = IMagnetometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x853c64cc_0698_4dda_a6df_9cb9cc4ab40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagnetometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMagnetometerStatics2 {
    type Vtable = IMagnetometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c0819f0_ffc6_4f89_a06f_18fa10792933);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagnetometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensor {
    type Vtable = IOrientationSensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e354635_cf6b_4c63_abd8_10252b0bf6ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensor2 {
    type Vtable = IOrientationSensor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d924cf9_2f1f_49c9_8042_4a1813d67760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorReadingType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensor3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensor3 {
    type Vtable = IOrientationSensor3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cce578d_646b_48c5_b7ee_44fdc4c6aafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensor3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorDeviceId {
    type Vtable = IOrientationSensorDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a69b648_4c29_49ec_b28f_ea1d117b66f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorReading {
    type Vtable = IOrientationSensorReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4756c993_6595_4897_bcc6_d537ee757564);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub RotationMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Quaternion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReading2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorReading2 {
    type Vtable = IOrientationSensorReading2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00576e5f_49f8_4c05_9e07_24fac79408c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReading2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PerformanceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PerformanceCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x012c1186_c3ba_46bc_ae65_7a98996cbfb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorReadingYawAccuracy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorReadingYawAccuracy {
    type Vtable = IOrientationSensorReadingYawAccuracy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1ac9824_3f5a_49a2_bc7b_1180bc38cd2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorReadingYawAccuracy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub YawAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagnetometerAccuracy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorStatics {
    type Vtable = IOrientationSensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10ef8712_fb4c_428a_898b_2765e409e669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorStatics2 {
    type Vtable = IOrientationSensorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59da0d0b_d40a_4c71_9276_8a272a0a6619);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorStatics3 {
    type Vtable = IOrientationSensorStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd82ce920_2777_40ff_9f59_d654b085f12f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientationSensorStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientationSensorStatics4 {
    type Vtable = IOrientationSensorStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa67feb55_2c85_4b28_a0fe_58c4b20495f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientationSensorStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometer {
    type Vtable = IPedometer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a1e013d_3d98_45f8_8920_8e4ecaca5f97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometer2 {
    type Vtable = IPedometer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5a406df_2b81_4add_b2ff_77ab6c98ba19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometer2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentReadings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentReadings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerDataThresholdFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometerDataThresholdFactory {
    type Vtable = IPedometerDataThresholdFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbad8f50_7a54_466b_9010_77a162fca5d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerDataThresholdFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensor: ::windows_core::RawPtr, stepgoal: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometerReading {
    type Vtable = IPedometerReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2245dcf4_a8e1_432f_896a_be0dd9b02d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StepKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PedometerStepKind) -> ::windows_core::HRESULT,
    pub CumulativeSteps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation")]
    pub CumulativeStepsDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CumulativeStepsDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf855e47e_abbc_4456_86a8_25cf2b333742);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometerStatics {
    type Vtable = IPedometerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82980a2f_4083_4dfb_b411_938ea0f4b946);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPedometerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPedometerStatics2 {
    type Vtable = IPedometerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f5c6bb_ce0e_4133_b47e_8627ea72f677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPedometerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensor {
    type Vtable = IProximitySensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54c076b8_ecfb_4944_b928_74fc504d47ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxDistanceInMillimeters: usize,
    #[cfg(feature = "Foundation")]
    pub MinDistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinDistanceInMillimeters: usize,
    pub GetCurrentReading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReadingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReadingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDisplayOnOffController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDisplayOnOffController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorDataThresholdFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensorDataThresholdFactory {
    type Vtable = IProximitySensorDataThresholdFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x905ac121_6d27_4ad3_9db5_6467f2a5ad9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorDataThresholdFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorReading(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensorReading {
    type Vtable = IProximitySensorReading_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71228d59_132d_4d5f_8ff9_2f0db8751ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReading_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub IsDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DistanceInMillimeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistanceInMillimeters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorReadingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorReadingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensorStatics {
    type Vtable = IProximitySensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29186649_6269_4e57_a5ad_82be80813392);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sensorid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximitySensorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProximitySensorStatics2 {
    type Vtable = IProximitySensorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbf473ae_e9ca_422f_ad67_4c3d25df350c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximitySensorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
#[repr(transparent)]
pub struct ISensorDataThreshold(::windows_core::IUnknown);
impl ISensorDataThreshold {}
impl ::core::convert::From<ISensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: ISensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &ISensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: ISensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &ISensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISensorDataThreshold {}
impl ::core::fmt::Debug for ISensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISensorDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISensorDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{54daec61-fe4b-4e07-b260-3a4cdfbe396e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISensorDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThreshold_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9106f1b7_e88d_48b1_bc90_619c7b349391);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SensorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SensorType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorQuaternion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISensorQuaternion {
    type Vtable = ISensorQuaternion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorQuaternion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub W: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub X: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Z: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorRotationMatrix(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a3d5a67_22f4_4392_9538_65d0bd064aa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorRotationMatrix_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub M11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M21: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M23: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M31: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub M33: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ff53856_214a_4dee_a3f9_616f1ab06ffd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OrientationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOrientationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensor2 {
    type Vtable = ISimpleOrientationSensor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa277a798_8870_453e_8bd6_b8f5d8d7941b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorDeviceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensorDeviceId {
    type Vtable = ISimpleOrientationSensorDeviceId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbc00acb_3b76_41f6_8091_30efe646d3cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorDeviceId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SimpleOrientation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensorStatics {
    type Vtable = ISimpleOrientationSensorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72ed066f_70aa_40c6_9b1b_3433f7459b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISimpleOrientationSensorStatics2 {
    type Vtable = ISimpleOrientationSensorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x848f9c7f_b138_4e11_8910_a2a2a3b56d83);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleOrientationSensorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(transparent)]
pub struct Inclinometer(::windows_core::IUnknown);
impl Inclinometer {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InclinometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows_core::Result<SensorReadingType> {
        let this = &::windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SensorReadingType>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SensorReadingType>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<InclinometerDataThreshold> {
        let this = &::windows_core::Interface::cast::<IInclinometer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InclinometerDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IInclinometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(::windows_core::Interface::as_raw(this), sensorreadingtype, result__.as_mut_ptr()).from_abi::<Inclinometer>(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), readingtype, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Inclinometer>>(result__)
        })
    }
    pub fn IInclinometerStatics<R, F: FnOnce(&IInclinometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Inclinometer, IInclinometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics2<R, F: FnOnce(&IInclinometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Inclinometer, IInclinometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics3<R, F: FnOnce(&IInclinometerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Inclinometer, IInclinometerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInclinometerStatics4<R, F: FnOnce(&IInclinometerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Inclinometer, IInclinometerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Inclinometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Inclinometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Inclinometer {}
impl ::core::fmt::Debug for Inclinometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Inclinometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Inclinometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Inclinometer;{2648ca6f-2286-406f-9161-f0c4bd806ebf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Inclinometer {
    type Vtable = IInclinometer_Vtbl;
    const IID: ::windows_core::GUID = <IInclinometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Inclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Inclinometer";
}
impl ::core::convert::From<Inclinometer> for ::windows_core::IUnknown {
    fn from(value: Inclinometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inclinometer> for ::windows_core::IUnknown {
    fn from(value: &Inclinometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Inclinometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Inclinometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Inclinometer> for ::windows_core::IInspectable {
    fn from(value: Inclinometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inclinometer> for ::windows_core::IInspectable {
    fn from(value: &Inclinometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Inclinometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Inclinometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Inclinometer {}
unsafe impl ::core::marker::Sync for Inclinometer {}
#[repr(transparent)]
pub struct InclinometerDataThreshold(::windows_core::IUnknown);
impl InclinometerDataThreshold {
    pub fn PitchInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PitchInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPitchInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPitchInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RollInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RollInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRollInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRollInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YawInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).YawInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetYawInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYawInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InclinometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InclinometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerDataThreshold {}
impl ::core::fmt::Debug for InclinometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InclinometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerDataThreshold;{f80a4783-7bfe-545e-bb60-a0ebc47bd2fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <IInclinometerDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerDataThreshold";
}
impl ::core::convert::From<InclinometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: InclinometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &InclinometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InclinometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InclinometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InclinometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: InclinometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &InclinometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InclinometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InclinometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InclinometerDataThreshold {}
unsafe impl ::core::marker::Sync for InclinometerDataThreshold {}
#[repr(transparent)]
pub struct InclinometerReading(::windows_core::IUnknown);
impl InclinometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn PitchDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PitchDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RollDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RollDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn YawDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).YawDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows_core::Result<MagnetometerAccuracy> {
        let this = &::windows_core::Interface::cast::<IInclinometerReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagnetometerAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).YawAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
}
impl ::core::clone::Clone for InclinometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InclinometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerReading {}
impl ::core::fmt::Debug for InclinometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InclinometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReading;{9f44f055-b6f6-497f-b127-1a775e501458})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InclinometerReading {
    type Vtable = IInclinometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IInclinometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReading";
}
impl ::core::convert::From<InclinometerReading> for ::windows_core::IUnknown {
    fn from(value: InclinometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerReading> for ::windows_core::IUnknown {
    fn from(value: &InclinometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InclinometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InclinometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InclinometerReading> for ::windows_core::IInspectable {
    fn from(value: InclinometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerReading> for ::windows_core::IInspectable {
    fn from(value: &InclinometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InclinometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InclinometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InclinometerReading {}
unsafe impl ::core::marker::Sync for InclinometerReading {}
#[repr(transparent)]
pub struct InclinometerReadingChangedEventArgs(::windows_core::IUnknown);
impl InclinometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InclinometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for InclinometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InclinometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InclinometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for InclinometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InclinometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InclinometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.InclinometerReadingChangedEventArgs;{4ae91dc1-e7eb-4938-8511-ae0d6b440438})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInclinometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReadingChangedEventArgs";
}
impl ::core::convert::From<InclinometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: InclinometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &InclinometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InclinometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: InclinometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InclinometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &InclinometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InclinometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InclinometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for InclinometerReadingChangedEventArgs {}
#[repr(transparent)]
pub struct LightSensor(::windows_core::IUnknown);
impl LightSensor {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LightSensorReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<LightSensorDataThreshold> {
        let this = &::windows_core::Interface::cast::<ILightSensor3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LightSensorDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILightSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<LightSensor> {
        Self::ILightSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LightSensor>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LightSensor>> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<LightSensor>>(result__)
        })
    }
    pub fn ILightSensorStatics<R, F: FnOnce(&ILightSensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LightSensor, ILightSensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILightSensorStatics2<R, F: FnOnce(&ILightSensorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LightSensor, ILightSensorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LightSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LightSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensor {}
impl ::core::fmt::Debug for LightSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LightSensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensor;{f84c0718-0c54-47ae-922e-789f57fb03a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LightSensor {
    type Vtable = ILightSensor_Vtbl;
    const IID: ::windows_core::GUID = <ILightSensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensor";
}
impl ::core::convert::From<LightSensor> for ::windows_core::IUnknown {
    fn from(value: LightSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensor> for ::windows_core::IUnknown {
    fn from(value: &LightSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LightSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LightSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LightSensor> for ::windows_core::IInspectable {
    fn from(value: LightSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensor> for ::windows_core::IInspectable {
    fn from(value: &LightSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LightSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LightSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LightSensor {}
unsafe impl ::core::marker::Sync for LightSensor {}
#[repr(transparent)]
pub struct LightSensorDataThreshold(::windows_core::IUnknown);
impl LightSensorDataThreshold {
    pub fn LuxPercentage(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).LuxPercentage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetLuxPercentage(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLuxPercentage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AbsoluteLux(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteLux)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetAbsoluteLux(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAbsoluteLux)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LightSensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LightSensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorDataThreshold {}
impl ::core::fmt::Debug for LightSensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LightSensorDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorDataThreshold;{b160afd1-878f-5492-9f2c-33dc3ae584a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <ILightSensorDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorDataThreshold";
}
impl ::core::convert::From<LightSensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: LightSensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &LightSensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LightSensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LightSensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LightSensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: LightSensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &LightSensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LightSensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LightSensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LightSensorDataThreshold {}
unsafe impl ::core::marker::Sync for LightSensorDataThreshold {}
#[repr(transparent)]
pub struct LightSensorReading(::windows_core::IUnknown);
impl LightSensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IlluminanceInLux(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).IlluminanceInLux)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for LightSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LightSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorReading {}
impl ::core::fmt::Debug for LightSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LightSensorReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReading;{ffdf6300-227c-4d2b-b302-fc0142485c68})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LightSensorReading {
    type Vtable = ILightSensorReading_Vtbl;
    const IID: ::windows_core::GUID = <ILightSensorReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReading";
}
impl ::core::convert::From<LightSensorReading> for ::windows_core::IUnknown {
    fn from(value: LightSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorReading> for ::windows_core::IUnknown {
    fn from(value: &LightSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LightSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LightSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LightSensorReading> for ::windows_core::IInspectable {
    fn from(value: LightSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorReading> for ::windows_core::IInspectable {
    fn from(value: &LightSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LightSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LightSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LightSensorReading {}
unsafe impl ::core::marker::Sync for LightSensorReading {}
#[repr(transparent)]
pub struct LightSensorReadingChangedEventArgs(::windows_core::IUnknown);
impl LightSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LightSensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for LightSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LightSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LightSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for LightSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LightSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LightSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.LightSensorReadingChangedEventArgs;{a3a2f4cf-258b-420c-b8ab-8edd601ecf50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILightSensorReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReadingChangedEventArgs";
}
impl ::core::convert::From<LightSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LightSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LightSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LightSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LightSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LightSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LightSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LightSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LightSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for LightSensorReadingChangedEventArgs {}
#[repr(transparent)]
pub struct Magnetometer(::windows_core::IUnknown);
impl Magnetometer {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportThreshold(&self) -> ::windows_core::Result<MagnetometerDataThreshold> {
        let this = &::windows_core::Interface::cast::<IMagnetometer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerDataThreshold>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMagnetometerDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<Magnetometer> {
        Self::IMagnetometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Magnetometer>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Magnetometer>>(result__)
        })
    }
    pub fn IMagnetometerStatics<R, F: FnOnce(&IMagnetometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Magnetometer, IMagnetometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMagnetometerStatics2<R, F: FnOnce(&IMagnetometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Magnetometer, IMagnetometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Magnetometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Magnetometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Magnetometer {}
impl ::core::fmt::Debug for Magnetometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Magnetometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Magnetometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Magnetometer;{484f626e-d3c9-4111-b3f6-2cf1faa418d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Magnetometer {
    type Vtable = IMagnetometer_Vtbl;
    const IID: ::windows_core::GUID = <IMagnetometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Magnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Magnetometer";
}
impl ::core::convert::From<Magnetometer> for ::windows_core::IUnknown {
    fn from(value: Magnetometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Magnetometer> for ::windows_core::IUnknown {
    fn from(value: &Magnetometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Magnetometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Magnetometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Magnetometer> for ::windows_core::IInspectable {
    fn from(value: Magnetometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Magnetometer> for ::windows_core::IInspectable {
    fn from(value: &Magnetometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Magnetometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Magnetometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Magnetometer {}
unsafe impl ::core::marker::Sync for Magnetometer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl ::core::marker::Copy for MagnetometerAccuracy {}
impl ::core::clone::Clone for MagnetometerAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagnetometerAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MagnetometerAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagnetometerAccuracy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.MagnetometerAccuracy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct MagnetometerDataThreshold(::windows_core::IUnknown);
impl MagnetometerDataThreshold {
    pub fn XAxisMicroteslas(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).XAxisMicroteslas)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetXAxisMicroteslas(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXAxisMicroteslas)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisMicroteslas(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).YAxisMicroteslas)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetYAxisMicroteslas(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetYAxisMicroteslas)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisMicroteslas(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).ZAxisMicroteslas)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetZAxisMicroteslas(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZAxisMicroteslas)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MagnetometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagnetometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerDataThreshold {}
impl ::core::fmt::Debug for MagnetometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagnetometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerDataThreshold;{d177cb01-9063-5fa5-b596-b445e9dc3401})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <IMagnetometerDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerDataThreshold";
}
impl ::core::convert::From<MagnetometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: MagnetometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &MagnetometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagnetometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagnetometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagnetometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: MagnetometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &MagnetometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagnetometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagnetometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagnetometerDataThreshold {}
unsafe impl ::core::marker::Sync for MagnetometerDataThreshold {}
#[repr(transparent)]
pub struct MagnetometerReading(::windows_core::IUnknown);
impl MagnetometerReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn MagneticFieldX(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MagneticFieldX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MagneticFieldY(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MagneticFieldY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MagneticFieldZ(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MagneticFieldZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn DirectionalAccuracy(&self) -> ::windows_core::Result<MagnetometerAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagnetometerAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).DirectionalAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MagnetometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagnetometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerReading {}
impl ::core::fmt::Debug for MagnetometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagnetometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReading;{0c2cc40d-ebfd-4e5c-bb11-afc29b3cae61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagnetometerReading {
    type Vtable = IMagnetometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IMagnetometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReading";
}
impl ::core::convert::From<MagnetometerReading> for ::windows_core::IUnknown {
    fn from(value: MagnetometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerReading> for ::windows_core::IUnknown {
    fn from(value: &MagnetometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagnetometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagnetometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagnetometerReading> for ::windows_core::IInspectable {
    fn from(value: MagnetometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerReading> for ::windows_core::IInspectable {
    fn from(value: &MagnetometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagnetometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagnetometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagnetometerReading {}
unsafe impl ::core::marker::Sync for MagnetometerReading {}
#[repr(transparent)]
pub struct MagnetometerReadingChangedEventArgs(::windows_core::IUnknown);
impl MagnetometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for MagnetometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagnetometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagnetometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for MagnetometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagnetometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MagnetometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs;{17eae872-2eb9-4ee7-8ad0-3127537d949b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMagnetometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs";
}
impl ::core::convert::From<MagnetometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: MagnetometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &MagnetometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagnetometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: MagnetometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagnetometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &MagnetometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MagnetometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagnetometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for MagnetometerReadingChangedEventArgs {}
#[repr(transparent)]
pub struct OrientationSensor(::windows_core::IUnknown);
impl OrientationSensor {
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OrientationSensorReading>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn ReadingType(&self) -> ::windows_core::Result<SensorReadingType> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SensorReadingType>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SensorReadingType>(result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReportLatency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportLatency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultForRelativeReadings() -> ::windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> ::windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(::windows_core::Interface::as_raw(this), sensorreadingtype, result__.as_mut_ptr()).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal)(::windows_core::Interface::as_raw(this), sensorreadingtype, optimizationgoal, result__.as_mut_ptr()).from_abi::<OrientationSensor>(result__)
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), readingtype, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal)(::windows_core::Interface::as_raw(this), readingtype, optimizationgoal, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<OrientationSensor>>(result__)
        })
    }
    pub fn IOrientationSensorStatics<R, F: FnOnce(&IOrientationSensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OrientationSensor, IOrientationSensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics2<R, F: FnOnce(&IOrientationSensorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OrientationSensor, IOrientationSensorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics3<R, F: FnOnce(&IOrientationSensorStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OrientationSensor, IOrientationSensorStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IOrientationSensorStatics4<R, F: FnOnce(&IOrientationSensorStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<OrientationSensor, IOrientationSensorStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OrientationSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensor {}
impl ::core::fmt::Debug for OrientationSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OrientationSensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensor;{5e354635-cf6b-4c63-abd8-10252b0bf6ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OrientationSensor {
    type Vtable = IOrientationSensor_Vtbl;
    const IID: ::windows_core::GUID = <IOrientationSensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensor";
}
impl ::core::convert::From<OrientationSensor> for ::windows_core::IUnknown {
    fn from(value: OrientationSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensor> for ::windows_core::IUnknown {
    fn from(value: &OrientationSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OrientationSensor> for ::windows_core::IInspectable {
    fn from(value: OrientationSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensor> for ::windows_core::IInspectable {
    fn from(value: &OrientationSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OrientationSensor {}
unsafe impl ::core::marker::Sync for OrientationSensor {}
#[repr(transparent)]
pub struct OrientationSensorReading(::windows_core::IUnknown);
impl OrientationSensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn RotationMatrix(&self) -> ::windows_core::Result<SensorRotationMatrix> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationMatrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SensorRotationMatrix>(result__)
        }
    }
    pub fn Quaternion(&self) -> ::windows_core::Result<SensorQuaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Quaternion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SensorQuaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PerformanceCount(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn YawAccuracy(&self) -> ::windows_core::Result<MagnetometerAccuracy> {
        let this = &::windows_core::Interface::cast::<IOrientationSensorReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MagnetometerAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).YawAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MagnetometerAccuracy>(result__)
        }
    }
}
impl ::core::clone::Clone for OrientationSensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OrientationSensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensorReading {}
impl ::core::fmt::Debug for OrientationSensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensorReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OrientationSensorReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReading;{4756c993-6595-4897-bcc6-d537ee757564})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OrientationSensorReading {
    type Vtable = IOrientationSensorReading_Vtbl;
    const IID: ::windows_core::GUID = <IOrientationSensorReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReading";
}
impl ::core::convert::From<OrientationSensorReading> for ::windows_core::IUnknown {
    fn from(value: OrientationSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensorReading> for ::windows_core::IUnknown {
    fn from(value: &OrientationSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OrientationSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OrientationSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OrientationSensorReading> for ::windows_core::IInspectable {
    fn from(value: OrientationSensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensorReading> for ::windows_core::IInspectable {
    fn from(value: &OrientationSensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OrientationSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OrientationSensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OrientationSensorReading {}
unsafe impl ::core::marker::Sync for OrientationSensorReading {}
#[repr(transparent)]
pub struct OrientationSensorReadingChangedEventArgs(::windows_core::IUnknown);
impl OrientationSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<OrientationSensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for OrientationSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OrientationSensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientationSensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for OrientationSensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationSensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OrientationSensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs;{012c1186-c3ba-46bc-ae65-7a98996cbfb8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IOrientationSensorReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs";
}
impl ::core::convert::From<OrientationSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: OrientationSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &OrientationSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OrientationSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: OrientationSensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientationSensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &OrientationSensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OrientationSensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OrientationSensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for OrientationSensorReadingChangedEventArgs {}
#[repr(transparent)]
pub struct Pedometer(::windows_core::IUnknown);
impl Pedometer {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PowerInMilliwatts(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).PowerInMilliwatts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentReadings(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>> {
        let this = &::windows_core::Interface::cast::<IPedometer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReadings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Pedometer>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Pedometer>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(fromtime: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemHistoryAsync)(::windows_core::Interface::as_raw(this), fromtime.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>>(fromtime: Param0, duration: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(::windows_core::Interface::as_raw(this), fromtime.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<'a, Param0: ::windows_core::IntoParam<'a, SensorDataThresholdTriggerDetails>>(triggerdetails: Param0) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>> {
        Self::IPedometerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(::windows_core::Interface::as_raw(this), triggerdetails.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PedometerReading>>(result__)
        })
    }
    pub fn IPedometerStatics<R, F: FnOnce(&IPedometerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Pedometer, IPedometerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPedometerStatics2<R, F: FnOnce(&IPedometerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Pedometer, IPedometerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Pedometer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Pedometer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Pedometer {}
impl ::core::fmt::Debug for Pedometer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Pedometer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Pedometer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.Pedometer;{9a1e013d-3d98-45f8-8920-8e4ecaca5f97})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Pedometer {
    type Vtable = IPedometer_Vtbl;
    const IID: ::windows_core::GUID = <IPedometer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Pedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Pedometer";
}
impl ::core::convert::From<Pedometer> for ::windows_core::IUnknown {
    fn from(value: Pedometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pedometer> for ::windows_core::IUnknown {
    fn from(value: &Pedometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Pedometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Pedometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Pedometer> for ::windows_core::IInspectable {
    fn from(value: Pedometer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pedometer> for ::windows_core::IInspectable {
    fn from(value: &Pedometer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Pedometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Pedometer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Pedometer {}
unsafe impl ::core::marker::Sync for Pedometer {}
#[repr(transparent)]
pub struct PedometerDataThreshold(::windows_core::IUnknown);
impl PedometerDataThreshold {
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, Pedometer>>(sensor: Param0, stepgoal: i32) -> ::windows_core::Result<PedometerDataThreshold> {
        Self::IPedometerDataThresholdFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), sensor.into_param().abi(), stepgoal, result__.as_mut_ptr()).from_abi::<PedometerDataThreshold>(result__)
        })
    }
    pub fn IPedometerDataThresholdFactory<R, F: FnOnce(&IPedometerDataThresholdFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PedometerDataThreshold, IPedometerDataThresholdFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PedometerDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PedometerDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerDataThreshold {}
impl ::core::fmt::Debug for PedometerDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PedometerDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PedometerDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <ISensorDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PedometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerDataThreshold";
}
impl ::core::convert::From<PedometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: PedometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &PedometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PedometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: PedometerDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &PedometerDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PedometerDataThreshold> for ISensorDataThreshold {
    type Error = ::windows_core::Error;
    fn try_from(value: PedometerDataThreshold) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PedometerDataThreshold> for ISensorDataThreshold {
    type Error = ::windows_core::Error;
    fn try_from(value: &PedometerDataThreshold) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISensorDataThreshold> for PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ISensorDataThreshold> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISensorDataThreshold> for &PedometerDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ISensorDataThreshold> {
        ::core::convert::TryInto::<ISensorDataThreshold>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PedometerDataThreshold {}
unsafe impl ::core::marker::Sync for PedometerDataThreshold {}
#[repr(transparent)]
pub struct PedometerReading(::windows_core::IUnknown);
impl PedometerReading {
    pub fn StepKind(&self) -> ::windows_core::Result<PedometerStepKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PedometerStepKind>::zeroed();
            (::windows_core::Interface::vtable(this).StepKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PedometerStepKind>(result__)
        }
    }
    pub fn CumulativeSteps(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CumulativeSteps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CumulativeStepsDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).CumulativeStepsDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PedometerReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PedometerReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerReading {}
impl ::core::fmt::Debug for PedometerReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PedometerReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReading;{2245dcf4-a8e1-432f-896a-be0dd9b02d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PedometerReading {
    type Vtable = IPedometerReading_Vtbl;
    const IID: ::windows_core::GUID = <IPedometerReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReading";
}
impl ::core::convert::From<PedometerReading> for ::windows_core::IUnknown {
    fn from(value: PedometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerReading> for ::windows_core::IUnknown {
    fn from(value: &PedometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PedometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PedometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PedometerReading> for ::windows_core::IInspectable {
    fn from(value: PedometerReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerReading> for ::windows_core::IInspectable {
    fn from(value: &PedometerReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PedometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PedometerReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PedometerReading {}
unsafe impl ::core::marker::Sync for PedometerReading {}
#[repr(transparent)]
pub struct PedometerReadingChangedEventArgs(::windows_core::IUnknown);
impl PedometerReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<PedometerReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PedometerReading>(result__)
        }
    }
}
impl ::core::clone::Clone for PedometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PedometerReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PedometerReadingChangedEventArgs {}
impl ::core::fmt::Debug for PedometerReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PedometerReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.PedometerReadingChangedEventArgs;{f855e47e-abbc-4456-86a8-25cf2b333742})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPedometerReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReadingChangedEventArgs";
}
impl ::core::convert::From<PedometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PedometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PedometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PedometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PedometerReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PedometerReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PedometerReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PedometerReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PedometerReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for PedometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl ::core::marker::Copy for PedometerStepKind {}
impl ::core::clone::Clone for PedometerStepKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PedometerStepKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PedometerStepKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PedometerStepKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PedometerStepKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PedometerStepKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.PedometerStepKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProximitySensor(::windows_core::IUnknown);
impl ProximitySensor {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxDistanceInMillimeters(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxDistanceInMillimeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinDistanceInMillimeters(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinDistanceInMillimeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    pub fn GetCurrentReading(&self) -> ::windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentReading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProximitySensorReading>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReadingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReadingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDisplayOnOffController(&self) -> ::windows_core::Result<ProximitySensorDisplayOnOffController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDisplayOnOffController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProximitySensorDisplayOnOffController>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sensorid: Param0) -> ::windows_core::Result<ProximitySensor> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), sensorid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProximitySensor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<'a, Param0: ::windows_core::IntoParam<'a, SensorDataThresholdTriggerDetails>>(triggerdetails: Param0) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>> {
        Self::IProximitySensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(::windows_core::Interface::as_raw(this), triggerdetails.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>(result__)
        })
    }
    pub fn IProximitySensorStatics<R, F: FnOnce(&IProximitySensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProximitySensor, IProximitySensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IProximitySensorStatics2<R, F: FnOnce(&IProximitySensorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProximitySensor, IProximitySensorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProximitySensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximitySensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensor {}
impl ::core::fmt::Debug for ProximitySensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProximitySensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensor;{54c076b8-ecfb-4944-b928-74fc504d47ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProximitySensor {
    type Vtable = IProximitySensor_Vtbl;
    const IID: ::windows_core::GUID = <IProximitySensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensor";
}
impl ::core::convert::From<ProximitySensor> for ::windows_core::IUnknown {
    fn from(value: ProximitySensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensor> for ::windows_core::IUnknown {
    fn from(value: &ProximitySensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProximitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProximitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximitySensor> for ::windows_core::IInspectable {
    fn from(value: ProximitySensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensor> for ::windows_core::IInspectable {
    fn from(value: &ProximitySensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProximitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProximitySensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximitySensor {}
unsafe impl ::core::marker::Sync for ProximitySensor {}
#[repr(transparent)]
pub struct ProximitySensorDataThreshold(::windows_core::IUnknown);
impl ProximitySensorDataThreshold {
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ProximitySensor>>(sensor: Param0) -> ::windows_core::Result<ProximitySensorDataThreshold> {
        Self::IProximitySensorDataThresholdFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), sensor.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProximitySensorDataThreshold>(result__)
        })
    }
    pub fn IProximitySensorDataThresholdFactory<R, F: FnOnce(&IProximitySensorDataThresholdFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProximitySensorDataThreshold, IProximitySensorDataThresholdFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProximitySensorDataThreshold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximitySensorDataThreshold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorDataThreshold {}
impl ::core::fmt::Debug for ProximitySensorDataThreshold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorDataThreshold").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProximitySensorDataThreshold {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDataThreshold;{54daec61-fe4b-4e07-b260-3a4cdfbe396e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProximitySensorDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
    const IID: ::windows_core::GUID = <ISensorDataThreshold as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProximitySensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDataThreshold";
}
impl ::core::convert::From<ProximitySensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: ProximitySensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorDataThreshold> for ::windows_core::IUnknown {
    fn from(value: &ProximitySensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximitySensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: ProximitySensorDataThreshold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorDataThreshold> for ::windows_core::IInspectable {
    fn from(value: &ProximitySensorDataThreshold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProximitySensorDataThreshold> for ISensorDataThreshold {
    type Error = ::windows_core::Error;
    fn try_from(value: ProximitySensorDataThreshold) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProximitySensorDataThreshold> for ISensorDataThreshold {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProximitySensorDataThreshold) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISensorDataThreshold> for ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ISensorDataThreshold> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISensorDataThreshold> for &ProximitySensorDataThreshold {
    fn into_param(self) -> ::windows_core::Param<'a, ISensorDataThreshold> {
        ::core::convert::TryInto::<ISensorDataThreshold>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProximitySensorDataThreshold {}
unsafe impl ::core::marker::Sync for ProximitySensorDataThreshold {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct ProximitySensorDisplayOnOffController(::windows_core::IUnknown);
#[cfg(feature = "Foundation")]
impl ProximitySensorDisplayOnOffController {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProximitySensorDisplayOnOffController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ProximitySensorDisplayOnOffController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ProximitySensorDisplayOnOffController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorDisplayOnOffController").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::RuntimeType for ProximitySensorDisplayOnOffController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorDisplayOnOffController;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows_core::Interface for ProximitySensorDisplayOnOffController {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
    const IID: ::windows_core::GUID = <super::super::Foundation::IClosable as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ProximitySensorDisplayOnOffController {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDisplayOnOffController";
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<ProximitySensorDisplayOnOffController> for ::windows_core::IUnknown {
    fn from(value: ProximitySensorDisplayOnOffController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&ProximitySensorDisplayOnOffController> for ::windows_core::IUnknown {
    fn from(value: &ProximitySensorDisplayOnOffController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<ProximitySensorDisplayOnOffController> for ::windows_core::IInspectable {
    fn from(value: ProximitySensorDisplayOnOffController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&ProximitySensorDisplayOnOffController> for ::windows_core::IInspectable {
    fn from(value: &ProximitySensorDisplayOnOffController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ProximitySensorDisplayOnOffController> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ProximitySensorDisplayOnOffController) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ProximitySensorDisplayOnOffController> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProximitySensorDisplayOnOffController) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &ProximitySensorDisplayOnOffController {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for ProximitySensorDisplayOnOffController {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for ProximitySensorDisplayOnOffController {}
#[repr(transparent)]
pub struct ProximitySensorReading(::windows_core::IUnknown);
impl ProximitySensorReading {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IsDetected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDetected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DistanceInMillimeters(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DistanceInMillimeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for ProximitySensorReading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximitySensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorReading {}
impl ::core::fmt::Debug for ProximitySensorReading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorReading").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProximitySensorReading {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReading;{71228d59-132d-4d5f-8ff9-2f0db8751ced})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProximitySensorReading {
    type Vtable = IProximitySensorReading_Vtbl;
    const IID: ::windows_core::GUID = <IProximitySensorReading as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReading";
}
impl ::core::convert::From<ProximitySensorReading> for ::windows_core::IUnknown {
    fn from(value: ProximitySensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorReading> for ::windows_core::IUnknown {
    fn from(value: &ProximitySensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProximitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProximitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximitySensorReading> for ::windows_core::IInspectable {
    fn from(value: ProximitySensorReading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorReading> for ::windows_core::IInspectable {
    fn from(value: &ProximitySensorReading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProximitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProximitySensorReading {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximitySensorReading {}
unsafe impl ::core::marker::Sync for ProximitySensorReading {}
#[repr(transparent)]
pub struct ProximitySensorReadingChangedEventArgs(::windows_core::IUnknown);
impl ProximitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> ::windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Reading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProximitySensorReading>(result__)
        }
    }
}
impl ::core::clone::Clone for ProximitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximitySensorReadingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximitySensorReadingChangedEventArgs {}
impl ::core::fmt::Debug for ProximitySensorReadingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximitySensorReadingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProximitySensorReadingChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs;{cfc2f366-c3e8-40fd-8cc3-67e289004938})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProximitySensorReadingChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs";
}
impl ::core::convert::From<ProximitySensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProximitySensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorReadingChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProximitySensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximitySensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProximitySensorReadingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximitySensorReadingChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProximitySensorReadingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProximitySensorReadingChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximitySensorReadingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ProximitySensorReadingChangedEventArgs {}
#[repr(transparent)]
pub struct SensorDataThresholdTriggerDetails(::windows_core::IUnknown);
impl SensorDataThresholdTriggerDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SensorType(&self) -> ::windows_core::Result<SensorType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SensorType>::zeroed();
            (::windows_core::Interface::vtable(this).SensorType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SensorType>(result__)
        }
    }
}
impl ::core::clone::Clone for SensorDataThresholdTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SensorDataThresholdTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorDataThresholdTriggerDetails {}
impl ::core::fmt::Debug for SensorDataThresholdTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorDataThresholdTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorDataThresholdTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorDataThresholdTriggerDetails;{9106f1b7-e88d-48b1-bc90-619c7b349391})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <ISensorDataThresholdTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorDataThresholdTriggerDetails";
}
impl ::core::convert::From<SensorDataThresholdTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: SensorDataThresholdTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorDataThresholdTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &SensorDataThresholdTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SensorDataThresholdTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: SensorDataThresholdTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorDataThresholdTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &SensorDataThresholdTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SensorDataThresholdTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SensorDataThresholdTriggerDetails {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorOptimizationGoal {}
impl ::core::clone::Clone for SensorOptimizationGoal {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorOptimizationGoal {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SensorOptimizationGoal {
    type Abi = Self;
}
impl ::core::fmt::Debug for SensorOptimizationGoal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorOptimizationGoal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorOptimizationGoal {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorOptimizationGoal;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SensorQuaternion(::windows_core::IUnknown);
impl SensorQuaternion {
    pub fn W(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).W)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn X(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).X)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Y(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Y)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Z(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Z)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for SensorQuaternion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SensorQuaternion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorQuaternion {}
impl ::core::fmt::Debug for SensorQuaternion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorQuaternion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorQuaternion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorQuaternion;{c9c5c827-c71c-46e7-9da3-36a193b232bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SensorQuaternion {
    type Vtable = ISensorQuaternion_Vtbl;
    const IID: ::windows_core::GUID = <ISensorQuaternion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorQuaternion";
}
impl ::core::convert::From<SensorQuaternion> for ::windows_core::IUnknown {
    fn from(value: SensorQuaternion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorQuaternion> for ::windows_core::IUnknown {
    fn from(value: &SensorQuaternion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SensorQuaternion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SensorQuaternion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SensorQuaternion> for ::windows_core::IInspectable {
    fn from(value: SensorQuaternion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorQuaternion> for ::windows_core::IInspectable {
    fn from(value: &SensorQuaternion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SensorQuaternion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SensorQuaternion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SensorQuaternion {}
unsafe impl ::core::marker::Sync for SensorQuaternion {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorReadingType {}
impl ::core::clone::Clone for SensorReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorReadingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SensorReadingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SensorReadingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorReadingType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorReadingType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorReadingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SensorRotationMatrix(::windows_core::IUnknown);
impl SensorRotationMatrix {
    pub fn M11(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M11)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M12(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M12)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M13(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M13)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M21(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M21)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M22(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M22)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M23(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M23)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M31(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M31)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M32(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M32)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn M33(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).M33)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for SensorRotationMatrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SensorRotationMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorRotationMatrix {}
impl ::core::fmt::Debug for SensorRotationMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorRotationMatrix").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorRotationMatrix {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SensorRotationMatrix;{0a3d5a67-22f4-4392-9538-65d0bd064aa6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_Vtbl;
    const IID: ::windows_core::GUID = <ISensorRotationMatrix as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorRotationMatrix";
}
impl ::core::convert::From<SensorRotationMatrix> for ::windows_core::IUnknown {
    fn from(value: SensorRotationMatrix) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorRotationMatrix> for ::windows_core::IUnknown {
    fn from(value: &SensorRotationMatrix) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SensorRotationMatrix {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SensorRotationMatrix {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SensorRotationMatrix> for ::windows_core::IInspectable {
    fn from(value: SensorRotationMatrix) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SensorRotationMatrix> for ::windows_core::IInspectable {
    fn from(value: &SensorRotationMatrix) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SensorRotationMatrix {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SensorRotationMatrix {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SensorRotationMatrix {}
unsafe impl ::core::marker::Sync for SensorRotationMatrix {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: Self = Self(0i32);
    pub const ActivitySensor: Self = Self(1i32);
    pub const Barometer: Self = Self(2i32);
    pub const Compass: Self = Self(3i32);
    pub const CustomSensor: Self = Self(4i32);
    pub const Gyroscope: Self = Self(5i32);
    pub const ProximitySensor: Self = Self(6i32);
    pub const Inclinometer: Self = Self(7i32);
    pub const LightSensor: Self = Self(8i32);
    pub const OrientationSensor: Self = Self(9i32);
    pub const Pedometer: Self = Self(10i32);
    pub const RelativeInclinometer: Self = Self(11i32);
    pub const RelativeOrientationSensor: Self = Self(12i32);
    pub const SimpleOrientationSensor: Self = Self(13i32);
}
impl ::core::marker::Copy for SensorType {}
impl ::core::clone::Clone for SensorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SensorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SensorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SensorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SensorType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl ::core::marker::Copy for SimpleOrientation {}
impl ::core::clone::Clone for SimpleOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SimpleOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SimpleOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for SimpleOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SimpleOrientation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SimpleOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SimpleOrientationSensor(::windows_core::IUnknown);
impl SimpleOrientationSensor {
    pub fn GetCurrentOrientation(&self) -> ::windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SimpleOrientation>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OrientationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOrientationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReadingTransform)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> ::windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &::windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Graphics::Display::DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).ReadingTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Display::DisplayOrientations>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISimpleOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SimpleOrientationSensor> {
        Self::ISimpleOrientationSensorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SimpleOrientationSensor>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>(result__)
        })
    }
    pub fn ISimpleOrientationSensorStatics<R, F: FnOnce(&ISimpleOrientationSensorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISimpleOrientationSensorStatics2<R, F: FnOnce(&ISimpleOrientationSensorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SimpleOrientationSensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleOrientationSensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleOrientationSensor {}
impl ::core::fmt::Debug for SimpleOrientationSensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientationSensor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SimpleOrientationSensor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensor;{5ff53856-214a-4dee-a3f9-616f1ab06ffd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_Vtbl;
    const IID: ::windows_core::GUID = <ISimpleOrientationSensor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensor";
}
impl ::core::convert::From<SimpleOrientationSensor> for ::windows_core::IUnknown {
    fn from(value: SimpleOrientationSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleOrientationSensor> for ::windows_core::IUnknown {
    fn from(value: &SimpleOrientationSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SimpleOrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SimpleOrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SimpleOrientationSensor> for ::windows_core::IInspectable {
    fn from(value: SimpleOrientationSensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleOrientationSensor> for ::windows_core::IInspectable {
    fn from(value: &SimpleOrientationSensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SimpleOrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SimpleOrientationSensor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SimpleOrientationSensor {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensor {}
#[repr(transparent)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(::windows_core::IUnknown);
impl SimpleOrientationSensorOrientationChangedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SimpleOrientation>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SimpleOrientation>(result__)
        }
    }
}
impl ::core::clone::Clone for SimpleOrientationSensorOrientationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleOrientationSensorOrientationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleOrientationSensorOrientationChangedEventArgs {}
impl ::core::fmt::Debug for SimpleOrientationSensorOrientationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleOrientationSensorOrientationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs;{bcd5c660-23d4-4b4c-a22e-ba81ade0c601})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISimpleOrientationSensorOrientationChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs";
}
impl ::core::convert::From<SimpleOrientationSensorOrientationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleOrientationSensorOrientationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SimpleOrientationSensorOrientationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleOrientationSensorOrientationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SimpleOrientationSensorOrientationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SimpleOrientationSensorOrientationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SimpleOrientationSensorOrientationChangedEventArgs {}
unsafe impl ::core::marker::Sync for SimpleOrientationSensorOrientationChangedEventArgs {}