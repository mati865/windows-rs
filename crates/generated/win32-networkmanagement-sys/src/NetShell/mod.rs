#[link(name = "windows")]
extern "system" {
    pub fn MatchEnumTag(hmodule: ::win32_foundation_sys::HANDLE, pwcarg: ::windows_core_sys::PCWSTR, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32;
    pub fn MatchToken(pwszusertoken: ::windows_core_sys::PCWSTR, pwszcmdtoken: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::BOOL;
    pub fn PreprocessCommand(hmodule: ::win32_foundation_sys::HANDLE, ppwcarguments: *mut ::windows_core_sys::PWSTR, dwcurrentindex: u32, dwargcount: u32, ptttags: *mut TAG_TYPE, dwtagcount: u32, dwminargs: u32, dwmaxargs: u32, pdwtagtype: *mut u32) -> u32;
    pub fn PrintError(hmodule: ::win32_foundation_sys::HANDLE, dwerrid: u32) -> u32;
    pub fn PrintMessage(pwszformat: ::windows_core_sys::PCWSTR) -> u32;
    pub fn PrintMessageFromModule(hmodule: ::win32_foundation_sys::HANDLE, dwmsgid: u32) -> u32;
    pub fn RegisterContext(pchildcontext: *const NS_CONTEXT_ATTRIBUTES) -> u32;
    pub fn RegisterHelper(pguidparentcontext: *const ::windows_core_sys::GUID, pfnregistersubcontext: *const NS_HELPER_ATTRIBUTES) -> u32;
}
#[repr(C)]
pub struct CMD_ENTRY {
    pub pwszCmdToken: ::windows_core_sys::PCWSTR,
    pub pfnCmdHandler: PFN_HANDLE_CMD,
    pub dwShortCmdHelpToken: u32,
    pub dwCmdHlpToken: u32,
    pub dwFlags: u32,
    pub pOsVersionCheck: PNS_OSVERSIONCHECK,
}
impl ::core::marker::Copy for CMD_ENTRY {}
impl ::core::clone::Clone for CMD_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CMD_GROUP_ENTRY {
    pub pwszCmdGroupToken: ::windows_core_sys::PCWSTR,
    pub dwShortCmdHelpToken: u32,
    pub ulCmdGroupSize: u32,
    pub dwFlags: u32,
    pub pCmdGroup: *mut CMD_ENTRY,
    pub pOsVersionCheck: PNS_OSVERSIONCHECK,
}
impl ::core::marker::Copy for CMD_GROUP_ENTRY {}
impl ::core::clone::Clone for CMD_GROUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEFAULT_CONTEXT_PRIORITY: u32 = 100u32;
pub const ERROR_CMD_NOT_FOUND: u32 = 15004u32;
pub const ERROR_CONTEXT_ALREADY_REGISTERED: u32 = 15019u32;
pub const ERROR_CONTINUE_IN_PARENT_CONTEXT: u32 = 15016u32;
pub const ERROR_DLL_LOAD_FAILED: u32 = 15006u32;
pub const ERROR_ENTRY_PT_NOT_FOUND: u32 = 15005u32;
pub const ERROR_HELPER_ALREADY_REGISTERED: u32 = 15018u32;
pub const ERROR_INIT_DISPLAY: u32 = 15007u32;
pub const ERROR_INVALID_OPTION_TAG: u32 = 15009u32;
pub const ERROR_INVALID_OPTION_VALUE: u32 = 15014u32;
pub const ERROR_INVALID_SYNTAX: u32 = 15001u32;
pub const ERROR_MISSING_OPTION: u32 = 15011u32;
pub const ERROR_NO_CHANGE: u32 = 15003u32;
pub const ERROR_NO_ENTRIES: u32 = 15000u32;
pub const ERROR_NO_TAG: u32 = 15010u32;
pub const ERROR_OKAY: u32 = 15015u32;
pub const ERROR_PARSING_FAILURE: u32 = 15020u32;
pub const ERROR_PROTOCOL_NOT_IN_TRANSPORT: u32 = 15002u32;
pub const ERROR_SHOW_USAGE: u32 = 15013u32;
pub const ERROR_SUPPRESS_OUTPUT: u32 = 15017u32;
pub const ERROR_TAG_ALREADY_PRESENT: u32 = 15008u32;
pub const ERROR_TRANSPORT_NOT_PRESENT: u32 = 15012u32;
pub const GET_RESOURCE_STRING_FN_NAME: &str = "GetResourceString";
pub const MAX_NAME_LEN: u32 = 48u32;
pub const NETSH_ARG_DELIMITER: &str = "=";
pub const NETSH_CMD_DELIMITER: &str = " ";
pub const NETSH_ERROR_BASE: u32 = 15000u32;
pub const NETSH_ERROR_END: u32 = 15019u32;
pub const NETSH_MAX_CMD_TOKEN_LENGTH: u32 = 128u32;
pub const NETSH_MAX_TOKEN_LENGTH: u32 = 64u32;
pub const NETSH_VERSION_50: u32 = 20480u32;
pub type NS_CMD_FLAGS = i32;
pub const CMD_FLAG_PRIVATE: NS_CMD_FLAGS = 1i32;
pub const CMD_FLAG_INTERACTIVE: NS_CMD_FLAGS = 2i32;
pub const CMD_FLAG_LOCAL: NS_CMD_FLAGS = 8i32;
pub const CMD_FLAG_ONLINE: NS_CMD_FLAGS = 16i32;
pub const CMD_FLAG_HIDDEN: NS_CMD_FLAGS = 32i32;
pub const CMD_FLAG_LIMIT_MASK: NS_CMD_FLAGS = 65535i32;
pub const CMD_FLAG_PRIORITY: NS_CMD_FLAGS = -2147483648i32;
#[repr(C)]
pub struct NS_CONTEXT_ATTRIBUTES {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0,
    pub pwszContext: ::windows_core_sys::PWSTR,
    pub guidHelper: ::windows_core_sys::GUID,
    pub dwFlags: u32,
    pub ulPriority: u32,
    pub ulNumTopCmds: u32,
    pub pTopCmds: *mut CMD_ENTRY,
    pub ulNumGroups: u32,
    pub pCmdGroups: *mut CMD_GROUP_ENTRY,
    pub pfnCommitFn: PNS_CONTEXT_COMMIT_FN,
    pub pfnDumpFn: PNS_CONTEXT_DUMP_FN,
    pub pfnConnectFn: PNS_CONTEXT_CONNECT_FN,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pfnOsVersionCheck: PNS_OSVERSIONCHECK,
}
impl ::core::marker::Copy for NS_CONTEXT_ATTRIBUTES {}
impl ::core::clone::Clone for NS_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union NS_CONTEXT_ATTRIBUTES_0 {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
impl ::core::marker::Copy for NS_CONTEXT_ATTRIBUTES_0 {}
impl ::core::clone::Clone for NS_CONTEXT_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NS_CONTEXT_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for NS_CONTEXT_ATTRIBUTES_0_0 {}
impl ::core::clone::Clone for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NS_EVENTS = i32;
pub const NS_EVENT_LOOP: NS_EVENTS = 65536i32;
pub const NS_EVENT_LAST_N: NS_EVENTS = 1i32;
pub const NS_EVENT_LAST_SECS: NS_EVENTS = 2i32;
pub const NS_EVENT_FROM_N: NS_EVENTS = 4i32;
pub const NS_EVENT_FROM_START: NS_EVENTS = 8i32;
pub const NS_GET_EVENT_IDS_FN_NAME: &str = "GetEventIds";
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0,
    pub guidHelper: ::windows_core_sys::GUID,
    pub pfnStart: PNS_HELPER_START_FN,
    pub pfnStop: PNS_HELPER_STOP_FN,
}
impl ::core::marker::Copy for NS_HELPER_ATTRIBUTES {}
impl ::core::clone::Clone for NS_HELPER_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union NS_HELPER_ATTRIBUTES_0 {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
impl ::core::marker::Copy for NS_HELPER_ATTRIBUTES_0 {}
impl ::core::clone::Clone for NS_HELPER_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for NS_HELPER_ATTRIBUTES_0_0 {}
impl ::core::clone::Clone for NS_HELPER_ATTRIBUTES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NS_MODE_CHANGE = i32;
pub const NETSH_COMMIT: NS_MODE_CHANGE = 0i32;
pub const NETSH_UNCOMMIT: NS_MODE_CHANGE = 1i32;
pub const NETSH_FLUSH: NS_MODE_CHANGE = 2i32;
pub const NETSH_COMMIT_STATE: NS_MODE_CHANGE = 3i32;
pub const NETSH_SAVE: NS_MODE_CHANGE = 4i32;
pub type NS_REQS = i32;
pub const NS_REQ_ZERO: NS_REQS = 0i32;
pub const NS_REQ_PRESENT: NS_REQS = 1i32;
pub const NS_REQ_ALLOW_MULTIPLE: NS_REQS = 2i32;
pub const NS_REQ_ONE_OR_MORE: NS_REQS = 3i32;
pub type PFN_HANDLE_CMD = ::core::option::Option<unsafe extern "system" fn(pwszmachine: ::windows_core_sys::PCWSTR, ppwcarguments: *mut ::windows_core_sys::PWSTR, dwcurrentindex: u32, dwargcount: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void, pbdone: *mut ::win32_foundation_sys::BOOL) -> u32>;
pub type PGET_RESOURCE_STRING_FN = ::core::option::Option<unsafe extern "system" fn(dwmsgid: u32, lpbuffer: ::windows_core_sys::PCWSTR, nbuffermax: u32) -> u32>;
pub type PNS_CONTEXT_COMMIT_FN = ::core::option::Option<unsafe extern "system" fn(dwaction: u32) -> u32>;
pub type PNS_CONTEXT_CONNECT_FN = ::core::option::Option<unsafe extern "system" fn(pwszmachine: ::windows_core_sys::PCWSTR) -> u32>;
pub type PNS_CONTEXT_DUMP_FN = ::core::option::Option<unsafe extern "system" fn(pwszrouter: ::windows_core_sys::PCWSTR, ppwcarguments: *const ::windows_core_sys::PWSTR, dwargcount: u32, pvdata: *const ::core::ffi::c_void) -> u32>;
pub type PNS_DLL_INIT_FN = ::core::option::Option<unsafe extern "system" fn(dwnetshversion: u32, preserved: *mut ::core::ffi::c_void) -> u32>;
pub type PNS_DLL_STOP_FN = ::core::option::Option<unsafe extern "system" fn(dwreserved: u32) -> u32>;
pub type PNS_HELPER_START_FN = ::core::option::Option<unsafe extern "system" fn(pguidparent: *const ::windows_core_sys::GUID, dwversion: u32) -> u32>;
pub type PNS_HELPER_STOP_FN = ::core::option::Option<unsafe extern "system" fn(dwreserved: u32) -> u32>;
pub type PNS_OSVERSIONCHECK = ::core::option::Option<unsafe extern "system" fn(cimostype: u32, cimosproductsuite: u32, cimosversion: ::windows_core_sys::PCWSTR, cimosbuildnumber: ::windows_core_sys::PCWSTR, cimservicepackmajorversion: ::windows_core_sys::PCWSTR, cimservicepackminorversion: ::windows_core_sys::PCWSTR, uireserved: u32, dwreserved: u32) -> ::win32_foundation_sys::BOOL>;
#[repr(C)]
pub struct TAG_TYPE {
    pub pwszTag: ::windows_core_sys::PCWSTR,
    pub dwRequired: u32,
    pub bPresent: ::win32_foundation_sys::BOOL,
}
impl ::core::marker::Copy for TAG_TYPE {}
impl ::core::clone::Clone for TAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_VALUE {
    pub pwszToken: ::windows_core_sys::PCWSTR,
    pub dwValue: u32,
}
impl ::core::marker::Copy for TOKEN_VALUE {}
impl ::core::clone::Clone for TOKEN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
