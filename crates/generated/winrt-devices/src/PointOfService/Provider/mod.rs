#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequest(::windows_core::IUnknown);
impl BarcodeScannerDisableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDisableScannerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDisableScannerRequest {}
impl ::core::fmt::Debug for BarcodeScannerDisableScannerRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerDisableScannerRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerDisableScannerRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest;{88ecf7c0-37b9-4275-8e77-c8e52ae5a9c8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerDisableScannerRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerDisableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerDisableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerDisableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerDisableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerDisableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequest {}
#[repr(transparent)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerDisableScannerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerDisableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerDisableScannerRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDisableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDisableScannerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDisableScannerRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerDisableScannerRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerDisableScannerRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs;{7006e142-e802-46f5-b604-352a15ce9232})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerDisableScannerRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDisableScannerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDisableScannerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerDisableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerDisableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDisableScannerRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequest(::windows_core::IUnknown);
impl BarcodeScannerEnableScannerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerEnableScannerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerEnableScannerRequest {}
impl ::core::fmt::Debug for BarcodeScannerEnableScannerRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerEnableScannerRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerEnableScannerRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest;{c0b3e9ba-816a-452b-bd77-b7e453ec446d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerEnableScannerRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerEnableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerEnableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerEnableScannerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerEnableScannerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerEnableScannerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequest {}
#[repr(transparent)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerEnableScannerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerEnableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerEnableScannerRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerEnableScannerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerEnableScannerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerEnableScannerRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerEnableScannerRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerEnableScannerRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs;{956c9419-7b4e-4451-8c41-8e10cfbc5b41})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerEnableScannerRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerEnableScannerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerEnableScannerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerEnableScannerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerEnableScannerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerEnableScannerRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerFrameReader(::windows_core::IUnknown);
impl BarcodeScannerFrameReader {
    pub fn StartAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn TryAcquireLatestFrameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScannerVideoFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryAcquireLatestFrameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScannerVideoFrame>>(result__)
        }
    }
    pub fn Connection(&self) -> ::windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerFrameReader {}
impl ::core::fmt::Debug for BarcodeScannerFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerFrameReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader;{dbc72b07-64c3-482b-93c8-65fb33c22208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerFrameReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
}
impl ::core::convert::From<BarcodeScannerFrameReader> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReader> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerFrameReader> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReader> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScannerFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerFrameReader {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReader {}
#[repr(transparent)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerFrameReaderFrameArrivedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerFrameReaderFrameArrivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs;{b0bbd604-54fd-436d-8629-712e787223dd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerFrameReaderFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerFrameReaderFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerFrameReaderFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerFrameReaderFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(::windows_core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Symbology)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReportCompletedAsync<'a, Param0: ::windows_core::IntoParam<'a, super::BarcodeSymbologyAttributes>>(&self, attributes: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), attributes.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerGetSymbologyAttributesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerGetSymbologyAttributesRequest {}
impl ::core::fmt::Debug for BarcodeScannerGetSymbologyAttributesRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerGetSymbologyAttributesRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest;{9774c46a-58e4-4c5f-b8e9-e41467632700})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerGetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequest {}
#[repr(transparent)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerGetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerGetSymbologyAttributesRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerGetSymbologyAttributesRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs;{7f89de3e-fb5d-493c-b402-356b24d574a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerGetSymbologyAttributesRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerGetSymbologyAttributesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerGetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequest(::windows_core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerHideVideoPreviewRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerHideVideoPreviewRequest {}
impl ::core::fmt::Debug for BarcodeScannerHideVideoPreviewRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerHideVideoPreviewRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest;{fa4ebe7f-6670-40e1-b90b-bb10d8d425fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerHideVideoPreviewRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerHideVideoPreviewRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerHideVideoPreviewRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerHideVideoPreviewRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerHideVideoPreviewRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequest {}
#[repr(transparent)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerHideVideoPreviewRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerHideVideoPreviewRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerHideVideoPreviewRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerHideVideoPreviewRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerHideVideoPreviewRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs;{16a281fc-d6be-4bc7-9df1-33741f3eadea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerHideVideoPreviewRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerHideVideoPreviewRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerHideVideoPreviewRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerHideVideoPreviewRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerHideVideoPreviewRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerProviderConnection(::windows_core::IUnknown);
impl BarcodeScannerProviderConnection {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSymbologies(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedSymbologies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<u32>>(result__)
        }
    }
    pub fn CompanyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CompanyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCompanyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Version(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportScannedDataAsync<'a, Param0: ::windows_core::IntoParam<'a, super::BarcodeScannerReport>>(&self, report: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportScannedDataAsync)(::windows_core::Interface::as_raw(this), report.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportTriggerStateAsync)(::windows_core::Interface::as_raw(this), state, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportErrorAsync<'a, Param0: ::windows_core::IntoParam<'a, super::UnifiedPosErrorData>>(&self, errordata: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportErrorAsync)(::windows_core::Interface::as_raw(this), errordata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportErrorAsyncWithScanReport<'a, Param0: ::windows_core::IntoParam<'a, super::UnifiedPosErrorData>, Param2: ::windows_core::IntoParam<'a, super::BarcodeScannerReport>>(&self, errordata: Param0, isretriable: bool, scanreport: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportErrorAsyncWithScanReport)(::windows_core::Interface::as_raw(this), errordata.into_param().abi(), isretriable, scanreport.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn EnableScannerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnableScannerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnableScannerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnableScannerRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DisableScannerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisableScannerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisableScannerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisableScannerRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetActiveSymbologiesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SetActiveSymbologiesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSetActiveSymbologiesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetActiveSymbologiesRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StartSoftwareTriggerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StartSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStartSoftwareTriggerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStartSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StopSoftwareTriggerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StopSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStopSoftwareTriggerRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopSoftwareTriggerRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetBarcodeSymbologyAttributesRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HideVideoPreviewRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HideVideoPreviewRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHideVideoPreviewRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHideVideoPreviewRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateFrameReaderAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAsync(&self, preferredformat: ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAsync)(::windows_core::Interface::as_raw(this), preferredformat, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFrameReaderWithFormatAndSizeAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, preferredformat: ::winrt_graphics::Imaging::BitmapPixelFormat, preferredsize: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithFormatAndSizeAsync)(::windows_core::Interface::as_raw(this), preferredformat, preferredsize.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerProviderConnection {}
impl ::core::fmt::Debug for BarcodeScannerProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerProviderConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection;{b44acbed-0b3a-4fa3-86c5-491ea30780eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerProviderConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
}
impl ::core::convert::From<BarcodeScannerProviderConnection> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderConnection> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerProviderConnection> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderConnection> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScannerProviderConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerProviderConnection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerProviderConnection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerProviderConnection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerProviderConnection {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderConnection {}
#[repr(transparent)]
pub struct BarcodeScannerProviderTriggerDetails(::windows_core::IUnknown);
impl BarcodeScannerProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows_core::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerProviderTriggerDetails {}
impl ::core::fmt::Debug for BarcodeScannerProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerProviderTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails;{50856d82-24e3-48ce-99c7-70aac1cbc9f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerProviderTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
}
impl ::core::convert::From<BarcodeScannerProviderTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerProviderTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerProviderTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for BarcodeScannerProviderTriggerDetails {}
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(::windows_core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequest {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Symbologies(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Symbologies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetActiveSymbologiesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetActiveSymbologiesRequest {}
impl ::core::fmt::Debug for BarcodeScannerSetActiveSymbologiesRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerSetActiveSymbologiesRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest;{db3f32b9-f7da-41a1-9f79-07bcd95f0bdf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerSetActiveSymbologiesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequest {}
#[repr(transparent)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerSetActiveSymbologiesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerSetActiveSymbologiesRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerSetActiveSymbologiesRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs;{06305afa-7bf6-4d52-801a-330272f60ae1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerSetActiveSymbologiesRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetActiveSymbologiesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerSetActiveSymbologiesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(::windows_core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequest {
    pub fn Symbology(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Symbology)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetSymbologyAttributesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetSymbologyAttributesRequest {}
impl ::core::fmt::Debug for BarcodeScannerSetSymbologyAttributesRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerSetSymbologyAttributesRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest;{32fb814f-a37f-48b0-acea-dce1480f12ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerSetSymbologyAttributesRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequest {}
#[repr(transparent)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerSetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerSetSymbologyAttributesRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerSetSymbologyAttributesRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs;{b2b89809-9824-47d4-85bd-d0077baa7bd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerSetSymbologyAttributesRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerSetSymbologyAttributesRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerSetSymbologyAttributesRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(::windows_core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStartSoftwareTriggerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStartSoftwareTriggerRequest {}
impl ::core::fmt::Debug for BarcodeScannerStartSoftwareTriggerRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStartSoftwareTriggerRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest;{e3fa7b27-ff62-4454-af4a-cb6144a3e3f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerStartSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequest {}
#[repr(transparent)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerStartSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerStartSoftwareTriggerRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStartSoftwareTriggerRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs;{2305d843-c88f-4f3b-8c3b-d3df071051ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerStartSoftwareTriggerRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStartSoftwareTriggerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerStartSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(::windows_core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequest {
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAsync)(::windows_core::Interface::as_raw(this), reason, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedWithFailedReasonAndDescriptionAsync)(::windows_core::Interface::as_raw(this), reason, failedreasondescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStopSoftwareTriggerRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStopSoftwareTriggerRequest {}
impl ::core::fmt::Debug for BarcodeScannerStopSoftwareTriggerRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStopSoftwareTriggerRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest;{6f9faf35-e287-4ca8-b70d-5a91d694f668})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequest> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequest> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequest> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequest> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerStopSoftwareTriggerRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequest {}
#[repr(transparent)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
impl BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<BarcodeScannerStopSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BarcodeScannerStopSoftwareTriggerRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStopSoftwareTriggerRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs;{eac34450-4eb7-481a-9273-147a273b99b8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerStopSoftwareTriggerRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStopSoftwareTriggerRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerStopSoftwareTriggerRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl ::core::marker::Copy for BarcodeScannerTriggerState {}
impl ::core::clone::Clone for BarcodeScannerTriggerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeScannerTriggerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BarcodeScannerTriggerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeScannerTriggerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerTriggerState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerTriggerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BarcodeScannerVideoFrame(::windows_core::IUnknown);
impl BarcodeScannerVideoFrame {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Format(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PixelData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PixelData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BarcodeScannerVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerVideoFrame {}
impl ::core::fmt::Debug for BarcodeScannerVideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerVideoFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerVideoFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame;{7e585248-9df7-4121-a175-801d8000112e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerVideoFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
}
impl ::core::convert::From<BarcodeScannerVideoFrame> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerVideoFrame> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerVideoFrame> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerVideoFrame> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScannerVideoFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerVideoFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerVideoFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerVideoFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerVideoFrame {}
unsafe impl ::core::marker::Sync for BarcodeScannerVideoFrame {}
#[repr(transparent)]
pub struct BarcodeSymbologyAttributesBuilder(::windows_core::IUnknown);
impl BarcodeSymbologyAttributesBuilder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BarcodeSymbologyAttributesBuilder, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitValidationSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitValidationSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCheckDigitTransmissionSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecodeLengthSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecodeLengthSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecodeLengthSupported)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAttributes(&self) -> ::windows_core::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeSymbologyAttributesBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeSymbologyAttributesBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeSymbologyAttributesBuilder {}
impl ::core::fmt::Debug for BarcodeSymbologyAttributesBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyAttributesBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeSymbologyAttributesBuilder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder;{c57b0cbf-e4f5-40b9-84cf-e63fbaea42b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeSymbologyAttributesBuilder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
}
impl ::core::convert::From<BarcodeSymbologyAttributesBuilder> for ::windows_core::IUnknown {
    fn from(value: BarcodeSymbologyAttributesBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributesBuilder> for ::windows_core::IUnknown {
    fn from(value: &BarcodeSymbologyAttributesBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeSymbologyAttributesBuilder> for ::windows_core::IInspectable {
    fn from(value: BarcodeSymbologyAttributesBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributesBuilder> for ::windows_core::IInspectable {
    fn from(value: &BarcodeSymbologyAttributesBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeSymbologyAttributesBuilder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributesBuilder {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributesBuilder {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88ecf7c0_37b9_4275_8e77_c8e52ae5a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequest2 {
    type Vtable = IBarcodeScannerDisableScannerRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccdfe625_65c3_4ccc_b457_f39c7a9ea60d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7006e142_e802_46f5_b604_352a15ce9232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b3e9ba_816a_452b_bd77_b7e453ec446d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequest2 {
    type Vtable = IBarcodeScannerEnableScannerRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71a4f2a8_9905_41ac_9121_b645916a84a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x956c9419_7b4e_4451_8c41_8e10cfbc5b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerFrameReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbc72b07_64c3_482b_93c8_65fb33c22208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryAcquireLatestFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0bbd604_54fd_436d_8629_712e787223dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9774c46a_58e4_4c5f_b8e9_e41467632700);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Symbology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a6a2b13_75a8_49fb_b852_bfb93d760af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f89de3e_fb5d_493c_b402_356b24d574a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa4ebe7f_6670_40e1_b90b_bb10d8d425fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequest2 {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e28435d_9814_431d_a2f2_d6248c5ad4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16a281fc_d6be_4bc7_9df1_33741f3eadea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb44acbed_0b3a_4fa3_86c5_491ea30780eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSymbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSymbologies: usize,
    pub CompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportScannedDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, report: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportTriggerStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: BarcodeScannerTriggerState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportErrorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportErrorAsyncWithScanReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errordata: ::windows_core::RawPtr, isretriable: bool, scanreport: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DisableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisableScannerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub HideVideoPreviewRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHideVideoPreviewRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerProviderConnection2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderConnection2 {
    type Vtable = IBarcodeScannerProviderConnection2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe9b53cd_1134_418c_a06b_04423a73f3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: ::winrt_graphics::Imaging::BitmapPixelFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAsync: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFrameReaderWithFormatAndSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredformat: ::winrt_graphics::Imaging::BitmapPixelFormat, preferredsize: ::winrt_graphics::Imaging::BitmapSize, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFrameReaderWithFormatAndSizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerProviderTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50856d82_24e3_48ce_99c7_70aac1cbc9f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb3f32b9_f7da_41a1_9f79_07bcd95f0bdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Symbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Symbologies: usize,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5ff6edf_fa9a_4749_b11b_e8fccb75bc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06305afa_7bf6_4d52_801a_330272f60ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32fb814f_a37f_48b0_acea_dce1480f12ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Symbology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdffbbfc1_dba8_4b77_be1e_b56cd72f65b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2b89809_9824_47d4_85bd_d0077baa7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3fa7b27_ff62_4454_af4a_cb6144a3e3f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb72a25c_6658_4765_a68e_327482653deb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2305d843_c88f_4f3b_8c3b_d3df071051ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f9faf35_e287_4ca8_b70d_5a91d694f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb57c5dd_fe50_49f8_a0b4_bdc230814da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: i32, failedreasondescription: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeac34450_4eb7_481a_9273_147a273b99b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e585248_9df7_4121_a175_801d8000112e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Format: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributesBuilder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc57b0cbf_e4f5_40b9_84cf_e63fbaea42b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributesBuilder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CreateAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
