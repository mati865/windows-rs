pub struct AppServiceCatalog;
impl AppServiceCatalog {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppServiceProvidersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appservicename: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>> {
        Self::IAppServiceCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppServiceProvidersAsync)(::windows_core::Interface::as_raw(this), appservicename.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>>(result__)
        })
    }
    pub fn IAppServiceCatalogStatics<R, F: FnOnce(&IAppServiceCatalogStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppServiceCatalog, IAppServiceCatalogStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AppServiceCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceCatalog";
}
#[repr(transparent)]
pub struct AppServiceClosedEventArgs(::windows_core::IUnknown);
impl AppServiceClosedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<AppServiceClosedStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppServiceClosedStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppServiceClosedStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceClosedEventArgs {}
impl ::core::fmt::Debug for AppServiceClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceClosedEventArgs;{de6016f6-cb03-4d35-ac8d-cc6303239731})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
}
impl ::core::convert::From<AppServiceClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppServiceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppServiceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceClosedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for AppServiceClosedStatus {}
impl ::core::clone::Clone for AppServiceClosedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceClosedStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppServiceClosedStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppServiceClosedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceClosedStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceClosedStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceClosedStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppServiceConnection(::windows_core::IUnknown);
impl AppServiceConnection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppServiceConnection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AppServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppServiceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAppServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppServiceName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageFamilyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RequestReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ServiceClosed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceClosed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceClosed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServiceClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn OpenRemoteAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>>(&self, remotesystemconnectionrequest: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = &::windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenRemoteAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUser)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn SendStatelessMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, AppServiceConnection>, Param1: ::windows_core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemConnectionRequest>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(connection: Param0, connectionrequest: Param1, message: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>> {
        Self::IAppServiceConnectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendStatelessMessageAsync)(::windows_core::Interface::as_raw(this), connection.into_param().abi(), connectionrequest.into_param().abi(), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IAppServiceConnectionStatics<R, F: FnOnce(&IAppServiceConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppServiceConnection, IAppServiceConnectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceConnection {}
impl ::core::fmt::Debug for AppServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceConnection;{9dd474a2-871f-4d52-89a9-9e090531bd27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceConnection {
    type Vtable = IAppServiceConnection_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceConnection";
}
impl ::core::convert::From<AppServiceConnection> for ::windows_core::IUnknown {
    fn from(value: AppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceConnection> for ::windows_core::IUnknown {
    fn from(value: &AppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceConnection> for ::windows_core::IInspectable {
    fn from(value: AppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceConnection> for ::windows_core::IInspectable {
    fn from(value: &AppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: AppServiceConnection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AppServiceConnection> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppServiceConnection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &AppServiceConnection {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppServiceConnection {}
unsafe impl ::core::marker::Sync for AppServiceConnection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl ::core::marker::Copy for AppServiceConnectionStatus {}
impl ::core::clone::Clone for AppServiceConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppServiceConnectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppServiceConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceConnectionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceConnectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppServiceDeferral(::windows_core::IUnknown);
impl AppServiceDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppServiceDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceDeferral {}
impl ::core::fmt::Debug for AppServiceDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceDeferral;{7e1b5322-eab0-4248-ae04-fdf93838e472})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceDeferral {
    type Vtable = IAppServiceDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceDeferral";
}
impl ::core::convert::From<AppServiceDeferral> for ::windows_core::IUnknown {
    fn from(value: AppServiceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceDeferral> for ::windows_core::IUnknown {
    fn from(value: &AppServiceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceDeferral> for ::windows_core::IInspectable {
    fn from(value: AppServiceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceDeferral> for ::windows_core::IInspectable {
    fn from(value: &AppServiceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceDeferral {}
unsafe impl ::core::marker::Sync for AppServiceDeferral {}
#[repr(transparent)]
pub struct AppServiceRequest(::windows_core::IUnknown);
impl AppServiceRequest {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendResponseAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendResponseAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceRequest {}
impl ::core::fmt::Debug for AppServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequest;{20e58d9d-18de-4b01-80ba-90a76204e3c8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceRequest {
    type Vtable = IAppServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequest";
}
impl ::core::convert::From<AppServiceRequest> for ::windows_core::IUnknown {
    fn from(value: AppServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequest> for ::windows_core::IUnknown {
    fn from(value: &AppServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceRequest> for ::windows_core::IInspectable {
    fn from(value: AppServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequest> for ::windows_core::IInspectable {
    fn from(value: &AppServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceRequest {}
unsafe impl ::core::marker::Sync for AppServiceRequest {}
#[repr(transparent)]
pub struct AppServiceRequestReceivedEventArgs(::windows_core::IUnknown);
impl AppServiceRequestReceivedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<AppServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppServiceRequest>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<AppServiceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppServiceDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceRequestReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceRequestReceivedEventArgs {}
impl ::core::fmt::Debug for AppServiceRequestReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceRequestReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceRequestReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs;{6e122360-ff65-44ae-9e45-857fe4180681})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceRequestReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
}
impl ::core::convert::From<AppServiceRequestReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceRequestReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppServiceRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceRequestReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppServiceRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceRequestReceivedEventArgs {}
#[repr(transparent)]
pub struct AppServiceResponse(::windows_core::IUnknown);
impl AppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppServiceResponseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppServiceResponseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceResponse {}
impl ::core::fmt::Debug for AppServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceResponse;{8d503cec-9aa3-4e68-9559-9de63e372ce4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceResponse {
    type Vtable = IAppServiceResponse_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceResponse as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceResponse";
}
impl ::core::convert::From<AppServiceResponse> for ::windows_core::IUnknown {
    fn from(value: AppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceResponse> for ::windows_core::IUnknown {
    fn from(value: &AppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceResponse> for ::windows_core::IInspectable {
    fn from(value: AppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceResponse> for ::windows_core::IInspectable {
    fn from(value: &AppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceResponse {}
unsafe impl ::core::marker::Sync for AppServiceResponse {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for AppServiceResponseStatus {}
impl ::core::clone::Clone for AppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppServiceResponseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceResponseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceResponseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceResponseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppServiceTriggerDetails(::windows_core::IUnknown);
impl AppServiceTriggerDetails {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppServiceConnection(&self) -> ::windows_core::Result<AppServiceConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppServiceConnection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppServiceConnection>(result__)
        }
    }
    pub fn IsRemoteSystemConnection(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSystemConnection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CheckCallerForCapabilityAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, capabilityname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppServiceTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckCallerForCapabilityAsync)(::windows_core::Interface::as_raw(this), capabilityname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CallerRemoteConnectionToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppServiceTriggerDetails4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerRemoteConnectionToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppServiceTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppServiceTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppServiceTriggerDetails {}
impl ::core::fmt::Debug for AppServiceTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppServiceTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceTriggerDetails;{88a2dcac-ad28-41b8-80bb-bdf1b2169e19})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAppServiceTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
}
impl ::core::convert::From<AppServiceTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: AppServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppServiceTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppServiceTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: AppServiceTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppServiceTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &AppServiceTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppServiceTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppServiceTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppServiceTriggerDetails {}
unsafe impl ::core::marker::Sync for AppServiceTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceCatalogStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceCatalogStatics {
    type Vtable = IAppServiceCatalogStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef0d2507_d132_4c85_8395_3c31d5a1e941);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceCatalogStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppServiceProvidersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appservicename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppServiceProvidersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde6016f6_cb03_4d35_ac8d_cc6303239731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceClosedStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnection {
    type Vtable = IAppServiceConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dd474a2_871f_4d52_89a9_9e090531bd27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnection2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnection2 {
    type Vtable = IAppServiceConnection2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bdfcd5f_2302_4fbd_8061_52511c2f8bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub OpenRemoteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    OpenRemoteAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "System")]
    pub SetUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnectionStatics {
    type Vtable = IAppServiceConnectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadc56ce9_d408_5673_8637_827a4b274168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub SendStatelessMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: ::windows_core::RawPtr, connectionrequest: ::windows_core::RawPtr, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    SendStatelessMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceDeferral {
    type Vtable = IAppServiceDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e1b5322_eab0_4248_ae04_fdf93838e472);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceRequest {
    type Vtable = IAppServiceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e58d9d_18de_4b01_80ba_90a76204e3c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendResponseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e122360_ff65_44ae_9e45_857fe4180681);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceResponse {
    type Vtable = IAppServiceResponse_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d503cec_9aa3_4e68_9559_9de63e372ce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceResponse_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceResponseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88a2dcac_ad28_41b8_80bb_bdf1b2169e19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails2 {
    type Vtable = IAppServiceTriggerDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe83d54b2_28cc_43f2_b465_c0482e59e2dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRemoteSystemConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails3 {
    type Vtable = IAppServiceTriggerDetails3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbd71e21_7939_4e68_9e3c_7780147aabb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CheckCallerForCapabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckCallerForCapabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppServiceTriggerDetails4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails4 {
    type Vtable = IAppServiceTriggerDetails4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1185b180_8861_5e30_ab55_1cf4d08bbf6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallerRemoteConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatelessAppServiceResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43754af7_a9ec_52fe_82e7_939b68dc9388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatelessAppServiceResponse_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StatelessAppServiceResponseStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct StatelessAppServiceResponse(::windows_core::IUnknown);
impl StatelessAppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<StatelessAppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StatelessAppServiceResponseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StatelessAppServiceResponseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StatelessAppServiceResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatelessAppServiceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatelessAppServiceResponse {}
impl ::core::fmt::Debug for StatelessAppServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StatelessAppServiceResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.StatelessAppServiceResponse;{43754af7-a9ec-52fe-82e7-939b68dc9388})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_Vtbl;
    const IID: ::windows_core::GUID = <IStatelessAppServiceResponse as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
}
impl ::core::convert::From<StatelessAppServiceResponse> for ::windows_core::IUnknown {
    fn from(value: StatelessAppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatelessAppServiceResponse> for ::windows_core::IUnknown {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StatelessAppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StatelessAppServiceResponse> for ::windows_core::IInspectable {
    fn from(value: StatelessAppServiceResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatelessAppServiceResponse> for ::windows_core::IInspectable {
    fn from(value: &StatelessAppServiceResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StatelessAppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StatelessAppServiceResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StatelessAppServiceResponse {}
unsafe impl ::core::marker::Sync for StatelessAppServiceResponse {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl ::core::marker::Copy for StatelessAppServiceResponseStatus {}
impl ::core::clone::Clone for StatelessAppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StatelessAppServiceResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StatelessAppServiceResponseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StatelessAppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StatelessAppServiceResponseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}