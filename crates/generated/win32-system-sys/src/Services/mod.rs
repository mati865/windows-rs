#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Security")]
    pub fn ChangeServiceConfig2A(hservice: ::win32_security_sys::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ChangeServiceConfig2W(hservice: ::win32_security_sys::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ChangeServiceConfigA(hservice: ::win32_security_sys::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows_core_sys::PCSTR, lploadordergroup: ::windows_core_sys::PCSTR, lpdwtagid: *mut u32, lpdependencies: ::windows_core_sys::PCSTR, lpservicestartname: ::windows_core_sys::PCSTR, lppassword: ::windows_core_sys::PCSTR, lpdisplayname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ChangeServiceConfigW(hservice: ::win32_security_sys::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows_core_sys::PCWSTR, lploadordergroup: ::windows_core_sys::PCWSTR, lpdwtagid: *mut u32, lpdependencies: ::windows_core_sys::PCWSTR, lpservicestartname: ::windows_core_sys::PCWSTR, lppassword: ::windows_core_sys::PCWSTR, lpdisplayname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn CloseServiceHandle(hscobject: ::win32_security_sys::SC_HANDLE) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ControlService(hservice: ::win32_security_sys::SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ControlServiceExA(hservice: ::win32_security_sys::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn ControlServiceExW(hservice: ::win32_security_sys::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn CreateServiceA(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCSTR, lpdisplayname: ::windows_core_sys::PCSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows_core_sys::PCSTR, lploadordergroup: ::windows_core_sys::PCSTR, lpdwtagid: *mut u32, lpdependencies: ::windows_core_sys::PCSTR, lpservicestartname: ::windows_core_sys::PCSTR, lppassword: ::windows_core_sys::PCSTR) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn CreateServiceW(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCWSTR, lpdisplayname: ::windows_core_sys::PCWSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows_core_sys::PCWSTR, lploadordergroup: ::windows_core_sys::PCWSTR, lpdwtagid: *mut u32, lpdependencies: ::windows_core_sys::PCWSTR, lpservicestartname: ::windows_core_sys::PCWSTR, lppassword: ::windows_core_sys::PCWSTR) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn DeleteService(hservice: ::win32_security_sys::SC_HANDLE) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumDependentServicesA(hservice: ::win32_security_sys::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumDependentServicesW(hservice: ::win32_security_sys::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumServicesStatusA(hscmanager: ::win32_security_sys::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumServicesStatusExA(hscmanager: ::win32_security_sys::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: ::windows_core_sys::PCSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumServicesStatusExW(hscmanager: ::win32_security_sys::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn EnumServicesStatusW(hscmanager: ::win32_security_sys::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: ::windows_core_sys::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn GetServiceDisplayNameA(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCSTR, lpdisplayname: ::windows_core_sys::PSTR, lpcchbuffer: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn GetServiceDisplayNameW(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCWSTR, lpdisplayname: ::windows_core_sys::PWSTR, lpcchbuffer: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn GetServiceKeyNameA(hscmanager: ::win32_security_sys::SC_HANDLE, lpdisplayname: ::windows_core_sys::PCSTR, lpservicename: ::windows_core_sys::PSTR, lpcchbuffer: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn GetServiceKeyNameW(hscmanager: ::win32_security_sys::SC_HANDLE, lpdisplayname: ::windows_core_sys::PCWSTR, lpservicename: ::windows_core_sys::PWSTR, lpcchbuffer: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn GetSharedServiceDirectory(servicehandle: ::win32_security_sys::SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: ::windows_core_sys::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn GetSharedServiceRegistryStateKey(servicehandle: ::win32_security_sys::SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn LockServiceDatabase(hscmanager: ::win32_security_sys::SC_HANDLE) -> *mut ::core::ffi::c_void;
    pub fn NotifyBootConfigStatus(bootacceptable: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn NotifyServiceStatusChangeA(hservice: ::win32_security_sys::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn NotifyServiceStatusChangeW(hservice: ::win32_security_sys::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn OpenSCManagerA(lpmachinename: ::windows_core_sys::PCSTR, lpdatabasename: ::windows_core_sys::PCSTR, dwdesiredaccess: u32) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn OpenSCManagerW(lpmachinename: ::windows_core_sys::PCWSTR, lpdatabasename: ::windows_core_sys::PCWSTR, dwdesiredaccess: u32) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn OpenServiceA(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCSTR, dwdesiredaccess: u32) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn OpenServiceW(hscmanager: ::win32_security_sys::SC_HANDLE, lpservicename: ::windows_core_sys::PCWSTR, dwdesiredaccess: u32) -> ::win32_security_sys::SC_HANDLE;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceConfig2A(hservice: ::win32_security_sys::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceConfig2W(hservice: ::win32_security_sys::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceConfigA(hservice: ::win32_security_sys::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceConfigW(hservice: ::win32_security_sys::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceLockStatusA(hscmanager: ::win32_security_sys::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceLockStatusW(hscmanager: ::win32_security_sys::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceObjectSecurity(hservice: ::win32_security_sys::SC_HANDLE, dwsecurityinformation: u32, lpsecuritydescriptor: ::win32_security_sys::PSECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceStatus(hservice: ::win32_security_sys::SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn QueryServiceStatusEx(hservice: ::win32_security_sys::SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn RegisterServiceCtrlHandlerA(lpservicename: ::windows_core_sys::PCSTR, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerExA(lpservicename: ::windows_core_sys::PCSTR, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerExW(lpservicename: ::windows_core_sys::PCWSTR, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerW(lpservicename: ::windows_core_sys::PCWSTR, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE;
    pub fn SetServiceBits(hservicestatus: SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: ::win32_foundation_sys::BOOL, bupdateimmediately: ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn SetServiceObjectSecurity(hservice: ::win32_security_sys::SC_HANDLE, dwsecurityinformation: ::win32_security_sys::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: ::win32_security_sys::PSECURITY_DESCRIPTOR) -> ::win32_foundation_sys::BOOL;
    pub fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn StartServiceA(hservice: ::win32_security_sys::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const ::windows_core_sys::PSTR) -> ::win32_foundation_sys::BOOL;
    pub fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> ::win32_foundation_sys::BOOL;
    pub fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn StartServiceW(hservice: ::win32_security_sys::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn WaitServiceState(hservice: ::win32_security_sys::SC_HANDLE, dwnotify: u32, dwtimeout: u32, hcancelevent: ::win32_foundation_sys::HANDLE) -> u32;
}
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 762980374, data2: 3166, data3: 17916, data4: [156, 231, 87, 14, 94, 205, 233, 201] };
pub const DOMAIN_JOIN_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 484575930, data2: 38993, data3: 17441, data4: [148, 48, 29, 222, 183, 102, 232, 9] };
pub const DOMAIN_LEAVE_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3719254382, data2: 22722, data3: 18534, data4: [149, 116, 195, 182, 21, 212, 46, 161] };
pub type ENUM_SERVICE_STATE = u32;
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = 1u32;
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = 2u32;
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = 3u32;
#[repr(C)]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: ::windows_core_sys::PSTR,
    pub lpDisplayName: ::windows_core_sys::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUSA {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: ::windows_core_sys::PWSTR,
    pub lpDisplayName: ::windows_core_sys::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUSW {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: ::windows_core_sys::PSTR,
    pub lpDisplayName: ::windows_core_sys::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSA {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: ::windows_core_sys::PWSTR,
    pub lpDisplayName: ::windows_core_sys::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSW {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ENUM_SERVICE_TYPE = u32;
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = 11u32;
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = 1u32;
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = 48u32;
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = 32u32;
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = 4u32;
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = 2u32;
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = 8u32;
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = 16u32;
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = 80u32;
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = 96u32;
pub const FIREWALL_PORT_CLOSE_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2705648952, data2: 36370, data3: 19940, data4: [157, 150, 230, 71, 64, 177, 165, 36] };
pub const FIREWALL_PORT_OPEN_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3075907079, data2: 33825, data3: 20192, data4: [173, 16, 134, 145, 90, 253, 173, 9] };
pub type HANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type HANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
pub type LPHANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type LPHANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
pub type LPSERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows_core_sys::PSTR)>;
pub type LPSERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows_core_sys::PWSTR)>;
pub const MACHINE_POLICY_PRESENT_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1704970982, data2: 23515, data3: 19881, data4: [177, 255, 202, 42, 23, 141, 70, 224] };
pub const NAMED_PIPE_EVENT_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 528601393, data2: 16300, data3: 17719, data4: [158, 12, 126, 123, 12, 47, 75, 85] };
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1328018142, data2: 5346, data3: 17163, data4: [165, 73, 124, 212, 140, 188, 130, 69] };
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3427509802, data2: 5678, data3: 17992, data4: [132, 122, 182, 189, 249, 147, 227, 53] };
pub type PFN_SC_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pparameter: *const ::core::ffi::c_void)>;
pub type PSC_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const ::core::ffi::c_void)>;
#[repr(C)]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: ::windows_core_sys::PSTR,
    pub lpLoadOrderGroup: ::windows_core_sys::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: ::windows_core_sys::PSTR,
    pub lpServiceStartName: ::windows_core_sys::PSTR,
    pub lpDisplayName: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGA {}
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: ::windows_core_sys::PWSTR,
    pub lpLoadOrderGroup: ::windows_core_sys::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: ::windows_core_sys::PWSTR,
    pub lpServiceStartName: ::windows_core_sys::PWSTR,
    pub lpDisplayName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGW {}
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: ::windows_core_sys::PSTR,
    pub dwLockDuration: u32,
}
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSA {}
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: ::windows_core_sys::PWSTR,
    pub dwLockDuration: u32,
}
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSW {}
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RPC_INTERFACE_EVENT_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3163607399, data2: 38000, data3: 16697, data4: [169, 186, 190, 11, 187, 245, 183, 77] };
#[repr(C)]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
impl ::core::marker::Copy for SC_ACTION {}
impl ::core::clone::Clone for SC_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SC_ACTION_TYPE = i32;
pub const SC_ACTION_NONE: SC_ACTION_TYPE = 0i32;
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = 1i32;
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = 2i32;
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = 3i32;
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = 4i32;
pub const SC_AGGREGATE_STORAGE_KEY: &str = "System\\CurrentControlSet\\Control\\ServiceAggregatedEvents";
pub type SC_ENUM_TYPE = i32;
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = 0i32;
pub type SC_EVENT_TYPE = i32;
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = 0i32;
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = 1i32;
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = 2i32;
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
pub const SC_MANAGER_CONNECT: u32 = 1u32;
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
pub const SC_MANAGER_LOCK: u32 = 8u32;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
pub type SC_STATUS_TYPE = i32;
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = 0i32;
pub const SERVICES_ACTIVE_DATABASE: &str = "ServicesActive";
pub const SERVICES_ACTIVE_DATABASEA: &str = "ServicesActive";
pub const SERVICES_ACTIVE_DATABASEW: &str = "ServicesActive";
pub const SERVICES_FAILED_DATABASE: &str = "ServicesFailed";
pub const SERVICES_FAILED_DATABASEA: &str = "ServicesFailed";
pub const SERVICES_FAILED_DATABASEW: &str = "ServicesFailed";
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
pub type SERVICE_CONFIG = u32;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = 3u32;
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = 1u32;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = 2u32;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = 4u32;
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = 9u32;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = 7u32;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = 6u32;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = 5u32;
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = 8u32;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = 12u32;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
#[repr(C)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: ::windows_core_sys::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: ::windows_core_sys::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
#[repr(C)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for SERVICE_DELAYED_AUTO_START_INFO {}
impl ::core::clone::Clone for SERVICE_DELAYED_AUTO_START_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for SERVICE_DESCRIPTIONA {}
impl ::core::clone::Clone for SERVICE_DESCRIPTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for SERVICE_DESCRIPTIONW {}
impl ::core::clone::Clone for SERVICE_DESCRIPTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SERVICE_DIRECTORY_TYPE = i32;
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = 0i32;
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = 1i32;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
pub type SERVICE_ERROR = u32;
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = 3u32;
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = 0u32;
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = 1u32;
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = 2u32;
#[repr(C)]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: ::windows_core_sys::PSTR,
    pub lpCommand: ::windows_core_sys::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSA {}
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: ::windows_core_sys::PWSTR,
    pub lpCommand: ::windows_core_sys::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSW {}
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONS_FLAG {}
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_INTERROGATE: u32 = 128u32;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
#[repr(C)]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
impl ::core::marker::Copy for SERVICE_LAUNCH_PROTECTED_INFO {}
impl ::core::clone::Clone for SERVICE_LAUNCH_PROTECTED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
pub type SERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8)>;
pub type SERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows_core_sys::PWSTR)>;
pub type SERVICE_NOTIFY = u32;
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = 128u32;
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = 16u32;
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = 512u32;
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = 256u32;
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = 32u32;
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = 64u32;
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = 8u32;
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = 2u32;
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = 4u32;
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = 1u32;
#[repr(C)]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_1 {}
impl ::core::clone::Clone for SERVICE_NOTIFY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_2A {}
impl ::core::clone::Clone for SERVICE_NOTIFY_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_2W {}
impl ::core::clone::Clone for SERVICE_NOTIFY_2W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
#[repr(C)]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: ::win32_foundation_sys::BOOLEAN,
}
impl ::core::marker::Copy for SERVICE_PREFERRED_NODE_INFO {}
impl ::core::clone::Clone for SERVICE_PREFERRED_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
impl ::core::marker::Copy for SERVICE_PRESHUTDOWN_INFO {}
impl ::core::clone::Clone for SERVICE_PRESHUTDOWN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
pub type SERVICE_REGISTRY_STATE_TYPE = i32;
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = 0i32;
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = 1i32;
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = 2i32;
#[repr(C)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SERVICE_RUNS_IN_PROCESS = u32;
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = 0u32;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = 1u32;
pub type SERVICE_SHARED_DIRECTORY_TYPE = i32;
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = 0i32;
pub type SERVICE_SHARED_REGISTRY_STATE_TYPE = i32;
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = 0i32;
#[repr(C)]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
impl ::core::marker::Copy for SERVICE_SID_INFO {}
impl ::core::clone::Clone for SERVICE_SID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
pub const SERVICE_START: u32 = 16u32;
#[repr(C)]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
impl ::core::marker::Copy for SERVICE_START_REASON {}
impl ::core::clone::Clone for SERVICE_START_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
pub type SERVICE_START_TYPE = u32;
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = 2u32;
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = 0u32;
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = 3u32;
pub const SERVICE_DISABLED: SERVICE_START_TYPE = 4u32;
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = 1u32;
#[repr(C)]
pub struct SERVICE_STATUS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
impl ::core::marker::Copy for SERVICE_STATUS {}
impl ::core::clone::Clone for SERVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SERVICE_STATUS_CURRENT_STATE = u32;
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = 5u32;
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = 6u32;
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = 7u32;
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = 4u32;
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = 2u32;
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = 3u32;
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = 1u32;
pub type SERVICE_STATUS_HANDLE = isize;
#[repr(C)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: SERVICE_RUNS_IN_PROCESS,
}
impl ::core::marker::Copy for SERVICE_STATUS_PROCESS {}
impl ::core::clone::Clone for SERVICE_STATUS_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SERVICE_STOP: u32 = 32u32;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
#[repr(C)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: ::windows_core_sys::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYA {}
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: ::windows_core_sys::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYW {}
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
impl ::core::marker::Copy for SERVICE_TIMECHANGE_INFO {}
impl ::core::clone::Clone for SERVICE_TIMECHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: SERVICE_TRIGGER_TYPE,
    pub dwAction: SERVICE_TRIGGER_ACTION,
    pub pTriggerSubtype: *mut ::windows_core_sys::GUID,
    pub cDataItems: u32,
    pub pDataItems: *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
impl ::core::marker::Copy for SERVICE_TRIGGER {}
impl ::core::clone::Clone for SERVICE_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SERVICE_TRIGGER_ACTION = u32;
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = 1u32;
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = 2u32;
#[repr(C)]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for SERVICE_TRIGGER_CUSTOM_STATE_ID {}
impl ::core::clone::Clone for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: *mut SERVICE_TRIGGER,
    pub pReserved: *mut u8,
}
impl ::core::marker::Copy for SERVICE_TRIGGER_INFO {}
impl ::core::clone::Clone for SERVICE_TRIGGER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE,
    pub cbData: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {}
impl ::core::clone::Clone for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = u32;
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 1u32;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 2u32;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 3u32;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 4u32;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 5u32;
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: &str = "TriggerStarted";
pub type SERVICE_TRIGGER_TYPE = u32;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = 20u32;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = 1u32;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = 3u32;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = 4u32;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = 5u32;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = 2u32;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = 6u32;
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
pub const USER_POLICY_PRESENT_GUID: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1425753800, data2: 61577, data3: 17996, data4: [177, 253, 89, 209, 182, 44, 59, 80] };
#[repr(C)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);
