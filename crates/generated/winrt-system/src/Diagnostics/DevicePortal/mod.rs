#[repr(transparent)]
pub struct DevicePortalConnection(::windows_core::IUnknown);
impl DevicePortalConnection {
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RequestReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn GetForAppServiceConnection<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::ApplicationModel::AppService::AppServiceConnection>>(appserviceconnection: Param0) -> ::windows_core::Result<DevicePortalConnection> {
        Self::IDevicePortalConnectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForAppServiceConnection)(::windows_core::Interface::as_raw(this), appserviceconnection.into_param().abi(), result__.as_mut_ptr()).from_abi::<DevicePortalConnection>(result__)
        })
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest2<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, request: Param0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: Param2) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest2)(::windows_core::Interface::as_raw(this), request.into_param().abi(), messagetype, protocol.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest3<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, request: Param0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: Param2, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest3)(::windows_core::Interface::as_raw(this), request.into_param().abi(), messagetype, protocol.into_param().abi(), outboundbuffersizeinbytes, maxmessagesize, receivemode, result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::ServerMessageWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>>(&self, request: Param0) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::ServerStreamWebSocket>(result__)
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest2<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Web::Http::HttpRequestMessage>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, request: Param0, protocol: Param1, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest2)(::windows_core::Interface::as_raw(this), request.into_param().abi(), protocol.into_param().abi(), outboundbuffersizeinbytes, nodelay, result__.as_mut_ptr()).from_abi::<super::super::super::Networking::Sockets::ServerStreamWebSocket>(result__)
        }
    }
    pub fn IDevicePortalConnectionStatics<R, F: FnOnce(&IDevicePortalConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DevicePortalConnection, IDevicePortalConnectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DevicePortalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnection {}
impl ::core::fmt::Debug for DevicePortalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePortalConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnection;{0f447f51-1198-4da1-8d54-bdef393e09b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePortalConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
}
impl ::core::convert::From<DevicePortalConnection> for ::windows_core::IUnknown {
    fn from(value: DevicePortalConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnection> for ::windows_core::IUnknown {
    fn from(value: &DevicePortalConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePortalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePortalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePortalConnection> for ::windows_core::IInspectable {
    fn from(value: DevicePortalConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnection> for ::windows_core::IInspectable {
    fn from(value: &DevicePortalConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePortalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePortalConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnection {}
unsafe impl ::core::marker::Sync for DevicePortalConnection {}
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(::windows_core::IUnknown);
impl DevicePortalConnectionClosedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<DevicePortalConnectionClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DevicePortalConnectionClosedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePortalConnectionClosedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnectionClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnectionClosedEventArgs {}
impl ::core::fmt::Debug for DevicePortalConnectionClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePortalConnectionClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs;{fcf70e38-7032-428c-9f50-945c15a9f0cb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePortalConnectionClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
}
impl ::core::convert::From<DevicePortalConnectionClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DevicePortalConnectionClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnectionClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DevicePortalConnectionClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePortalConnectionClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DevicePortalConnectionClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnectionClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DevicePortalConnectionClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePortalConnectionClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnectionClosedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionClosedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: Self = Self(0i32);
    pub const ResourceLimitsExceeded: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const NotAuthorized: Self = Self(3i32);
    pub const UserNotPresent: Self = Self(4i32);
    pub const ServiceTerminated: Self = Self(5i32);
}
impl ::core::marker::Copy for DevicePortalConnectionClosedReason {}
impl ::core::clone::Clone for DevicePortalConnectionClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePortalConnectionClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DevicePortalConnectionClosedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePortalConnectionClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePortalConnectionClosedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
impl DevicePortalConnectionRequestReceivedEventArgs {
    #[cfg(feature = "Web_Http")]
    pub fn RequestMessage(&self) -> ::windows_core::Result<super::super::super::Web::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Web::Http::HttpRequestMessage>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn ResponseMessage(&self) -> ::windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    pub fn IsWebSocketUpgradeRequest(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWebSocketUpgradeRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebSocketProtocolsRequested(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebSocketProtocolsRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = &::windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePortalConnectionRequestReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePortalConnectionRequestReceivedEventArgs {}
impl ::core::fmt::Debug for DevicePortalConnectionRequestReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionRequestReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs;{64dae045-6fda-4459-9ebd-ecce22e38559})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePortalConnectionRequestReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
}
impl ::core::convert::From<DevicePortalConnectionRequestReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnectionRequestReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePortalConnectionRequestReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePortalConnectionRequestReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DevicePortalConnectionRequestReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePortalConnectionRequestReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePortalConnectionRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePortalConnectionRequestReceivedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f447f51_1198_4da1_8d54_bdef393e09b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePortalConnectionClosedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64dae045_6fda_4459_9ebd_ecce22e38559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Web_Http")]
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    RequestMessage: usize,
    #[cfg(feature = "Web_Http")]
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    ResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalConnectionStatics {
    type Vtable = IDevicePortalConnectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bbe31e7_e9b9_4645_8fed_a53eea0edbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalWebSocketConnection {
    type Vtable = IDevicePortalWebSocketConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67657920_d65a_42f0_aef4_787808098b7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest2: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest3: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, protocol: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79fdcaba_175c_4739_9f74_dda797c35b3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsWebSocketUpgradeRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub WebSocketProtocolsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebSocketProtocolsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}