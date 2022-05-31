
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "ActiveDirectory")]
pub mod ActiveDirectory;
#[cfg(feature = "BackgroundIntelligentTransferService")]
pub mod BackgroundIntelligentTransferService;
#[cfg(feature = "Clustering")]
pub mod Clustering;
#[cfg(feature = "HttpServer")]
pub mod HttpServer;
#[cfg(feature = "Ldap")]
pub mod Ldap;
#[cfg(feature = "NetworkListManager")]
pub mod NetworkListManager;
#[cfg(feature = "RemoteDifferentialCompression")]
pub mod RemoteDifferentialCompression;
#[cfg(feature = "WebSocket")]
pub mod WebSocket;
#[cfg(feature = "WinHttp")]
pub mod WinHttp;
#[cfg(feature = "WinInet")]
pub mod WinInet;
#[cfg(feature = "WinSock")]
pub mod WinSock;
#[cfg(feature = "WindowsWebServices")]
pub mod WindowsWebServices;