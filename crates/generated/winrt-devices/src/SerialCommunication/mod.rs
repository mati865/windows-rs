#[repr(transparent)]
pub struct ErrorReceivedEventArgs(::windows_core::IUnknown);
impl ErrorReceivedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<SerialError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SerialError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SerialError>(result__)
        }
    }
}
impl ::core::clone::Clone for ErrorReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ErrorReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ErrorReceivedEventArgs {}
impl ::core::fmt::Debug for ErrorReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ErrorReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.ErrorReceivedEventArgs;{fcc6bf59-1283-4d8a-bfdf-566b33ddb28f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ErrorReceivedEventArgs {
    type Vtable = IErrorReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IErrorReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ErrorReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ErrorReceivedEventArgs";
}
impl ::core::convert::From<ErrorReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ErrorReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ErrorReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ErrorReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ErrorReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ErrorReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ErrorReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ErrorReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ErrorReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ErrorReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ErrorReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ErrorReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ErrorReceivedEventArgs {}
unsafe impl ::core::marker::Sync for ErrorReceivedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IErrorReceivedEventArgs {
    type Vtable = IErrorReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcc6bf59_1283_4d8a_bfdf_566b33ddb28f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPinChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPinChangedEventArgs {
    type Vtable = IPinChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2bf1db0_fc9c_4607_93d0_fa5e8343ee22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PinChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialPinChange) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISerialDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISerialDevice {
    type Vtable = ISerialDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe187ccc6_2210_414f_b65a_f5553a03372a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISerialDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BaudRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetBaudRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BreakSignalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBreakSignalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CarrierDetectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ClearToSendState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DataBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetDataBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    pub DataSetReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Handshake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialHandshake) -> ::windows_core::HRESULT,
    pub SetHandshake: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialHandshake) -> ::windows_core::HRESULT,
    pub IsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRequestToSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRequestToSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Parity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialParity) -> ::windows_core::HRESULT,
    pub SetParity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialParity) -> ::windows_core::HRESULT,
    pub PortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReadTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetReadTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StopBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SerialStopBitCount) -> ::windows_core::HRESULT,
    pub SetStopBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SerialStopBitCount) -> ::windows_core::HRESULT,
    pub UsbVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsbProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub WriteTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetWriteTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub ErrorReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PinChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePinChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISerialDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISerialDeviceStatics {
    type Vtable = ISerialDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x058c4a70_0836_4993_ae1a_b61ae3be056b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISerialDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorFromPortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorFromUsbVidPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PinChangedEventArgs(::windows_core::IUnknown);
impl PinChangedEventArgs {
    pub fn PinChange(&self) -> ::windows_core::Result<SerialPinChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SerialPinChange>::zeroed();
            (::windows_core::Interface::vtable(this).PinChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SerialPinChange>(result__)
        }
    }
}
impl ::core::clone::Clone for PinChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PinChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PinChangedEventArgs {}
impl ::core::fmt::Debug for PinChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PinChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PinChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.PinChangedEventArgs;{a2bf1db0-fc9c-4607-93d0-fa5e8343ee22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PinChangedEventArgs {
    type Vtable = IPinChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPinChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PinChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.PinChangedEventArgs";
}
impl ::core::convert::From<PinChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PinChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PinChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PinChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PinChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PinChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PinChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PinChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PinChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PinChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PinChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PinChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PinChangedEventArgs {}
unsafe impl ::core::marker::Sync for PinChangedEventArgs {}
#[repr(transparent)]
pub struct SerialDevice(::windows_core::IUnknown);
impl SerialDevice {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BaudRate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BaudRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetBaudRate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaudRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BreakSignalState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BreakSignalState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBreakSignalState(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBreakSignalState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BytesReceived(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CarrierDetectState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CarrierDetectState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ClearToSendState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ClearToSendState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DataBits(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).DataBits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetDataBits(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataBits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataSetReadyState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DataSetReadyState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Handshake(&self) -> ::windows_core::Result<SerialHandshake> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SerialHandshake>::zeroed();
            (::windows_core::Interface::vtable(this).Handshake)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SerialHandshake>(result__)
        }
    }
    pub fn SetHandshake(&self, value: SerialHandshake) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandshake)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDataTerminalReadyEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDataTerminalReadyEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDataTerminalReadyEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDataTerminalReadyEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestToSendEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequestToSendEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRequestToSendEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRequestToSendEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Parity(&self) -> ::windows_core::Result<SerialParity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SerialParity>::zeroed();
            (::windows_core::Interface::vtable(this).Parity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SerialParity>(result__)
        }
    }
    pub fn SetParity(&self, value: SerialParity) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetParity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PortName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PortName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ReadTimeout(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ReadTimeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetReadTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReadTimeout)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopBits(&self) -> ::windows_core::Result<SerialStopBitCount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SerialStopBitCount>::zeroed();
            (::windows_core::Interface::vtable(this).StopBits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SerialStopBitCount>(result__)
        }
    }
    pub fn SetStopBits(&self, value: SerialStopBitCount) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStopBits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UsbVendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsbVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn UsbProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsbProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn WriteTimeout(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).WriteTimeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetWriteTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWriteTimeout)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    pub fn ErrorReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs>>>(&self, reporthandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorReceived)(::windows_core::Interface::as_raw(this), reporthandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveErrorReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PinChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs>>>(&self, reporthandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PinChanged)(::windows_core::Interface::as_raw(this), reporthandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePinChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePinChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromPortName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(portname: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromPortName)(::windows_core::Interface::as_raw(this), portname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromUsbVidPid(vendorid: u16, productid: u16) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromUsbVidPid)(::windows_core::Interface::as_raw(this), vendorid, productid, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SerialDevice>> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SerialDevice>>(result__)
        })
    }
    pub fn ISerialDeviceStatics<R, F: FnOnce(&ISerialDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SerialDevice, ISerialDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SerialDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SerialDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SerialDevice {}
impl ::core::fmt::Debug for SerialDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.SerialCommunication.SerialDevice;{e187ccc6-2210-414f-b65a-f5553a03372a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SerialDevice {
    type Vtable = ISerialDevice_Vtbl;
    const IID: ::windows_core::GUID = <ISerialDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SerialDevice {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.SerialDevice";
}
impl ::core::convert::From<SerialDevice> for ::windows_core::IUnknown {
    fn from(value: SerialDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SerialDevice> for ::windows_core::IUnknown {
    fn from(value: &SerialDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SerialDevice> for ::windows_core::IInspectable {
    fn from(value: SerialDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SerialDevice> for ::windows_core::IInspectable {
    fn from(value: &SerialDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SerialDevice> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SerialDevice) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SerialDevice> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SerialDevice) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SerialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SerialDevice {}
unsafe impl ::core::marker::Sync for SerialDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SerialError(pub i32);
impl SerialError {
    pub const Frame: Self = Self(0i32);
    pub const BufferOverrun: Self = Self(1i32);
    pub const ReceiveFull: Self = Self(2i32);
    pub const ReceiveParity: Self = Self(3i32);
    pub const TransmitFull: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialError {}
impl ::core::clone::Clone for SerialError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SerialError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SerialError {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SerialHandshake(pub i32);
impl SerialHandshake {
    pub const None: Self = Self(0i32);
    pub const RequestToSend: Self = Self(1i32);
    pub const XOnXOff: Self = Self(2i32);
    pub const RequestToSendXOnXOff: Self = Self(3i32);
}
impl ::core::marker::Copy for SerialHandshake {}
impl ::core::clone::Clone for SerialHandshake {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SerialHandshake {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SerialHandshake {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialHandshake {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialHandshake").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialHandshake {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialHandshake;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SerialParity(pub i32);
impl SerialParity {
    pub const None: Self = Self(0i32);
    pub const Odd: Self = Self(1i32);
    pub const Even: Self = Self(2i32);
    pub const Mark: Self = Self(3i32);
    pub const Space: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialParity {}
impl ::core::clone::Clone for SerialParity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SerialParity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SerialParity {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialParity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialParity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialParity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialParity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SerialPinChange(pub i32);
impl SerialPinChange {
    pub const BreakSignal: Self = Self(0i32);
    pub const CarrierDetect: Self = Self(1i32);
    pub const ClearToSend: Self = Self(2i32);
    pub const DataSetReady: Self = Self(3i32);
    pub const RingIndicator: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialPinChange {}
impl ::core::clone::Clone for SerialPinChange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SerialPinChange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SerialPinChange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialPinChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialPinChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialPinChange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialPinChange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SerialStopBitCount(pub i32);
impl SerialStopBitCount {
    pub const One: Self = Self(0i32);
    pub const OnePointFive: Self = Self(1i32);
    pub const Two: Self = Self(2i32);
}
impl ::core::marker::Copy for SerialStopBitCount {}
impl ::core::clone::Clone for SerialStopBitCount {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SerialStopBitCount {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SerialStopBitCount {
    type Abi = Self;
}
impl ::core::fmt::Debug for SerialStopBitCount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SerialStopBitCount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SerialStopBitCount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialStopBitCount;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
