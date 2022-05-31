
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Connectivity")]
pub mod Connectivity;
#[cfg(feature = "NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Proximity")]
pub mod Proximity;
#[cfg(feature = "PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Sockets")]
pub mod Sockets;
#[cfg(feature = "Vpn")]
pub mod Vpn;
#[cfg(feature = "XboxLive")]
pub mod XboxLive;
#[repr(transparent)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EndpointPair = *mut ::core::ffi::c_void;
pub type HostName = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}