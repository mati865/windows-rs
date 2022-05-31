
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Dhcp")]
pub mod Dhcp;
#[cfg(feature = "Dns")]
pub mod Dns;
#[cfg(feature = "InternetConnectionWizard")]
pub mod InternetConnectionWizard;
#[cfg(feature = "IpHelper")]
pub mod IpHelper;
#[cfg(feature = "MobileBroadband")]
pub mod MobileBroadband;
#[cfg(feature = "Multicast")]
pub mod Multicast;
#[cfg(feature = "Ndis")]
pub mod Ndis;
#[cfg(feature = "NetBios")]
pub mod NetBios;
#[cfg(feature = "NetManagement")]
pub mod NetManagement;
#[cfg(feature = "NetShell")]
pub mod NetShell;
#[cfg(feature = "NetworkDiagnosticsFramework")]
pub mod NetworkDiagnosticsFramework;
#[cfg(feature = "NetworkPolicyServer")]
pub mod NetworkPolicyServer;
#[cfg(feature = "P2P")]
pub mod P2P;
#[cfg(feature = "QoS")]
pub mod QoS;
#[cfg(feature = "Rras")]
pub mod Rras;
#[cfg(feature = "Snmp")]
pub mod Snmp;
#[cfg(feature = "WNet")]
pub mod WNet;
#[cfg(feature = "WebDav")]
pub mod WebDav;
#[cfg(feature = "WiFi")]
pub mod WiFi;
#[cfg(feature = "WindowsConnectNow")]
pub mod WindowsConnectNow;
#[cfg(feature = "WindowsConnectionManager")]
pub mod WindowsConnectionManager;
#[cfg(feature = "WindowsFilteringPlatform")]
pub mod WindowsFilteringPlatform;
#[cfg(feature = "WindowsFirewall")]
pub mod WindowsFirewall;
#[cfg(feature = "WindowsNetworkVirtualization")]
pub mod WindowsNetworkVirtualization;