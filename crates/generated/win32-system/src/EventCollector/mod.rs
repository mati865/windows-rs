pub const EC_CREATE_NEW: u32 = 1u32;
pub const EC_OPEN_ALWAYS: u32 = 0u32;
pub const EC_OPEN_EXISTING: u32 = 2u32;
pub const EC_READ_ACCESS: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_CONFIGURATION_MODE(pub i32);
pub const EcConfigurationModeNormal: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(0i32);
pub const EcConfigurationModeCustom: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(1i32);
pub const EcConfigurationModeMinLatency: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(2i32);
pub const EcConfigurationModeMinBandwidth: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(3i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_CONFIGURATION_MODE {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_CONFIGURATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_CONFIGURATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_CONFIGURATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_CONFIGURATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_CONFIGURATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_CONTENT_FORMAT(pub i32);
pub const EcContentFormatEvents: EC_SUBSCRIPTION_CONTENT_FORMAT = EC_SUBSCRIPTION_CONTENT_FORMAT(1i32);
pub const EcContentFormatRenderedText: EC_SUBSCRIPTION_CONTENT_FORMAT = EC_SUBSCRIPTION_CONTENT_FORMAT(2i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_CONTENT_FORMAT {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_CONTENT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_CONTENT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_CONTENT_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_CONTENT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_CONTENT_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_CREDENTIALS_TYPE(pub i32);
pub const EcSubscriptionCredDefault: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(0i32);
pub const EcSubscriptionCredNegotiate: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(1i32);
pub const EcSubscriptionCredDigest: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(2i32);
pub const EcSubscriptionCredBasic: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(3i32);
pub const EcSubscriptionCredLocalMachine: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(4i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_CREDENTIALS_TYPE {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_CREDENTIALS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_CREDENTIALS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_CREDENTIALS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_CREDENTIALS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_CREDENTIALS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_DELIVERY_MODE(pub i32);
pub const EcDeliveryModePull: EC_SUBSCRIPTION_DELIVERY_MODE = EC_SUBSCRIPTION_DELIVERY_MODE(1i32);
pub const EcDeliveryModePush: EC_SUBSCRIPTION_DELIVERY_MODE = EC_SUBSCRIPTION_DELIVERY_MODE(2i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_DELIVERY_MODE {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_DELIVERY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_DELIVERY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_DELIVERY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_DELIVERY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_DELIVERY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_PROPERTY_ID(pub i32);
pub const EcSubscriptionEnabled: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(0i32);
pub const EcSubscriptionEventSources: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(1i32);
pub const EcSubscriptionEventSourceAddress: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(2i32);
pub const EcSubscriptionEventSourceEnabled: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(3i32);
pub const EcSubscriptionEventSourceUserName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(4i32);
pub const EcSubscriptionEventSourcePassword: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(5i32);
pub const EcSubscriptionDescription: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(6i32);
pub const EcSubscriptionURI: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(7i32);
pub const EcSubscriptionConfigurationMode: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(8i32);
pub const EcSubscriptionExpires: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(9i32);
pub const EcSubscriptionQuery: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(10i32);
pub const EcSubscriptionTransportName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(11i32);
pub const EcSubscriptionTransportPort: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(12i32);
pub const EcSubscriptionDeliveryMode: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(13i32);
pub const EcSubscriptionDeliveryMaxItems: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(14i32);
pub const EcSubscriptionDeliveryMaxLatencyTime: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(15i32);
pub const EcSubscriptionHeartbeatInterval: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(16i32);
pub const EcSubscriptionLocale: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(17i32);
pub const EcSubscriptionContentFormat: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(18i32);
pub const EcSubscriptionLogFile: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(19i32);
pub const EcSubscriptionPublisherName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(20i32);
pub const EcSubscriptionCredentialsType: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(21i32);
pub const EcSubscriptionCommonUserName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(22i32);
pub const EcSubscriptionCommonPassword: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(23i32);
pub const EcSubscriptionHostName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(24i32);
pub const EcSubscriptionReadExistingEvents: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(25i32);
pub const EcSubscriptionDialect: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(26i32);
pub const EcSubscriptionType: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(27i32);
pub const EcSubscriptionAllowedIssuerCAs: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(28i32);
pub const EcSubscriptionAllowedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(29i32);
pub const EcSubscriptionDeniedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(30i32);
pub const EcSubscriptionAllowedSourceDomainComputers: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(31i32);
pub const EcSubscriptionPropertyIdEND: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(32i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_PROPERTY_ID {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(pub i32);
pub const EcRuntimeStatusActiveStatusDisabled: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(1i32);
pub const EcRuntimeStatusActiveStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(2i32);
pub const EcRuntimeStatusActiveStatusInactive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(3i32);
pub const EcRuntimeStatusActiveStatusTrying: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(4i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(pub i32);
pub const EcSubscriptionRunTimeStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(0i32);
pub const EcSubscriptionRunTimeStatusLastError: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(1i32);
pub const EcSubscriptionRunTimeStatusLastErrorMessage: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(2i32);
pub const EcSubscriptionRunTimeStatusLastErrorTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(3i32);
pub const EcSubscriptionRunTimeStatusNextRetryTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(4i32);
pub const EcSubscriptionRunTimeStatusEventSources: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(5i32);
pub const EcSubscriptionRunTimeStatusLastHeartbeatTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(6i32);
pub const EcSubscriptionRunTimeStatusInfoIdEND: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(7i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SUBSCRIPTION_TYPE(pub i32);
pub const EcSubscriptionTypeSourceInitiated: EC_SUBSCRIPTION_TYPE = EC_SUBSCRIPTION_TYPE(0i32);
pub const EcSubscriptionTypeCollectorInitiated: EC_SUBSCRIPTION_TYPE = EC_SUBSCRIPTION_TYPE(1i32);
impl ::core::marker::Copy for EC_SUBSCRIPTION_TYPE {}
impl ::core::clone::Clone for EC_SUBSCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SUBSCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SUBSCRIPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SUBSCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SUBSCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct EC_VARIANT {
    pub Anonymous: EC_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
impl ::core::marker::Copy for EC_VARIANT {}
impl ::core::clone::Clone for EC_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for EC_VARIANT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EC_VARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EC_VARIANT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EC_VARIANT {}
impl ::core::default::Default for EC_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union EC_VARIANT_0 {
    pub BooleanVal: ::win32_foundation::BOOL,
    pub UInt32Val: u32,
    pub DateTimeVal: u64,
    pub StringVal: ::windows_core::PCWSTR,
    pub BinaryVal: *mut u8,
    pub BooleanArr: *mut ::win32_foundation::BOOL,
    pub Int32Arr: *mut i32,
    pub StringArr: *mut ::windows_core::PWSTR,
    pub PropertyHandleVal: isize,
}
impl ::core::marker::Copy for EC_VARIANT_0 {}
impl ::core::clone::Clone for EC_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for EC_VARIANT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EC_VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EC_VARIANT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for EC_VARIANT_0 {}
impl ::core::default::Default for EC_VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_VARIANT_TYPE(pub i32);
pub const EcVarTypeNull: EC_VARIANT_TYPE = EC_VARIANT_TYPE(0i32);
pub const EcVarTypeBoolean: EC_VARIANT_TYPE = EC_VARIANT_TYPE(1i32);
pub const EcVarTypeUInt32: EC_VARIANT_TYPE = EC_VARIANT_TYPE(2i32);
pub const EcVarTypeDateTime: EC_VARIANT_TYPE = EC_VARIANT_TYPE(3i32);
pub const EcVarTypeString: EC_VARIANT_TYPE = EC_VARIANT_TYPE(4i32);
pub const EcVarObjectArrayPropertyHandle: EC_VARIANT_TYPE = EC_VARIANT_TYPE(5i32);
impl ::core::marker::Copy for EC_VARIANT_TYPE {}
impl ::core::clone::Clone for EC_VARIANT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_VARIANT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_VARIANT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_VARIANT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_VARIANT_TYPE").field(&self.0).finish()
    }
}
pub const EC_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EC_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EC_WRITE_ACCESS: u32 = 2u32;
#[inline]
pub unsafe fn EcClose(object: isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcClose(object: isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcClose(::core::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcDeleteSubscription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(subscriptionname: Param0, flags: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcDeleteSubscription(subscriptionname: ::windows_core::PCWSTR, flags: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcDeleteSubscription(subscriptionname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcEnumNextSubscription(subscriptionenum: isize, subscriptionnamebuffer: &mut [u16], subscriptionnamebufferused: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcEnumNextSubscription(subscriptionenum: isize, subscriptionnamebuffersize: u32, subscriptionnamebuffer: ::windows_core::PWSTR, subscriptionnamebufferused: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcEnumNextSubscription(::core::mem::transmute(subscriptionenum), subscriptionnamebuffer.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(subscriptionnamebuffer)), ::core::mem::transmute(subscriptionnamebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcGetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcGetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcGetObjectArrayProperty(::core::mem::transmute(objectarray), ::core::mem::transmute(propertyid), ::core::mem::transmute(arrayindex), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcGetObjectArraySize(::core::mem::transmute(objectarray), ::core::mem::transmute(objectarraysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcGetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcGetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcGetSubscriptionProperty(::core::mem::transmute(subscription), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvaluebuffersize), ::core::mem::transmute(propertyvaluebuffer), ::core::mem::transmute(propertyvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcGetSubscriptionRunTimeStatus<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(subscriptionname: Param0, statusinfoid: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename: Param2, flags: u32, statusvaluebuffersize: u32, statusvaluebuffer: *mut EC_VARIANT, statusvaluebufferused: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcGetSubscriptionRunTimeStatus(subscriptionname: ::windows_core::PCWSTR, statusinfoid: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename: ::windows_core::PCWSTR, flags: u32, statusvaluebuffersize: u32, statusvaluebuffer: *mut EC_VARIANT, statusvaluebufferused: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcGetSubscriptionRunTimeStatus(subscriptionname.into_param().abi(), ::core::mem::transmute(statusinfoid), eventsourcename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(statusvaluebuffersize), ::core::mem::transmute(statusvaluebuffer), ::core::mem::transmute(statusvaluebufferused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcInsertObjectArrayElement(objectarray: isize, arrayindex: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcInsertObjectArrayElement(objectarray: isize, arrayindex: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcInsertObjectArrayElement(::core::mem::transmute(objectarray), ::core::mem::transmute(arrayindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcOpenSubscription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(subscriptionname: Param0, accessmask: u32, flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcOpenSubscription(subscriptionname: ::windows_core::PCWSTR, accessmask: u32, flags: u32) -> isize;
        }
        ::core::mem::transmute(EcOpenSubscription(subscriptionname.into_param().abi(), ::core::mem::transmute(accessmask), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcOpenSubscriptionEnum(flags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcOpenSubscriptionEnum(flags: u32) -> isize;
        }
        ::core::mem::transmute(EcOpenSubscriptionEnum(::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcRemoveObjectArrayElement(objectarray: isize, arrayindex: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcRemoveObjectArrayElement(objectarray: isize, arrayindex: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcRemoveObjectArrayElement(::core::mem::transmute(objectarray), ::core::mem::transmute(arrayindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcRetrySubscription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(subscriptionname: Param0, eventsourcename: Param1, flags: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcRetrySubscription(subscriptionname: ::windows_core::PCWSTR, eventsourcename: ::windows_core::PCWSTR, flags: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcRetrySubscription(subscriptionname.into_param().abi(), eventsourcename.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcSaveSubscription(subscription: isize, flags: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcSaveSubscription(subscription: isize, flags: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcSaveSubscription(::core::mem::transmute(subscription), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcSetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvalue: *mut EC_VARIANT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcSetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvalue: *mut EC_VARIANT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcSetObjectArrayProperty(::core::mem::transmute(objectarray), ::core::mem::transmute(propertyid), ::core::mem::transmute(arrayindex), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EcSetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvalue: *mut EC_VARIANT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EcSetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvalue: *mut EC_VARIANT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EcSetSubscriptionProperty(::core::mem::transmute(subscription), ::core::mem::transmute(propertyid), ::core::mem::transmute(flags), ::core::mem::transmute(propertyvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
