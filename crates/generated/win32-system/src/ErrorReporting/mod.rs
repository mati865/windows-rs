pub const APPCRASH_EVENT: &str = "APPCRASH";
#[inline]
pub unsafe fn AddERExcludedApplicationA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(szapplication: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddERExcludedApplicationA(szapplication: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AddERExcludedApplicationA(szapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AddERExcludedApplicationW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(wszapplication: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddERExcludedApplicationW(wszapplication: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AddERExcludedApplicationW(wszapplication.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EFaultRepRetVal(pub i32);
pub const frrvOk: EFaultRepRetVal = EFaultRepRetVal(0i32);
pub const frrvOkManifest: EFaultRepRetVal = EFaultRepRetVal(1i32);
pub const frrvOkQueued: EFaultRepRetVal = EFaultRepRetVal(2i32);
pub const frrvErr: EFaultRepRetVal = EFaultRepRetVal(3i32);
pub const frrvErrNoDW: EFaultRepRetVal = EFaultRepRetVal(4i32);
pub const frrvErrTimeout: EFaultRepRetVal = EFaultRepRetVal(5i32);
pub const frrvLaunchDebugger: EFaultRepRetVal = EFaultRepRetVal(6i32);
pub const frrvOkHeadless: EFaultRepRetVal = EFaultRepRetVal(7i32);
pub const frrvErrAnotherInstance: EFaultRepRetVal = EFaultRepRetVal(8i32);
pub const frrvErrNoMemory: EFaultRepRetVal = EFaultRepRetVal(9i32);
pub const frrvErrDoubleFault: EFaultRepRetVal = EFaultRepRetVal(10i32);
impl ::core::marker::Copy for EFaultRepRetVal {}
impl ::core::clone::Clone for EFaultRepRetVal {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EFaultRepRetVal {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EFaultRepRetVal {
    type Abi = Self;
}
impl ::core::fmt::Debug for EFaultRepRetVal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EFaultRepRetVal").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HREPORT(pub isize);
impl HREPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HREPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HREPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HREPORT {}
impl ::core::fmt::Debug for HREPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HREPORT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HREPORT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HREPORTSTORE(pub isize);
impl HREPORTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HREPORTSTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HREPORTSTORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HREPORTSTORE {}
impl ::core::fmt::Debug for HREPORTSTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HREPORTSTORE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HREPORTSTORE {
    type Abi = Self;
}
pub const PACKAGED_APPCRASH_EVENT: &str = "MoAppCrash";
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut ::win32_foundation::BOOL, pwszdebuggerlaunch: ::windows_core::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut ::win32_foundation::BOOL, pwszeventname: ::windows_core::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> ::windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: ::windows_core::PWSTR, pchname: *mut u32, pwszvalue: ::windows_core::PWSTR, pchvalue: *mut u32) -> ::windows_core::HRESULT>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REPORT_STORE_TYPES(pub i32);
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(0i32);
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(1i32);
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(2i32);
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(3i32);
pub const E_STORE_INVALID: REPORT_STORE_TYPES = REPORT_STORE_TYPES(4i32);
impl ::core::marker::Copy for REPORT_STORE_TYPES {}
impl ::core::clone::Clone for REPORT_STORE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPORT_STORE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REPORT_STORE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for REPORT_STORE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_STORE_TYPES").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal;
        }
        ::core::mem::transmute(ReportFault(::core::mem::transmute(pep), ::core::mem::transmute(dwopt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_CONSENT(pub i32);
pub const WerConsentNotAsked: WER_CONSENT = WER_CONSENT(1i32);
pub const WerConsentApproved: WER_CONSENT = WER_CONSENT(2i32);
pub const WerConsentDenied: WER_CONSENT = WER_CONSENT(3i32);
pub const WerConsentAlwaysPrompt: WER_CONSENT = WER_CONSENT(4i32);
pub const WerConsentMax: WER_CONSENT = WER_CONSENT(5i32);
impl ::core::marker::Copy for WER_CONSENT {}
impl ::core::clone::Clone for WER_CONSENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_CONSENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_CONSENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_CONSENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_CONSENT").field(&self.0).finish()
    }
}
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: ::win32_foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
}
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS {}
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_DUMP_CUSTOM_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_DUMP_CUSTOM_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS {}
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: ::win32_foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
}
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS_V2 {}
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_DUMP_CUSTOM_OPTIONS_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_DUMP_CUSTOM_OPTIONS_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V2 {}
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: ::win32_foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
    pub pvDumpKey: *mut ::core::ffi::c_void,
    pub hSnapshot: ::win32_foundation::HANDLE,
    pub dwThreadID: u32,
}
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS_V3 {}
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V3")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .field("pvDumpKey", &self.pvDumpKey)
            .field("hSnapshot", &self.hSnapshot)
            .field("dwThreadID", &self.dwThreadID)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_DUMP_CUSTOM_OPTIONS_V3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_DUMP_CUSTOM_OPTIONS_V3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V3 {}
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WER_DUMP_MASK_START: u32 = 1u32;
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_DUMP_TYPE(pub i32);
pub const WerDumpTypeNone: WER_DUMP_TYPE = WER_DUMP_TYPE(0i32);
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = WER_DUMP_TYPE(1i32);
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = WER_DUMP_TYPE(2i32);
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = WER_DUMP_TYPE(3i32);
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = WER_DUMP_TYPE(4i32);
pub const WerDumpTypeMax: WER_DUMP_TYPE = WER_DUMP_TYPE(5i32);
impl ::core::marker::Copy for WER_DUMP_TYPE {}
impl ::core::clone::Clone for WER_DUMP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_DUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_DUMP_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_DUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_DUMP_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct WER_EXCEPTION_INFORMATION {
    pub pExceptionPointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
    pub bClientPointers: ::win32_foundation::BOOL,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for WER_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for WER_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_EXCEPTION_INFORMATION").field("pExceptionPointers", &self.pExceptionPointers).field("bClientPointers", &self.bClientPointers).finish()
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
unsafe impl ::windows_core::Abi for WER_EXCEPTION_INFORMATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for WER_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_EXCEPTION_INFORMATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_FAULT_REPORTING(pub u32);
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: WER_FAULT_REPORTING = WER_FAULT_REPORTING(4u32);
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: WER_FAULT_REPORTING = WER_FAULT_REPORTING(1u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE: WER_FAULT_REPORTING = WER_FAULT_REPORTING(2u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: WER_FAULT_REPORTING = WER_FAULT_REPORTING(8u32);
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: WER_FAULT_REPORTING = WER_FAULT_REPORTING(16u32);
impl ::core::marker::Copy for WER_FAULT_REPORTING {}
impl ::core::clone::Clone for WER_FAULT_REPORTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FAULT_REPORTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_FAULT_REPORTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_FAULT_REPORTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FAULT_REPORTING").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FAULT_REPORTING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FAULT_REPORTING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FAULT_REPORTING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_FILE(pub u32);
pub const WER_FILE_ANONYMOUS_DATA: WER_FILE = WER_FILE(2u32);
pub const WER_FILE_DELETE_WHEN_DONE: WER_FILE = WER_FILE(1u32);
impl ::core::marker::Copy for WER_FILE {}
impl ::core::clone::Clone for WER_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FILE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_FILE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_FILE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FILE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FILE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FILE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FILE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WER_FILE_COMPRESSED: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_FILE_TYPE(pub i32);
pub const WerFileTypeMicrodump: WER_FILE_TYPE = WER_FILE_TYPE(1i32);
pub const WerFileTypeMinidump: WER_FILE_TYPE = WER_FILE_TYPE(2i32);
pub const WerFileTypeHeapdump: WER_FILE_TYPE = WER_FILE_TYPE(3i32);
pub const WerFileTypeUserDocument: WER_FILE_TYPE = WER_FILE_TYPE(4i32);
pub const WerFileTypeOther: WER_FILE_TYPE = WER_FILE_TYPE(5i32);
pub const WerFileTypeTriagedump: WER_FILE_TYPE = WER_FILE_TYPE(6i32);
pub const WerFileTypeCustomDump: WER_FILE_TYPE = WER_FILE_TYPE(7i32);
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = WER_FILE_TYPE(8i32);
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = WER_FILE_TYPE(9i32);
pub const WerFileTypeMax: WER_FILE_TYPE = WER_FILE_TYPE(10i32);
impl ::core::marker::Copy for WER_FILE_TYPE {}
impl ::core::clone::Clone for WER_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_FILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE_TYPE").field(&self.0).finish()
    }
}
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
pub const WER_P0: u32 = 0u32;
pub const WER_P1: u32 = 1u32;
pub const WER_P2: u32 = 2u32;
pub const WER_P3: u32 = 3u32;
pub const WER_P4: u32 = 4u32;
pub const WER_P5: u32 = 5u32;
pub const WER_P6: u32 = 6u32;
pub const WER_P7: u32 = 7u32;
pub const WER_P8: u32 = 8u32;
pub const WER_P9: u32 = 9u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_REGISTER_FILE_TYPE(pub i32);
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(1i32);
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(2i32);
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(3i32);
impl ::core::marker::Copy for WER_REGISTER_FILE_TYPE {}
impl ::core::clone::Clone for WER_REGISTER_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REGISTER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_REGISTER_FILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_REGISTER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REGISTER_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WER_REPORT_INFORMATION {
    pub dwSize: u32,
    pub hProcess: ::win32_foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: ::win32_foundation::HWND,
}
impl ::core::marker::Copy for WER_REPORT_INFORMATION {}
impl ::core::clone::Clone for WER_REPORT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION").field("dwSize", &self.dwSize).field("hProcess", &self.hProcess).field("wzConsentKey", &self.wzConsentKey).field("wzFriendlyEventName", &self.wzFriendlyEventName).field("wzApplicationName", &self.wzApplicationName).field("wzApplicationPath", &self.wzApplicationPath).field("wzDescription", &self.wzDescription).field("hwndParent", &self.hwndParent).finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_INFORMATION {}
impl ::core::default::Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V3 {
    pub dwSize: u32,
    pub hProcess: ::win32_foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: ::win32_foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
}
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V3 {}
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V3")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_INFORMATION_V3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_INFORMATION_V3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V3 {}
impl ::core::default::Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V4 {
    pub dwSize: u32,
    pub hProcess: ::win32_foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: ::win32_foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: ::win32_foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: ::win32_foundation::HANDLE,
}
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V4 {}
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V4")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_INFORMATION_V4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_INFORMATION_V4>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V4 {}
impl ::core::default::Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V5 {
    pub dwSize: u32,
    pub hProcess: ::win32_foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: ::win32_foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: ::win32_foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: ::win32_foundation::HANDLE,
    pub submitResultMax: WER_SUBMIT_RESULT,
}
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V5 {}
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V5")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .field("submitResultMax", &self.submitResultMax)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_INFORMATION_V5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_INFORMATION_V5>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V5 {}
impl ::core::default::Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_core::GUID,
    pub ReportId: ::windows_core::GUID,
    pub CreationTime: ::win32_foundation::FILETIME,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for WER_REPORT_METADATA_V1 {}
impl ::core::clone::Clone for WER_REPORT_METADATA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_METADATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V1").field("Signature", &self.Signature).field("BucketId", &self.BucketId).field("ReportId", &self.ReportId).field("CreationTime", &self.CreationTime).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_METADATA_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_METADATA_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_METADATA_V1 {}
impl ::core::default::Default for WER_REPORT_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_core::GUID,
    pub ReportId: ::windows_core::GUID,
    pub CreationTime: ::win32_foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WER_REPORT_METADATA_V2 {}
impl ::core::clone::Clone for WER_REPORT_METADATA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_METADATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V2")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_METADATA_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_METADATA_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_METADATA_V2 {}
impl ::core::default::Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_core::GUID,
    pub ReportId: ::windows_core::GUID,
    pub CreationTime: ::win32_foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: ::windows_core::PWSTR,
    pub FriendlyEventName: [u16; 128],
    pub ApplicationName: [u16; 128],
    pub ApplicationPath: [u16; 260],
    pub Description: [u16; 512],
    pub BucketIdString: [u16; 260],
    pub LegacyBucketId: u64,
}
impl ::core::marker::Copy for WER_REPORT_METADATA_V3 {}
impl ::core::clone::Clone for WER_REPORT_METADATA_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_METADATA_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V3")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .field("FriendlyEventName", &self.FriendlyEventName)
            .field("ApplicationName", &self.ApplicationName)
            .field("ApplicationPath", &self.ApplicationPath)
            .field("Description", &self.Description)
            .field("BucketIdString", &self.BucketIdString)
            .field("LegacyBucketId", &self.LegacyBucketId)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_METADATA_V3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_METADATA_V3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_METADATA_V3 {}
impl ::core::default::Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_PARAMETER {
    pub Name: [u16; 129],
    pub Value: [u16; 260],
}
impl ::core::marker::Copy for WER_REPORT_PARAMETER {}
impl ::core::clone::Clone for WER_REPORT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_PARAMETER").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_PARAMETER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_PARAMETER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_PARAMETER {}
impl ::core::default::Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WER_REPORT_SIGNATURE {
    pub EventName: [u16; 65],
    pub Parameters: [WER_REPORT_PARAMETER; 10],
}
impl ::core::marker::Copy for WER_REPORT_SIGNATURE {}
impl ::core::clone::Clone for WER_REPORT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WER_REPORT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_SIGNATURE").field("EventName", &self.EventName).field("Parameters", &self.Parameters).finish()
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_SIGNATURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WER_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_REPORT_SIGNATURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WER_REPORT_SIGNATURE {}
impl ::core::default::Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_REPORT_TYPE(pub i32);
pub const WerReportNonCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(0i32);
pub const WerReportCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(1i32);
pub const WerReportApplicationCrash: WER_REPORT_TYPE = WER_REPORT_TYPE(2i32);
pub const WerReportApplicationHang: WER_REPORT_TYPE = WER_REPORT_TYPE(3i32);
pub const WerReportKernel: WER_REPORT_TYPE = WER_REPORT_TYPE(4i32);
pub const WerReportInvalid: WER_REPORT_TYPE = WER_REPORT_TYPE(5i32);
impl ::core::marker::Copy for WER_REPORT_TYPE {}
impl ::core::clone::Clone for WER_REPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_REPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_REPORT_UI(pub i32);
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = WER_REPORT_UI(1i32);
pub const WerUIIconFilePath: WER_REPORT_UI = WER_REPORT_UI(2i32);
pub const WerUIConsentDlgHeader: WER_REPORT_UI = WER_REPORT_UI(3i32);
pub const WerUIConsentDlgBody: WER_REPORT_UI = WER_REPORT_UI(4i32);
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(5i32);
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(6i32);
pub const WerUICloseText: WER_REPORT_UI = WER_REPORT_UI(7i32);
pub const WerUICloseDlgHeader: WER_REPORT_UI = WER_REPORT_UI(8i32);
pub const WerUICloseDlgBody: WER_REPORT_UI = WER_REPORT_UI(9i32);
pub const WerUICloseDlgButtonText: WER_REPORT_UI = WER_REPORT_UI(10i32);
pub const WerUIMax: WER_REPORT_UI = WER_REPORT_UI(11i32);
impl ::core::marker::Copy for WER_REPORT_UI {}
impl ::core::clone::Clone for WER_REPORT_UI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REPORT_UI {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_REPORT_UI {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_REPORT_UI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_UI").field(&self.0).finish()
    }
}
pub const WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH: &str = "OutOfProcessExceptionEventDebuggerLaunchCallback";
pub const WER_RUNTIME_EXCEPTION_EVENT_FUNCTION: &str = "OutOfProcessExceptionEventCallback";
pub const WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE_FUNCTION: &str = "OutOfProcessExceptionEventSignatureCallback";
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: ::win32_foundation::HANDLE,
    pub hThread: ::win32_foundation::HANDLE,
    pub exceptionRecord: super::Diagnostics::Debug::EXCEPTION_RECORD,
    pub context: super::Diagnostics::Debug::CONTEXT,
    pub pwszReportId: ::windows_core::PCWSTR,
    pub bIsFatal: ::win32_foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for WER_RUNTIME_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
unsafe impl ::windows_core::Abi for WER_RUNTIME_EXCEPTION_INFORMATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WER_RUNTIME_EXCEPTION_INFORMATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for WER_RUNTIME_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_SUBMIT_FLAGS(pub u32);
pub const WER_SUBMIT_ADD_REGISTERED_DATA: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(16u32);
pub const WER_SUBMIT_HONOR_RECOVERY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1u32);
pub const WER_SUBMIT_HONOR_RESTART: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2u32);
pub const WER_SUBMIT_NO_ARCHIVE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(256u32);
pub const WER_SUBMIT_NO_CLOSE_UI: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(64u32);
pub const WER_SUBMIT_NO_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(128u32);
pub const WER_SUBMIT_OUTOFPROCESS: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(32u32);
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1024u32);
pub const WER_SUBMIT_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4u32);
pub const WER_SUBMIT_SHOW_DEBUG: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8u32);
pub const WER_SUBMIT_START_MINIMIZED: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(512u32);
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2048u32);
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4096u32);
pub const WER_SUBMIT_REPORT_MACHINE_ID: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8192u32);
impl ::core::marker::Copy for WER_SUBMIT_FLAGS {}
impl ::core::clone::Clone for WER_SUBMIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_SUBMIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_SUBMIT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_SUBMIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_SUBMIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_SUBMIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WER_SUBMIT_RESULT(pub i32);
pub const WerReportQueued: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(1i32);
pub const WerReportUploaded: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(2i32);
pub const WerReportDebug: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(3i32);
pub const WerReportFailed: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(4i32);
pub const WerDisabled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(5i32);
pub const WerReportCancelled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(6i32);
pub const WerDisabledQueue: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(7i32);
pub const WerReportAsync: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(8i32);
pub const WerCustomAction: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(9i32);
pub const WerThrottled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(10i32);
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(11i32);
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(12i32);
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(13i32);
impl ::core::marker::Copy for WER_SUBMIT_RESULT {}
impl ::core::clone::Clone for WER_SUBMIT_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_SUBMIT_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WER_SUBMIT_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WER_SUBMIT_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_RESULT").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WerAddExcludedApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pwzexename: Param0, ballusers: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerAddExcludedApplication(pwzexename: ::windows_core::PCWSTR, ballusers: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        WerAddExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerFreeString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwszstr: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerFreeString(pwszstr: ::windows_core::PCWSTR);
        }
        WerFreeString(pwszstr.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerGetFlags<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hprocess: Param0) -> ::windows_core::Result<WER_FAULT_REPORTING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerGetFlags(hprocess: ::win32_foundation::HANDLE, pdwflags: *mut WER_FAULT_REPORTING) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_FAULT_REPORTING>::zeroed();
        WerGetFlags(hprocess.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_FAULT_REPORTING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows_core::HRESULT;
        }
        WerRegisterAdditionalProcess(::core::mem::transmute(processid), ::core::mem::transmute(captureextrainfoforthreadid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterAppLocalDump<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(localappdatarelativepath: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterAppLocalDump(localappdatarelativepath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerRegisterAppLocalDump(localappdatarelativepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterCustomMetadata<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(key: Param0, value: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterCustomMetadata(key: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerRegisterCustomMetadata(key.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterExcludedMemoryBlock(address: *const ::core::ffi::c_void, size: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterExcludedMemoryBlock(address: *const ::core::ffi::c_void, size: u32) -> ::windows_core::HRESULT;
        }
        WerRegisterExcludedMemoryBlock(::core::mem::transmute(address), ::core::mem::transmute(size)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzfile: Param0, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterFile(pwzfile: ::windows_core::PCWSTR, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows_core::HRESULT;
        }
        WerRegisterFile(pwzfile.into_param().abi(), ::core::mem::transmute(regfiletype), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterMemoryBlock(pvaddress: *const ::core::ffi::c_void, dwsize: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterMemoryBlock(pvaddress: *const ::core::ffi::c_void, dwsize: u32) -> ::windows_core::HRESULT;
        }
        WerRegisterMemoryBlock(::core::mem::transmute(pvaddress), ::core::mem::transmute(dwsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRegisterRuntimeExceptionModule<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwszoutofprocesscallbackdll: Param0, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: ::windows_core::PCWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerRemoveExcludedApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pwzexename: Param0, ballusers: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerRemoveExcludedApplication(pwzexename: ::windows_core::PCWSTR, ballusers: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        WerRemoveExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn WerReportAddDump<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hreporthandle: Param0, hprocess: Param1, hthread: Param2, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportAddDump(hreporthandle: HREPORT, hprocess: ::win32_foundation::HANDLE, hthread: ::win32_foundation::HANDLE, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows_core::HRESULT;
        }
        WerReportAddDump(hreporthandle.into_param().abi(), hprocess.into_param().abi(), hthread.into_param().abi(), ::core::mem::transmute(dumptype), ::core::mem::transmute(pexceptionparam), ::core::mem::transmute(pdumpcustomoptions), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportAddFile<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreporthandle: Param0, pwzpath: Param1, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportAddFile(hreporthandle: HREPORT, pwzpath: ::windows_core::PCWSTR, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows_core::HRESULT;
        }
        WerReportAddFile(hreporthandle.into_param().abi(), pwzpath.into_param().abi(), ::core::mem::transmute(repfiletype), ::core::mem::transmute(dwfileflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportCloseHandle<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>>(hreporthandle: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportCloseHandle(hreporthandle: HREPORT) -> ::windows_core::HRESULT;
        }
        WerReportCloseHandle(hreporthandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportCreate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzeventtype: Param0, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION) -> ::windows_core::Result<HREPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportCreate(pwzeventtype: ::windows_core::PCWSTR, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION, phreporthandle: *mut HREPORT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HREPORT>::zeroed();
        WerReportCreate(pwzeventtype.into_param().abi(), ::core::mem::transmute(reptype), ::core::mem::transmute(preportinformation), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HREPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportHang<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwndhungapp: Param0, pwzhungapplicationname: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportHang(hwndhungapp: ::win32_foundation::HWND, pwzhungapplicationname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerReportHang(hwndhungapp.into_param().abi(), pwzhungapplicationname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportSetParameter<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreporthandle: Param0, dwparamid: u32, pwzname: Param2, pwzvalue: Param3) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSetParameter(hreporthandle: HREPORT, dwparamid: u32, pwzname: ::windows_core::PCWSTR, pwzvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerReportSetParameter(hreporthandle.into_param().abi(), ::core::mem::transmute(dwparamid), pwzname.into_param().abi(), pwzvalue.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportSetUIOption<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreporthandle: Param0, repuitypeid: WER_REPORT_UI, pwzvalue: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSetUIOption(hreporthandle: HREPORT, repuitypeid: WER_REPORT_UI, pwzvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerReportSetUIOption(hreporthandle.into_param().abi(), ::core::mem::transmute(repuitypeid), pwzvalue.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerReportSubmit<'a, Param0: ::windows_core::IntoParam<'a, HREPORT>>(hreporthandle: Param0, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS) -> ::windows_core::Result<WER_SUBMIT_RESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerReportSubmit(hreporthandle: HREPORT, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_SUBMIT_RESULT>::zeroed();
        WerReportSubmit(hreporthandle.into_param().abi(), ::core::mem::transmute(consent), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_SUBMIT_RESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows_core::HRESULT;
        }
        WerSetFlags(::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreClose<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreClose(hreportstore: HREPORTSTORE);
        }
        WerStoreClose(hreportstore.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreGetFirstReportKey<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetFirstReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        WerStoreGetFirstReportKey(hreportstore.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreGetNextReportKey<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetNextReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        WerStoreGetNextReportKey(hreportstore.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreGetReportCount<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetReportCount(hreportstore: HREPORTSTORE, pdwreportcount: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        WerStoreGetReportCount(hreportstore.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreGetSizeOnDisk<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>>(hreportstore: Param0) -> ::windows_core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreGetSizeOnDisk(hreportstore: HREPORTSTORE, pqwsizeinbytes: *mut u64) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        WerStoreGetSizeOnDisk(hreportstore.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES) -> ::windows_core::Result<HREPORTSTORE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES, phreportstore: *mut HREPORTSTORE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HREPORTSTORE>::zeroed();
        WerStoreOpen(::core::mem::transmute(repstoretype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HREPORTSTORE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStorePurge() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStorePurge() -> ::windows_core::HRESULT;
        }
        WerStorePurge().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV1<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows_core::Result<WER_REPORT_METADATA_V1> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV1(hreportstore: HREPORTSTORE, pszreportkey: ::windows_core::PCWSTR, preportmetadata: *mut WER_REPORT_METADATA_V1) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_REPORT_METADATA_V1>::zeroed();
        WerStoreQueryReportMetadataV1(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_REPORT_METADATA_V1>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV2<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows_core::Result<WER_REPORT_METADATA_V2> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV2(hreportstore: HREPORTSTORE, pszreportkey: ::windows_core::PCWSTR, preportmetadata: *mut WER_REPORT_METADATA_V2) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_REPORT_METADATA_V2>::zeroed();
        WerStoreQueryReportMetadataV2(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_REPORT_METADATA_V2>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV3<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreportstore: Param0, pszreportkey: Param1) -> ::windows_core::Result<WER_REPORT_METADATA_V3> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreQueryReportMetadataV3(hreportstore: HREPORTSTORE, pszreportkey: ::windows_core::PCWSTR, preportmetadata: *mut WER_REPORT_METADATA_V3) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_REPORT_METADATA_V3>::zeroed();
        WerStoreQueryReportMetadataV3(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_REPORT_METADATA_V3>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerStoreUploadReport<'a, Param0: ::windows_core::IntoParam<'a, HREPORTSTORE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hreportstore: Param0, pszreportkey: Param1, dwflags: u32) -> ::windows_core::Result<WER_SUBMIT_RESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerStoreUploadReport(hreportstore: HREPORTSTORE, pszreportkey: ::windows_core::PCWSTR, dwflags: u32, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<WER_SUBMIT_RESULT>::zeroed();
        WerStoreUploadReport(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WER_SUBMIT_RESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows_core::HRESULT;
        }
        WerUnregisterAdditionalProcess(::core::mem::transmute(processid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterAppLocalDump() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterAppLocalDump() -> ::windows_core::HRESULT;
        }
        WerUnregisterAppLocalDump().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterCustomMetadata<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(key: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterCustomMetadata(key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerUnregisterCustomMetadata(key.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterExcludedMemoryBlock(address: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterExcludedMemoryBlock(address: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        WerUnregisterExcludedMemoryBlock(::core::mem::transmute(address)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterFile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzfilepath: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterFile(pwzfilepath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WerUnregisterFile(pwzfilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterMemoryBlock(pvaddress: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterMemoryBlock(pvaddress: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        WerUnregisterMemoryBlock(::core::mem::transmute(pvaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WerUnregisterRuntimeExceptionModule<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwszoutofprocesscallbackdll: Param0, pcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: ::windows_core::PCWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type pfn_ADDEREXCLUDEDAPPLICATIONA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCSTR) -> EFaultRepRetVal>;
pub type pfn_ADDEREXCLUDEDAPPLICATIONW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR) -> EFaultRepRetVal>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type pfn_REPORTFAULT = ::core::option::Option<unsafe extern "system" fn(param0: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, param1: u32) -> EFaultRepRetVal>;
