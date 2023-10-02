#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> super::super::Foundation:: BOOL);
    ProcessIdToSessionId(dwprocessid, psessionid).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCloseServer<P0>(hserver: P0)
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::super::Foundation:: HANDLE) -> ());
    WTSCloseServer(hserver.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionA<P0, P1>(logonid: u32, targetlogonid: u32, ppassword: P0, bwait: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : u32, targetlogonid : u32, ppassword : ::windows_core::PCSTR, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSConnectSessionA(logonid, targetlogonid, ppassword.into_param().abi(), bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSConnectSessionW<P0, P1>(logonid: u32, targetlogonid: u32, ppassword: P0, bwait: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : u32, targetlogonid : u32, ppassword : ::windows_core::PCWSTR, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSConnectSessionW(logonid, targetlogonid, ppassword.into_param().abi(), bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerA<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCSTR, pbuffer : *const WTSLISTENERCONFIGA, flag : u32) -> super::super::Foundation:: BOOL);
    WTSCreateListenerA(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), pbuffer, flag).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSCreateListenerW<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCWSTR, pbuffer : *const WTSLISTENERCONFIGW, flag : u32) -> super::super::Foundation:: BOOL);
    WTSCreateListenerW(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), pbuffer, flag).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSDisconnectSession<P0, P1>(hserver: P0, sessionid: u32, bwait: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSDisconnectSession(hserver.into_param().abi(), sessionid, bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnableChildSessions<P0>(benable: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnableChildSessions(benable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSEnableChildSessions(benable.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersA<P0>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: ::core::option::Option<*mut *mut i8>, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plisteners : *mut *mut i8, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateListenersA(hserver.into_param().abi(), preserved, reserved, ::core::mem::transmute(plisteners.unwrap_or(::std::ptr::null_mut())), pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateListenersW<P0>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: ::core::option::Option<*mut *mut u16>, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plisteners : *mut *mut u16, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateListenersW(hserver.into_param().abi(), preserved, reserved, ::core::mem::transmute(plisteners.unwrap_or(::std::ptr::null_mut())), pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA<P0>(hserver: P0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesA(hserver.into_param().abi(), reserved, version, ppprocessinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExA<P0>(hserver: P0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows_core::PSTR, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut ::windows_core::PSTR, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesExA(hserver.into_param().abi(), plevel, sessionid, ppprocessinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExW<P0>(hserver: P0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut ::windows_core::PWSTR, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut ::windows_core::PWSTR, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesExW(hserver.into_param().abi(), plevel, sessionid, ppprocessinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW<P0>(hserver: P0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesW(hserver.into_param().abi(), reserved, version, ppprocessinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersA<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : ::windows_core::PCSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateServersA(pdomainname.into_param().abi(), reserved, version, ppserverinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateServersW<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : ::windows_core::PCWSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateServersW(pdomainname.into_param().abi(), reserved, version, ppserverinfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsA<P0>(hserver: P0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsA(hserver.into_param().abi(), reserved, version, ppsessioninfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExA<P0>(hserver: P0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1A, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsExA(hserver.into_param().abi(), plevel, filter, ppsessioninfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExW<P0>(hserver: P0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1W, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsExW(hserver.into_param().abi(), plevel, filter, ppsessioninfo, pcount).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSEnumerateSessionsW<P0>(hserver: P0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsW(hserver.into_param().abi(), reserved, version, ppsessioninfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : *mut ::core::ffi::c_void) -> ());
    WTSFreeMemory(pmemory)
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const ::core::ffi::c_void, numberofentries : u32) -> super::super::Foundation:: BOOL);
    WTSFreeMemoryExA(wtstypeclass, pmemory, numberofentries).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const ::core::ffi::c_void, numberofentries : u32) -> super::super::Foundation:: BOOL);
    WTSFreeMemoryExW(wtstypeclass, pmemory, numberofentries).ok()
}
#[inline]
pub unsafe fn WTSGetActiveConsoleSessionId() -> u32 {
    ::windows_targets::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> u32);
    WTSGetActiveConsoleSessionId()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSGetChildSessionId(psessionid : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetChildSessionId(psessionid)
}
#[doc = "Required features: `Win32_Foundation`, `Win32_Security`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetListenerSecurityA(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), securityinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[doc = "Required features: `Win32_Foundation`, `Win32_Security`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetListenerSecurityW(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), securityinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSIsChildSessionsEnabled(pbenabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSIsChildSessionsEnabled(pbenabled)
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSLogoffSession<P0, P1>(hserver: P0, sessionid: u32, bwait: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSLogoffSession(hserver.into_param().abi(), sessionid, bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : ::windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerA(pservername.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : ::windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerExA(pservername.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerExW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : ::windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerExW(pservername.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSOpenServerW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : ::windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerW(pservername.into_param().abi())
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigA<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *mut WTSLISTENERCONFIGA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCSTR, pbuffer : *mut WTSLISTENERCONFIGA) -> super::super::Foundation:: BOOL);
    WTSQueryListenerConfigA(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), pbuffer).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryListenerConfigW<P0, P1>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *mut WTSLISTENERCONFIGW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCWSTR, pbuffer : *mut WTSLISTENERCONFIGW) -> super::super::Foundation:: BOOL);
    WTSQueryListenerConfigW(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), pbuffer).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationA<P0>(hserver: P0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows_core::PSTR, pbytesreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut ::windows_core::PSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQuerySessionInformationA(hserver.into_param().abi(), sessionid, wtsinfoclass, ppbuffer, pbytesreturned).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQuerySessionInformationW<P0>(hserver: P0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut ::windows_core::PWSTR, pbytesreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut ::windows_core::PWSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQuerySessionInformationW(hserver.into_param().abi(), sessionid, wtsinfoclass, ppbuffer, pbytesreturned).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows_core::PSTR, pbytesreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : ::windows_core::PCSTR, pusername : ::windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut ::windows_core::PSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQueryUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), wtsconfigclass, ppbuffer, pbytesreturned).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserConfigW<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut ::windows_core::PWSTR, pbytesreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : ::windows_core::PCWSTR, pusername : ::windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut ::windows_core::PWSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQueryUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), wtsconfigclass, ppbuffer, pbytesreturned).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : u32, phtoken : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSQueryUserToken(sessionid, phtoken).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotification<P0>(hwnd: P0, dwflags: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::super::Foundation:: HWND, dwflags : u32) -> super::super::Foundation:: BOOL);
    WTSRegisterSessionNotification(hwnd.into_param().abi(), dwflags).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx<P0, P1>(hserver: P0, hwnd: P1, dwflags: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND, dwflags : u32) -> super::super::Foundation:: BOOL);
    WTSRegisterSessionNotificationEx(hserver.into_param().abi(), hwnd.into_param().abi(), dwflags).ok()
}
#[doc = "Required features: `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageA<P0, P1>(hserver: P0, sessionid: u32, ptitle: &[u8], pmessage: &[u8], style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : ::windows_core::PCSTR, titlelength : u32, pmessage : ::windows_core::PCSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSSendMessageA(hserver.into_param().abi(), sessionid, ::core::mem::transmute(ptitle.as_ptr()), ptitle.len() as _, ::core::mem::transmute(pmessage.as_ptr()), pmessage.len() as _, style, timeout, presponse, bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn WTSSendMessageW<P0, P1, P2, P3>(hserver: P0, sessionid: u32, ptitle: P1, titlelength: u32, pmessage: P2, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : ::windows_core::PCWSTR, titlelength : u32, pmessage : ::windows_core::PCWSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSSendMessageW(hserver.into_param().abi(), sessionid, ptitle.into_param().abi(), titlelength, pmessage.into_param().abi(), messagelength, style, timeout, presponse, bwait.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`, `Win32_Security`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<P0, P1, P2>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: BOOL);
    WTSSetListenerSecurityA(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`, `Win32_Security`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<P0, P1, P2>(hserver: P0, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const ::core::ffi::c_void, reserved : u32, plistenername : ::windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: BOOL);
    WTSSetListenerSecurityW(hserver.into_param().abi(), preserved, reserved, plistenername.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetRenderHint<P0>(prenderhintid: *mut u64, hwndowner: P0, renderhinttype: u32, phintdata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSetRenderHint(prenderhintid : *mut u64, hwndowner : super::super::Foundation:: HWND, renderhinttype : u32, cbhintdatalength : u32, phintdata : *const u8) -> ::windows_core::HRESULT);
    WTSSetRenderHint(prenderhintid, hwndowner.into_param().abi(), renderhinttype, phintdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(phintdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: &[u8]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : ::windows_core::PCSTR, pusername : ::windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : ::windows_core::PCSTR, datalength : u32) -> super::super::Foundation:: BOOL);
    WTSSetUserConfigA(pservername.into_param().abi(), pusername.into_param().abi(), wtsconfigclass, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSSetUserConfigW<P0, P1, P2>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: P2, datalength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : ::windows_core::PCWSTR, pusername : ::windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : ::windows_core::PCWSTR, datalength : u32) -> super::super::Foundation:: BOOL);
    WTSSetUserConfigW(pservername.into_param().abi(), pusername.into_param().abi(), wtsconfigclass, pbuffer.into_param().abi(), datalength).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSShutdownSystem<P0>(hserver: P0, shutdownflag: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::super::Foundation:: HANDLE, shutdownflag : u32) -> super::super::Foundation:: BOOL);
    WTSShutdownSystem(hserver.into_param().abi(), shutdownflag).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : ::windows_core::PCSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> super::super::Foundation:: BOOL);
    WTSStartRemoteControlSessionA(ptargetservername.into_param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : ::windows_core::PCWSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> super::super::Foundation:: BOOL);
    WTSStartRemoteControlSessionW(ptargetservername.into_param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSStopRemoteControlSession(logonid: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : u32) -> super::super::Foundation:: BOOL);
    WTSStopRemoteControlSession(logonid).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSTerminateProcess<P0>(hserver: P0, processid: u32, exitcode: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::super::Foundation:: HANDLE, processid : u32, exitcode : u32) -> super::super::Foundation:: BOOL);
    WTSTerminateProcess(hserver.into_param().abi(), processid, exitcode).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification<P0>(hwnd: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    WTSUnRegisterSessionNotification(hwnd.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx<P0, P1>(hserver: P0, hwnd: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    WTSUnRegisterSessionNotificationEx(hserver.into_param().abi(), hwnd.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelClose<P0>(hchannelhandle: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelClose(hchannelhandle.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelOpen<P0, P1>(hserver: P0, sessionid: u32, pvirtualname: P1) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::super::Foundation:: HANDLE, sessionid : u32, pvirtualname : ::windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = WTSVirtualChannelOpen(hserver.into_param().abi(), sessionid, pvirtualname.into_param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<P0>(sessionid: u32, pvirtualname: P0, flags: u32) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : u32, pvirtualname : ::windows_core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
    let result__ = WTSVirtualChannelOpenEx(sessionid, pvirtualname.into_param().abi(), flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput<P0>(hchannelhandle: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelPurgeInput(hchannelhandle.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput<P0>(hchannelhandle: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelPurgeOutput(hchannelhandle.into_param().abi()).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelQuery<P0>(hchannelhandle: P0, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::super::Foundation:: HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut *mut ::core::ffi::c_void, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelQuery(hchannelhandle.into_param().abi(), param1, ppbuffer, pbytesreturned).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelRead<P0>(hchannelhandle: P0, timeout: u32, buffer: &mut [u8], pbytesread: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::super::Foundation:: HANDLE, timeout : u32, buffer : ::windows_core::PSTR, buffersize : u32, pbytesread : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelRead(hchannelhandle.into_param().abi(), timeout, ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _, pbytesread).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSVirtualChannelWrite<P0>(hchannelhandle: P0, buffer: &[u8], pbyteswritten: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::super::Foundation:: HANDLE, buffer : ::windows_core::PCSTR, length : u32, pbyteswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelWrite(hchannelhandle.into_param().abi(), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _, pbyteswritten).ok()
}
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WTSWaitSystemEvent<P0>(hserver: P0, eventmask: u32, peventflags: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::super::Foundation:: HANDLE, eventmask : u32, peventflags : *mut u32) -> super::super::Foundation:: BOOL);
    WTSWaitSystemEvent(hserver.into_param().abi(), eventmask, peventflags).ok()
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsTSUserEx(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTSUserEx {
    pub unsafe fn TerminalServicesProfilePath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TerminalServicesProfilePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTerminalServicesProfilePath<P0>(&self, pnewval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTerminalServicesProfilePath)(::windows_core::Interface::as_raw(self), pnewval.into_param().abi()).ok()
    }
    pub unsafe fn TerminalServicesHomeDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TerminalServicesHomeDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTerminalServicesHomeDirectory<P0>(&self, pnewval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTerminalServicesHomeDirectory)(::windows_core::Interface::as_raw(self), pnewval.into_param().abi()).ok()
    }
    pub unsafe fn TerminalServicesHomeDrive(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TerminalServicesHomeDrive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTerminalServicesHomeDrive<P0>(&self, pnewval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTerminalServicesHomeDrive)(::windows_core::Interface::as_raw(self), pnewval.into_param().abi()).ok()
    }
    pub unsafe fn AllowLogon(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowLogon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllowLogon(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowLogon)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn EnableRemoteControl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnableRemoteControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnableRemoteControl(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableRemoteControl)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxDisconnectionTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxDisconnectionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxDisconnectionTime(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxDisconnectionTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxConnectionTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxConnectionTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxConnectionTime(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxConnectionTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxIdleTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxIdleTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxIdleTime(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxIdleTime)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ReconnectionAction(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReconnectionAction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReconnectionAction(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReconnectionAction)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn BrokenConnectionAction(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BrokenConnectionAction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBrokenConnectionAction(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBrokenConnectionAction)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ConnectClientDrivesAtLogon(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConnectClientDrivesAtLogon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectClientDrivesAtLogon)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ConnectClientPrintersAtLogon(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConnectClientPrintersAtLogon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectClientPrintersAtLogon)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn DefaultToMainPrinter(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DefaultToMainPrinter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultToMainPrinter(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultToMainPrinter)(::windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn TerminalServicesWorkDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TerminalServicesWorkDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTerminalServicesWorkDirectory<P0>(&self, pnewval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTerminalServicesWorkDirectory)(::windows_core::Interface::as_raw(self), pnewval.into_param().abi()).ok()
    }
    pub unsafe fn TerminalServicesInitialProgram(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TerminalServicesInitialProgram)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTerminalServicesInitialProgram<P0>(&self, pnewval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTerminalServicesInitialProgram)(::windows_core::Interface::as_raw(self), pnewval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsTSUserEx, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsTSUserEx {
    type Vtable = IADsTSUserEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsTSUserEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4930e79_2989_4462_8a60_2fcf2f2955ef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTSUserEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TerminalServicesProfilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTerminalServicesProfilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTerminalServicesHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTerminalServicesHomeDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AllowLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAllowLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub EnableRemoteControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetEnableRemoteControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub MaxDisconnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxDisconnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub MaxConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub MaxIdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxIdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub ReconnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT,
    pub SetReconnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub BrokenConnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT,
    pub SetBrokenConnectionAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub ConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT,
    pub SetConnectClientDrivesAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub ConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetConnectClientPrintersAtLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub DefaultToMainPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetDefaultToMainPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub TerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTerminalServicesWorkDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTerminalServicesInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioDeviceEndpoint(::windows_core::IUnknown);
impl IAudioDeviceEndpoint {
    pub unsafe fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBuffer)(::windows_core::Interface::as_raw(self), maxperiod, u32latencycoefficient).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRTCaps(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRTCaps)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEventDrivenCapable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEventDrivenCapable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteExclusiveModeParametersToSharedMemory)(::windows_core::Interface::as_raw(self), htargetprocess, hnsperiod, hnsbufferduration, u32latencycoefficient, pu32sharedmemorysize, phsharedmemory).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAudioDeviceEndpoint, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceEndpoint {
    type Vtable = IAudioDeviceEndpoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceEndpoint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4952f5a_a0b2_4cc4_8b82_9358488dd8ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceEndpoint_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRTCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRTCaps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventDrivenCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventDrivenCapable: usize,
    pub WriteExclusiveModeParametersToSharedMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioEndpoint(::windows_core::IUnknown);
impl IAudioEndpoint {
    #[doc = "Required features: `Win32_Media_Audio`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFrameFormat(&self) -> ::windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFrameFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFramesPerPacket(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFramesPerPacket)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLatency(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStreamFlags(&self, streamflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamFlags)(::windows_core::Interface::as_raw(self), streamflags).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetEventHandle)(::windows_core::Interface::as_raw(self), eventhandle.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAudioEndpoint, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEndpoint {
    type Vtable = IAudioEndpoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioEndpoint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30a99515_1527_4451_af9f_00c5f0234daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpoint_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFrameFormat: usize,
    pub GetFramesPerPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows_core::HRESULT,
    pub SetStreamFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioEndpointControl(::windows_core::IUnknown);
impl IAudioEndpointControl {
    pub unsafe fn Start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAudioEndpointControl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEndpointControl {
    type Vtable = IAudioEndpointControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioEndpointControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc684b72a_6df4_4774_bdf9_76b77509b653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioEndpointRT(::windows_core::IUnknown);
impl IAudioEndpointRT {
    pub unsafe fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
        (::windows_core::Interface::vtable(self).GetCurrentPadding)(::windows_core::Interface::as_raw(self), ppadding, paecurrentposition)
    }
    pub unsafe fn ProcessingComplete(&self) {
        (::windows_core::Interface::vtable(self).ProcessingComplete)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPinInactive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPinInactive)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPinActive(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPinActive)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAudioEndpointRT, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioEndpointRT {
    type Vtable = IAudioEndpointRT_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioEndpointRT {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfd2005f_a6e5_4d39_a265_939ada9fbb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointRT_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION),
    pub ProcessingComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SetPinInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPinActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioInputEndpointRT(::windows_core::IUnknown);
impl IAudioInputEndpointRT {
    #[doc = "Required features: `Win32_Media_Audio_Apo`"]
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
        (::windows_core::Interface::vtable(self).GetInputDataPointer)(::windows_core::Interface::as_raw(self), pconnectionproperty, paetimestamp)
    }
    pub unsafe fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize) {
        (::windows_core::Interface::vtable(self).ReleaseInputDataPointer)(::windows_core::Interface::as_raw(self), u32framecount, pdatapointer)
    }
    pub unsafe fn PulseEndpoint(&self) {
        (::windows_core::Interface::vtable(self).PulseEndpoint)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IAudioInputEndpointRT, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioInputEndpointRT {
    type Vtable = IAudioInputEndpointRT_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioInputEndpointRT {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8026ab61_92b2_43c1_a1df_5c37ebd08d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputEndpointRT_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub GetInputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    GetInputDataPointer: usize,
    pub ReleaseInputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize),
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAudioOutputEndpointRT(::windows_core::IUnknown);
impl IAudioOutputEndpointRT {
    pub unsafe fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
        (::windows_core::Interface::vtable(self).GetOutputDataPointer)(::windows_core::Interface::as_raw(self), u32framecount, paetimestamp)
    }
    #[doc = "Required features: `Win32_Media_Audio_Apo`"]
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
        (::windows_core::Interface::vtable(self).ReleaseOutputDataPointer)(::windows_core::Interface::as_raw(self), pconnectionproperty)
    }
    pub unsafe fn PulseEndpoint(&self) {
        (::windows_core::Interface::vtable(self).PulseEndpoint)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IAudioOutputEndpointRT, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioOutputEndpointRT {
    type Vtable = IAudioOutputEndpointRT_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAudioOutputEndpointRT {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fa906e4_c31c_4e31_932e_19a66385e9aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputEndpointRT_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOutputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointer: usize,
    pub PulseEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopClient(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClient {
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reconnect(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reconnect)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> ::windows_core::Result<IRemoteDesktopClientSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Settings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Actions(&self) -> ::windows_core::Result<IRemoteDesktopClientActions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Actions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TouchPointer(&self) -> ::windows_core::Result<IRemoteDesktopClientTouchPointer> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TouchPointer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteSavedCredentials<P0>(&self, servername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteSavedCredentials)(::windows_core::Interface::as_raw(self), servername.into_param().abi()).ok()
    }
    pub unsafe fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateSessionDisplaySettings)(::windows_core::Interface::as_raw(self), width, height).ok()
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attachEvent<P0, P1>(&self, eventname: P0, callback: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).attachEvent)(::windows_core::Interface::as_raw(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn detachEvent<P0, P1>(&self, eventname: P0, callback: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).detachEvent)(::windows_core::Interface::as_raw(self), eventname.into_param().abi(), callback.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRemoteDesktopClient, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRemoteDesktopClient {
    type Vtable = IRemoteDesktopClient_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRemoteDesktopClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d25668_625a_4905_be4e_304caa13f89c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TouchPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, touchpointer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TouchPointer: usize,
    pub DeleteSavedCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UpdateSessionDisplaySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub attachEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attachEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub detachEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    detachEvent: usize,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopClientActions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientActions {
    pub unsafe fn SuspendScreenUpdates(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendScreenUpdates)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeScreenUpdates(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeScreenUpdates)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExecuteRemoteAction)(::windows_core::Interface::as_raw(self), remoteaction).ok()
    }
    pub unsafe fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSnapshot)(::windows_core::Interface::as_raw(self), snapshotencoding, snapshotformat, snapshotwidth, snapshotheight, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRemoteDesktopClientActions, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRemoteDesktopClientActions {
    type Vtable = IRemoteDesktopClientActions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRemoteDesktopClientActions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d54bc4e_1028_45d4_8b0a_b9b6bffba176);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientActions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SuspendScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExecuteRemoteAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows_core::HRESULT,
    pub GetSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopClientSettings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientSettings {
    pub unsafe fn ApplySettings<P0>(&self, rdpfilecontents: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ApplySettings)(::windows_core::Interface::as_raw(self), rdpfilecontents.into_param().abi()).ok()
    }
    pub unsafe fn RetrieveSettings(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RetrieveSettings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRdpProperty<P0>(&self, propertyname: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRdpProperty)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetRdpProperty<P0>(&self, propertyname: P0, value: super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRdpProperty)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRemoteDesktopClientSettings, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRemoteDesktopClientSettings {
    type Vtable = IRemoteDesktopClientSettings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRemoteDesktopClientSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48a0f2a7_2713_431f_bbac_6f4558e7d64d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdpfilecontents: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RetrieveSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetRdpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetRdpProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetRdpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetRdpProperty: usize,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopClientTouchPointer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientTouchPointer {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventsEnabled<P0>(&self, eventsenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEventsEnabled)(::windows_core::Interface::as_raw(self), eventsenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventsEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventsEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPointerSpeed(&self, pointerspeed: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPointerSpeed)(::windows_core::Interface::as_raw(self), pointerspeed).ok()
    }
    pub unsafe fn PointerSpeed(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PointerSpeed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IRemoteDesktopClientTouchPointer, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IRemoteDesktopClientTouchPointer {
    type Vtable = IRemoteDesktopClientTouchPointer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IRemoteDesktopClientTouchPointer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x260ec22d_8cbc_44b5_9e88_2a37f6c93ae9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientTouchPointer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventsEnabled: usize,
    pub SetPointerSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows_core::HRESULT,
    pub PointerSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteSystemAdditionalInfoProvider(::windows_core::IUnknown);
impl IRemoteSystemAdditionalInfoProvider {
    pub unsafe fn GetAdditionalInfo<T>(&self, deduplicationid: *mut ::windows_core::HSTRING) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetAdditionalInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(deduplicationid), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IRemoteSystemAdditionalInfoProvider, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteSystemAdditionalInfoProvider {
    type Vtable = IRemoteSystemAdditionalInfoProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteSystemAdditionalInfoProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeaa3d5f_ec63_4d27_af38_e86b1d7292cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAdditionalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGAccountingEngine(::windows_core::IUnknown);
impl ITSGAccountingEngine {
    pub unsafe fn DoAccounting(&self, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoAccounting)(::windows_core::Interface::as_raw(self), accountingdatatype, ::core::mem::transmute(accountingdata)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITSGAccountingEngine, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGAccountingEngine {
    type Vtable = ITSGAccountingEngine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGAccountingEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ce2a0c9_e874_4f1a_86f4_06bbb9115338);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAccountingEngine_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DoAccounting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGAuthenticateUserSink(::windows_core::IUnknown);
impl ITSGAuthenticateUserSink {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUserAuthenticated<P0, P1, P2>(&self, username: P0, userdomain: P1, context: usize, usertoken: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
    {
        (::windows_core::Interface::vtable(self).OnUserAuthenticated)(::windows_core::Interface::as_raw(self), username.into_param().abi(), userdomain.into_param().abi(), context, usertoken.into_param().abi()).ok()
    }
    pub unsafe fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: ::windows_core::HRESULT, specificerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnUserAuthenticationFailed)(::windows_core::Interface::as_raw(self), context, genericerrorcode, specificerrorcode).ok()
    }
    pub unsafe fn ReauthenticateUser(&self, context: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReauthenticateUser)(::windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn DisconnectUser(&self, context: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectUser)(::windows_core::Interface::as_raw(self), context).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITSGAuthenticateUserSink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGAuthenticateUserSink {
    type Vtable = ITSGAuthenticateUserSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGAuthenticateUserSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c3e2e73_a782_47f9_8dfb_77ee1ed27a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticateUserSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUserAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUserAuthenticated: usize,
    pub OnUserAuthenticationFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows_core::HRESULT, specificerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ReauthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize) -> ::windows_core::HRESULT,
    pub DisconnectUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGAuthenticationEngine(::windows_core::IUnknown);
impl ITSGAuthenticationEngine {
    pub unsafe fn AuthenticateUser<P0>(&self, mainsessionid: ::windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITSGAuthenticateUserSink>,
    {
        (::windows_core::Interface::vtable(self).AuthenticateUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mainsessionid), cookiedata, numcookiebytes, context, psink.into_param().abi()).ok()
    }
    pub unsafe fn CancelAuthentication(&self, mainsessionid: ::windows_core::GUID, context: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelAuthentication)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mainsessionid), context).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITSGAuthenticationEngine, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGAuthenticationEngine {
    type Vtable = ITSGAuthenticationEngine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGAuthenticationEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ee3e5bf_04ab_4691_998c_d7f622321a56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticationEngine_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, context: usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGAuthorizeConnectionSink(::windows_core::IUnknown);
impl ITSGAuthorizeConnectionSink {
    pub unsafe fn OnConnectionAuthorized(&self, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, pbsohresponse: &[u8], idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnConnectionAuthorized)(::windows_core::Interface::as_raw(self), hrin, ::core::mem::transmute(mainsessionid), pbsohresponse.len() as _, ::core::mem::transmute(pbsohresponse.as_ptr()), idletimeout, sessiontimeout, sessiontimeoutaction, trustclass, policyattributes).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITSGAuthorizeConnectionSink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGAuthorizeConnectionSink {
    type Vtable = ITSGAuthorizeConnectionSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGAuthorizeConnectionSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc27ece33_7781_4318_98ef_1cf2da7b7005);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeConnectionSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnConnectionAuthorized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGAuthorizeResourceSink(::windows_core::IUnknown);
impl ITSGAuthorizeResourceSink {
    pub unsafe fn OnChannelAuthorized(&self, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, subsessionid: i32, allowedresourcenames: &[::windows_core::BSTR], failedresourcenames: &[::windows_core::BSTR]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnChannelAuthorized)(::windows_core::Interface::as_raw(self), hrin, ::core::mem::transmute(mainsessionid), subsessionid, ::core::mem::transmute(allowedresourcenames.as_ptr()), allowedresourcenames.len() as _, ::core::mem::transmute(failedresourcenames.as_ptr()), failedresourcenames.len() as _).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITSGAuthorizeResourceSink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGAuthorizeResourceSink {
    type Vtable = ITSGAuthorizeResourceSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGAuthorizeResourceSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeddfcd4_fa12_4435_ae55_7ad1a9779af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeResourceSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnChannelAuthorized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, subsessionid: i32, allowedresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numallowedresourcenames: u32, failedresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numfailedresourcenames: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITSGPolicyEngine(::windows_core::IUnknown);
impl ITSGPolicyEngine {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthorizeConnection<P0, P1, P2, P3, P4>(&self, mainsessionid: ::windows_core::GUID, username: P0, authtype: AAAuthSchemes, clientmachineip: P1, clientmachinename: P2, sohdata: &[u8], cookiedata: &[u8], usertoken: P3, psink: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P4: ::windows_core::IntoParam<ITSGAuthorizeConnectionSink>,
    {
        (::windows_core::Interface::vtable(self).AuthorizeConnection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mainsessionid), username.into_param().abi(), authtype, clientmachineip.into_param().abi(), clientmachinename.into_param().abi(), ::core::mem::transmute(sohdata.as_ptr()), sohdata.len() as _, ::core::mem::transmute(cookiedata.as_ptr()), cookiedata.len() as _, usertoken.into_param().abi(), psink.into_param().abi()).ok()
    }
    pub unsafe fn AuthorizeResource<P0, P1, P2>(&self, mainsessionid: ::windows_core::GUID, subsessionid: i32, username: P0, resourcenames: &[::windows_core::BSTR], alternateresourcenames: &[::windows_core::BSTR], portnumber: u32, operation: P1, cookie: &[u8], psink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<ITSGAuthorizeResourceSink>,
    {
        (::windows_core::Interface::vtable(self).AuthorizeResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mainsessionid), subsessionid, username.into_param().abi(), ::core::mem::transmute(resourcenames.as_ptr()), resourcenames.len() as _, ::core::mem::transmute(alternateresourcenames.as_ptr()), alternateresourcenames.len() as _, portnumber, operation.into_param().abi(), ::core::mem::transmute(cookie.as_ptr()), cookie.len() as _, psink.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsQuarantineEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsQuarantineEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITSGPolicyEngine, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITSGPolicyEngine {
    type Vtable = ITSGPolicyEngine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITSGPolicyEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bc24f08_6223_42f4_a5b4_8e37cd135bbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGPolicyEngine_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthorizeConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::std::mem::MaybeUninit<::windows_core::BSTR>, clientmachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthorizeConnection: usize,
    pub AuthorizeResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, subsessionid: i32, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numresources: u32, alternateresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numalternateresourcename: u32, portnumber: u32, operation: ::std::mem::MaybeUninit<::windows_core::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsQuarantineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsQuarantineEnabled: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbBaseNotifySink(::windows_core::IUnknown);
impl ITsSbBaseNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbBaseNotifySink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbBaseNotifySink {
    type Vtable = ITsSbBaseNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbBaseNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x808a6537_1282_4989_9e09_f43938b71722);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbBaseNotifySink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbClientConnection(::windows_core::IUnknown);
impl ITsSbClientConnection {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitialProgram(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitialProgram)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadBalanceResult(&self) -> ::windows_core::Result<ITsSbLoadBalanceResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadBalanceResult)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FarmName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FarmName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutContext<P0>(&self, contextid: P0, context: super::Variant::VARIANT, existingcontext: ::core::option::Option<*mut super::Variant::VARIANT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutContext)(::windows_core::Interface::as_raw(self), contextid.into_param().abi(), ::core::mem::transmute(context), ::core::mem::transmute(existingcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetContext<P0>(&self, contextid: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), contextid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Environment(&self) -> ::windows_core::Result<ITsSbEnvironment> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Environment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_ConnectionError(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_ConnectionError)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SamUserAccount(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SamUserAccount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn ClientConnectionPropertySet(&self) -> ::windows_core::Result<ITsSbClientConnectionPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientConnectionPropertySet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFirstAssignment(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsFirstAssignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RdFarmType(&self) -> ::windows_core::Result<RD_FARM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RdFarmType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSidString(&self) -> ::windows_core::Result<*mut i8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserSidString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisconnectedSession(&self) -> ::windows_core::Result<ITsSbSession> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisconnectedSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbClientConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbClientConnection {
    type Vtable = ITsSbClientConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbClientConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18857499_ad61_4b1b_b7df_cbcd41fb8338);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoadBalanceResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextid: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: super::Variant::VARIANT, existingcontext: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextid: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetContext: usize,
    pub Environment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_ConnectionError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SamUserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ClientConnectionPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ClientConnectionPropertySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFirstAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFirstAssignment: usize,
    pub RdFarmType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows_core::HRESULT,
    pub UserSidString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows_core::HRESULT,
    pub GetDisconnectedSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbClientConnectionPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnectionPropertySet {
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Variant::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Com::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
::windows_core::imp::interface_hierarchy!(ITsSbClientConnectionPropertySet, ::windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::Interface for ITsSbClientConnectionPropertySet {
    type Vtable = ITsSbClientConnectionPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::ComInterface for ITsSbClientConnectionPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe51995b0_46d6_11dd_aa21_cedc55d89593);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnectionPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbEnvironment(::windows_core::IUnknown);
impl ITsSbEnvironment {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ServerWeight(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerWeight)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnvironmentPropertySet(&self) -> ::windows_core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnvironmentPropertySet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetEnvironmentPropertySet<P0>(&self, pval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbEnvironmentPropertySet>,
    {
        (::windows_core::Interface::vtable(self).SetEnvironmentPropertySet)(::windows_core::Interface::as_raw(self), pval.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbEnvironment, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbEnvironment {
    type Vtable = ITsSbEnvironment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbEnvironment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c87f7f7_bf51_4a5c_87bf_8e94fb6e2256);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironment_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServerWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnvironmentPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnvironmentPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetEnvironmentPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetEnvironmentPropertySet: usize,
}
#[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbEnvironmentPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironmentPropertySet {
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Variant::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Com::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
::windows_core::imp::interface_hierarchy!(ITsSbEnvironmentPropertySet, ::windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::Interface for ITsSbEnvironmentPropertySet {
    type Vtable = ITsSbEnvironmentPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::ComInterface for ITsSbEnvironmentPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0d1bf7e_7acf_11dd_a243_e51156d89593);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironmentPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbFilterPluginStore(::windows_core::IUnknown);
impl ITsSbFilterPluginStore {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveProperties<P0>(&self, ppropertyset: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbPropertySet>,
    {
        (::windows_core::Interface::vtable(self).SaveProperties)(::windows_core::Interface::as_raw(self), ppropertyset.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnumerateProperties(&self) -> ::windows_core::Result<ITsSbPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumerateProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteProperties<P0>(&self, propertyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteProperties)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbFilterPluginStore, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbFilterPluginStore {
    type Vtable = ITsSbFilterPluginStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbFilterPluginStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85b44b0f_ed78_413f_9702_fa6d3b5ee755);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbFilterPluginStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnumerateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnumerateProperties: usize,
    pub DeleteProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbGenericNotifySink(::windows_core::IUnknown);
impl ITsSbGenericNotifySink {
    pub unsafe fn OnCompleted(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCompleted)(::windows_core::Interface::as_raw(self), status).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWaitTimeout(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWaitTimeout)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbGenericNotifySink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbGenericNotifySink {
    type Vtable = ITsSbGenericNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbGenericNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c4c8c4f_300b_46ad_9164_8468a7e7568c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGenericNotifySink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWaitTimeout: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbGlobalStore(::windows_core::IUnknown);
impl ITsSbGlobalStore {
    pub unsafe fn QueryTarget<P0, P1, P2>(&self, providername: P0, targetname: P1, farmname: P2) -> ::windows_core::Result<ITsSbTarget>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryTarget)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), targetname.into_param().abi(), farmname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn QuerySessionBySessionId<P0, P1>(&self, providername: P0, dwsessionid: u32, targetname: P1) -> ::windows_core::Result<ITsSbSession>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QuerySessionBySessionId)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), dwsessionid, targetname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms<P0>(&self, providername: P0, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateFarms)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateTargets<P0, P1, P2>(&self, providername: P0, farmname: P1, envname: P2, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateTargets)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), farmname.into_param().abi(), envname.into_param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateEnvironmentsByProvider<P0>(&self, providername: P0, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateEnvironmentsByProvider)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), pdwcount, ppval).ok()
    }
    pub unsafe fn EnumerateSessions<P0, P1, P2, P3, P4, P5>(&self, providername: P0, targetname: P1, username: P2, userdomain: P3, poolname: P4, initialprogram: P5, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
        P5: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateSessions)(::windows_core::Interface::as_raw(self), providername.into_param().abi(), targetname.into_param().abi(), username.into_param().abi(), userdomain.into_param().abi(), poolname.into_param().abi(), initialprogram.into_param().abi(), psessionstate, pdwcount, ppval).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFarmProperty<P0, P1>(&self, farmname: P0, propertyname: P1, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetFarmProperty)(::windows_core::Interface::as_raw(self), farmname.into_param().abi(), propertyname.into_param().abi(), pvarvalue).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbGlobalStore, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbGlobalStore {
    type Vtable = ITsSbGlobalStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbGlobalStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ab60f7b_bd72_4d9f_8a3a_a0ea5574e635);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGlobalStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, envname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::HRESULT,
    pub EnumerateEnvironmentsByProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFarmProperty: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbLoadBalanceResult(::windows_core::IUnknown);
impl ITsSbLoadBalanceResult {
    pub unsafe fn TargetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbLoadBalanceResult, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbLoadBalanceResult {
    type Vtable = ITsSbLoadBalanceResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbLoadBalanceResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24fdb7ac_fea6_11dc_9672_9a8956d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalanceResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbLoadBalancing(::windows_core::IUnknown);
impl ITsSbLoadBalancing {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn GetMostSuitableTarget<P0, P1>(&self, pconnection: P0, plbsink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbClientConnection>,
        P1: ::windows_core::IntoParam<ITsSbLoadBalancingNotifySink>,
    {
        (::windows_core::Interface::vtable(self).GetMostSuitableTarget)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi(), plbsink.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbLoadBalancing, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbLoadBalancing {
    type Vtable = ITsSbLoadBalancing_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbLoadBalancing {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24329274_9eb7_11dc_ae98_f2b456d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancing_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub GetMostSuitableTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, plbsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbLoadBalancingNotifySink(::windows_core::IUnknown);
impl ITsSbLoadBalancingNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnGetMostSuitableTarget<P0, P1>(&self, plbresult: P0, fisnewconnection: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbLoadBalanceResult>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnGetMostSuitableTarget)(::windows_core::Interface::as_raw(self), plbresult.into_param().abi(), fisnewconnection.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbLoadBalancingNotifySink, ::windows_core::IUnknown, ITsSbBaseNotifySink);
unsafe impl ::windows_core::Interface for ITsSbLoadBalancingNotifySink {
    type Vtable = ITsSbLoadBalancingNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbLoadBalancingNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f8a8297_3244_4e6a_958a_27c822c1e141);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancingNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnGetMostSuitableTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbresult: *mut ::core::ffi::c_void, fisnewconnection: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnGetMostSuitableTarget: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbOrchestration(::windows_core::IUnknown);
impl ITsSbOrchestration {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn PrepareTargetForConnect<P0, P1>(&self, pconnection: P0, porchestrationnotifysink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbClientConnection>,
        P1: ::windows_core::IntoParam<ITsSbOrchestrationNotifySink>,
    {
        (::windows_core::Interface::vtable(self).PrepareTargetForConnect)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi(), porchestrationnotifysink.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbOrchestration, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbOrchestration {
    type Vtable = ITsSbOrchestration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbOrchestration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64fc1172_9eb7_11dc_8b00_3aba56d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestration_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub PrepareTargetForConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, porchestrationnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbOrchestrationNotifySink(::windows_core::IUnknown);
impl ITsSbOrchestrationNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
    pub unsafe fn OnReadyToConnect<P0>(&self, ptarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTarget>,
    {
        (::windows_core::Interface::vtable(self).OnReadyToConnect)(::windows_core::Interface::as_raw(self), ptarget.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbOrchestrationNotifySink, ::windows_core::IUnknown, ITsSbBaseNotifySink);
unsafe impl ::windows_core::Interface for ITsSbOrchestrationNotifySink {
    type Vtable = ITsSbOrchestrationNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbOrchestrationNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36c37d61_926b_442f_bca5_118c6d50dcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestrationNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnReadyToConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPlacement(::windows_core::IUnknown);
impl ITsSbPlacement {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn QueryEnvironmentForTarget<P0, P1>(&self, pconnection: P0, pplacementsink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbClientConnection>,
        P1: ::windows_core::IntoParam<ITsSbPlacementNotifySink>,
    {
        (::windows_core::Interface::vtable(self).QueryEnvironmentForTarget)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi(), pplacementsink.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbPlacement, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbPlacement {
    type Vtable = ITsSbPlacement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbPlacement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaadee5f_6d32_480e_9e36_ddab2329f06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacement_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub QueryEnvironmentForTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pplacementsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPlacementNotifySink(::windows_core::IUnknown);
impl ITsSbPlacementNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
    pub unsafe fn OnQueryEnvironmentCompleted<P0>(&self, penvironment: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbEnvironment>,
    {
        (::windows_core::Interface::vtable(self).OnQueryEnvironmentCompleted)(::windows_core::Interface::as_raw(self), penvironment.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbPlacementNotifySink, ::windows_core::IUnknown, ITsSbBaseNotifySink);
unsafe impl ::windows_core::Interface for ITsSbPlacementNotifySink {
    type Vtable = ITsSbPlacementNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbPlacementNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68a0c487_2b4f_46c2_94a1_6ce685183634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacementNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnQueryEnvironmentCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPlugin(::windows_core::IUnknown);
impl ITsSbPlugin {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbPlugin, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbPlugin {
    type Vtable = ITsSbPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48cd7406_caab_465f_a5d6_baa863b9ea4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlugin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPluginNotifySink(::windows_core::IUnknown);
impl ITsSbPluginNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
    pub unsafe fn OnInitialized(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnInitialized)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn OnTerminated(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTerminated)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbPluginNotifySink, ::windows_core::IUnknown, ITsSbBaseNotifySink);
unsafe impl ::windows_core::Interface for ITsSbPluginNotifySink {
    type Vtable = ITsSbPluginNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbPluginNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44dfe30b_c3be_40f5_bf82_7a95bb795adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnTerminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPluginPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPluginPropertySet {
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Variant::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Com::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
::windows_core::imp::interface_hierarchy!(ITsSbPluginPropertySet, ::windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::Interface for ITsSbPluginPropertySet {
    type Vtable = ITsSbPluginPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::ComInterface for ITsSbPluginPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95006e34_7eff_4b6c_bb40_49a4fda7cea6);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPropertySet {
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Variant::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Com::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
::windows_core::imp::interface_hierarchy!(ITsSbPropertySet, ::windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::Interface for ITsSbPropertySet {
    type Vtable = ITsSbPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::ComInterface for ITsSbPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c025171_bb1e_4baf_a212_6d5e9774b33b);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPropertySet_Vtbl {
    pub base__: super::Com::StructuredStorage::IPropertyBag_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbProvider(::windows_core::IUnknown);
impl ITsSbProvider {
    pub unsafe fn CreateTargetObject<P0, P1>(&self, targetname: P0, environmentname: P1) -> ::windows_core::Result<ITsSbTarget>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetObject)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), environmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateLoadBalanceResultObject<P0>(&self, targetname: P0) -> ::windows_core::Result<ITsSbLoadBalanceResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateLoadBalanceResultObject)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSessionObject<P0, P1, P2>(&self, targetname: P0, username: P1, domain: P2, sessionid: u32) -> ::windows_core::Result<ITsSbSession>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSessionObject)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), sessionid, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreatePluginPropertySet(&self) -> ::windows_core::Result<ITsSbPluginPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePluginPropertySet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateTargetPropertySetObject(&self) -> ::windows_core::Result<ITsSbTargetPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetPropertySetObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEnvironmentObject<P0>(&self, name: P0, serverweight: u32) -> ::windows_core::Result<ITsSbEnvironment>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateEnvironmentObject)(::windows_core::Interface::as_raw(self), name.into_param().abi(), serverweight, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResourcePluginStore(&self) -> ::windows_core::Result<ITsSbResourcePluginStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResourcePluginStore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilterPluginStore(&self) -> ::windows_core::Result<ITsSbFilterPluginStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilterPluginStore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterForNotification<P0, P1>(&self, notificationtype: u32, resourcetomonitor: P0, ppluginnotification: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<ITsSbResourceNotification>,
    {
        (::windows_core::Interface::vtable(self).RegisterForNotification)(::windows_core::Interface::as_raw(self), notificationtype, resourcetomonitor.into_param().abi(), ppluginnotification.into_param().abi()).ok()
    }
    pub unsafe fn UnRegisterForNotification<P0>(&self, notificationtype: u32, resourcetomonitor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).UnRegisterForNotification)(::windows_core::Interface::as_raw(self), notificationtype, resourcetomonitor.into_param().abi()).ok()
    }
    pub unsafe fn GetInstanceOfGlobalStore(&self) -> ::windows_core::Result<ITsSbGlobalStore> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInstanceOfGlobalStore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateEnvironmentPropertySetObject(&self) -> ::windows_core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateEnvironmentPropertySetObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbProvider, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbProvider {
    type Vtable = ITsSbProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87a4098f_6d7b_44dd_bc17_8ce44e370d52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateTargetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLoadBalanceResultObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pplbresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSessionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, sessionid: u32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreatePluginPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreatePluginPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateTargetPropertySetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateTargetPropertySetObject: usize,
    pub CreateEnvironmentObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverweight: u32, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetResourcePluginStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilterPluginStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterForNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppluginnotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnRegisterForNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetInstanceOfGlobalStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppglobalstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEnvironmentPropertySetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEnvironmentPropertySetObject: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbProvisioning(::windows_core::IUnknown);
impl ITsSbProvisioning {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn CreateVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<ITsSbProvisioningPluginNotifySink>,
    {
        (::windows_core::Interface::vtable(self).CreateVirtualMachines)(::windows_core::Interface::as_raw(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    pub unsafe fn PatchVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<ITsSbProvisioningPluginNotifySink>,
    {
        (::windows_core::Interface::vtable(self).PatchVirtualMachines)(::windows_core::Interface::as_raw(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi(), pvmpatchinfo).ok()
    }
    pub unsafe fn DeleteVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<ITsSbProvisioningPluginNotifySink>,
    {
        (::windows_core::Interface::vtable(self).DeleteVirtualMachines)(::windows_core::Interface::as_raw(self), jobxmlstring.into_param().abi(), jobguid.into_param().abi(), psink.into_param().abi()).ok()
    }
    pub unsafe fn CancelJob<P0>(&self, jobguid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CancelJob)(::windows_core::Interface::as_raw(self), jobguid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbProvisioning, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbProvisioning {
    type Vtable = ITsSbProvisioning_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbProvisioning {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f6f0dbb_9e4f_462b_9c3f_fccc3dcb6232);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioning_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub CreateVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PatchVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows_core::HRESULT,
    pub DeleteVirtualMachines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbProvisioningPluginNotifySink(::windows_core::IUnknown);
impl ITsSbProvisioningPluginNotifySink {
    pub unsafe fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnJobCreated)(::windows_core::Interface::as_raw(self), pvmnotifyinfo).ok()
    }
    pub unsafe fn OnVirtualMachineStatusChanged<P0>(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnVirtualMachineStatusChanged)(::windows_core::Interface::as_raw(self), pvmnotifyentry, vmnotifystatus, errorcode, errordescr.into_param().abi()).ok()
    }
    pub unsafe fn OnJobCompleted<P0>(&self, resultcode: ::windows_core::HRESULT, resultdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnJobCompleted)(::windows_core::Interface::as_raw(self), resultcode, resultdescription.into_param().abi()).ok()
    }
    pub unsafe fn OnJobCancelled(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnJobCancelled)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockVirtualMachine)(::windows_core::Interface::as_raw(self), pvmnotifyentry).ok()
    }
    pub unsafe fn OnVirtualMachineHostStatusChanged<P0, P1>(&self, vmhost: P0, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnVirtualMachineHostStatusChanged)(::windows_core::Interface::as_raw(self), vmhost.into_param().abi(), vmhostnotifystatus, errorcode, errordescr.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbProvisioningPluginNotifySink, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbProvisioningPluginNotifySink {
    type Vtable = ITsSbProvisioningPluginNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbProvisioningPluginNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaca87a8e_818b_4581_a032_49c3dfb9c701);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioningPluginNotifySink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnJobCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows_core::HRESULT,
    pub OnVirtualMachineStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OnJobCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultcode: ::windows_core::HRESULT, resultdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OnJobCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LockVirtualMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows_core::HRESULT,
    pub OnVirtualMachineHostStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vmhost: ::std::mem::MaybeUninit<::windows_core::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbResourceNotification(::windows_core::IUnknown);
impl ITsSbResourceNotification {
    pub unsafe fn NotifySessionChange<P0>(&self, changetype: TSSESSION_STATE, psession: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbSession>,
    {
        (::windows_core::Interface::vtable(self).NotifySessionChange)(::windows_core::Interface::as_raw(self), changetype, psession.into_param().abi()).ok()
    }
    pub unsafe fn NotifyTargetChange<P0>(&self, targetchangetype: u32, ptarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTarget>,
    {
        (::windows_core::Interface::vtable(self).NotifyTargetChange)(::windows_core::Interface::as_raw(self), targetchangetype, ptarget.into_param().abi()).ok()
    }
    pub unsafe fn NotifyClientConnectionStateChange<P0>(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbClientConnection>,
    {
        (::windows_core::Interface::vtable(self).NotifyClientConnectionStateChange)(::windows_core::Interface::as_raw(self), changetype, pconnection.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbResourceNotification, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbResourceNotification {
    type Vtable = ITsSbResourceNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbResourceNotification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65d3e85a_c39b_11dc_b92d_3cd255d89593);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotification_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NotifySessionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyTargetChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyClientConnectionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbResourceNotificationEx(::windows_core::IUnknown);
impl ITsSbResourceNotificationEx {
    pub unsafe fn NotifySessionChangeEx<P0, P1, P2>(&self, targetname: P0, username: P1, domain: P2, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NotifySessionChangeEx)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), username.into_param().abi(), domain.into_param().abi(), sessionid, sessionstate).ok()
    }
    pub unsafe fn NotifyTargetChangeEx<P0>(&self, targetname: P0, targetchangetype: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NotifyTargetChangeEx)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), targetchangetype).ok()
    }
    pub unsafe fn NotifyClientConnectionStateChangeEx<P0, P1, P2, P3, P4>(&self, username: P0, domain: P1, initialprogram: P2, poolname: P3, targetname: P4, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NotifyClientConnectionStateChangeEx)(::windows_core::Interface::as_raw(self), username.into_param().abi(), domain.into_param().abi(), initialprogram.into_param().abi(), poolname.into_param().abi(), targetname.into_param().abi(), connectionchangetype).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbResourceNotificationEx, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbResourceNotificationEx {
    type Vtable = ITsSbResourceNotificationEx_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbResourceNotificationEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8a47fde_ca91_44d2_b897_3aa28a43b2b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotificationEx_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NotifySessionChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows_core::HRESULT,
    pub NotifyTargetChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetchangetype: u32) -> ::windows_core::HRESULT,
    pub NotifyClientConnectionStateChangeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbResourcePlugin(::windows_core::IUnknown);
impl ITsSbResourcePlugin {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbResourcePlugin, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbResourcePlugin {
    type Vtable = ITsSbResourcePlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbResourcePlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea8db42c_98ed_4535_a88b_2a164f35490f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbResourcePluginStore(::windows_core::IUnknown);
impl ITsSbResourcePluginStore {
    pub unsafe fn QueryTarget<P0, P1>(&self, targetname: P0, farmname: P1) -> ::windows_core::Result<ITsSbTarget>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryTarget)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), farmname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn QuerySessionBySessionId<P0>(&self, dwsessionid: u32, targetname: P0) -> ::windows_core::Result<ITsSbSession>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QuerySessionBySessionId)(::windows_core::Interface::as_raw(self), dwsessionid, targetname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddTargetToStore<P0>(&self, ptarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTarget>,
    {
        (::windows_core::Interface::vtable(self).AddTargetToStore)(::windows_core::Interface::as_raw(self), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn AddSessionToStore<P0>(&self, psession: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbSession>,
    {
        (::windows_core::Interface::vtable(self).AddSessionToStore)(::windows_core::Interface::as_raw(self), psession.into_param().abi()).ok()
    }
    pub unsafe fn AddEnvironmentToStore<P0>(&self, penvironment: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbEnvironment>,
    {
        (::windows_core::Interface::vtable(self).AddEnvironmentToStore)(::windows_core::Interface::as_raw(self), penvironment.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveEnvironmentFromStore<P0, P1>(&self, environmentname: P0, bignoreowner: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).RemoveEnvironmentFromStore)(::windows_core::Interface::as_raw(self), environmentname.into_param().abi(), bignoreowner.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumerateFarms)(::windows_core::Interface::as_raw(self), pdwcount, pval).ok()
    }
    pub unsafe fn QueryEnvironment<P0>(&self, environmentname: P0) -> ::windows_core::Result<ITsSbEnvironment>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryEnvironment)(::windows_core::Interface::as_raw(self), environmentname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumerateEnvironments)(::windows_core::Interface::as_raw(self), pdwcount, pval).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveTarget<P0, P1>(&self, ptarget: P0, bforcewrite: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTarget>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SaveTarget)(::windows_core::Interface::as_raw(self), ptarget.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveEnvironment<P0, P1>(&self, penvironment: P0, bforcewrite: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbEnvironment>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SaveEnvironment)(::windows_core::Interface::as_raw(self), penvironment.into_param().abi(), bforcewrite.into_param().abi()).ok()
    }
    pub unsafe fn SaveSession<P0>(&self, psession: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbSession>,
    {
        (::windows_core::Interface::vtable(self).SaveSession)(::windows_core::Interface::as_raw(self), psession.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTargetProperty<P0, P1>(&self, targetname: P0, propertyname: P1, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetProperty)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), propertyname.into_param().abi(), pproperty).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEnvironmentProperty<P0, P1>(&self, environmentname: P0, propertyname: P1, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEnvironmentProperty)(::windows_core::Interface::as_raw(self), environmentname.into_param().abi(), propertyname.into_param().abi(), pproperty).ok()
    }
    pub unsafe fn SetTargetState<P0>(&self, targetname: P0, newstate: TARGET_STATE) -> ::windows_core::Result<TARGET_STATE>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetTargetState)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), newstate, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSessionState<P0>(&self, sbsession: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbSession>,
    {
        (::windows_core::Interface::vtable(self).SetSessionState)(::windows_core::Interface::as_raw(self), sbsession.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateTargets<P0, P1, P2>(&self, farmname: P0, envname: P1, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: P2, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateTargets)(::windows_core::Interface::as_raw(self), farmname.into_param().abi(), envname.into_param().abi(), sortbyfieldid, sortybypropname.into_param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateSessions<P0, P1, P2, P3, P4>(&self, targetname: P0, username: P1, userdomain: P2, poolname: P3, initialprogram: P4, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumerateSessions)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), username.into_param().abi(), userdomain.into_param().abi(), poolname.into_param().abi(), initialprogram.into_param().abi(), psessionstate, pdwcount, ppval).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFarmProperty<P0, P1>(&self, farmname: P0, propertyname: P1, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetFarmProperty)(::windows_core::Interface::as_raw(self), farmname.into_param().abi(), propertyname.into_param().abi(), pvarvalue).ok()
    }
    pub unsafe fn DeleteTarget<P0, P1>(&self, targetname: P0, hostname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteTarget)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), hostname.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTargetPropertyWithVersionCheck<P0, P1>(&self, ptarget: P0, propertyname: P1, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTarget>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetPropertyWithVersionCheck)(::windows_core::Interface::as_raw(self), ptarget.into_param().abi(), propertyname.into_param().abi(), pproperty).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEnvironmentPropertyWithVersionCheck<P0, P1>(&self, penvironment: P0, propertyname: P1, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbEnvironment>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEnvironmentPropertyWithVersionCheck)(::windows_core::Interface::as_raw(self), penvironment.into_param().abi(), propertyname.into_param().abi(), pproperty).ok()
    }
    pub unsafe fn AcquireTargetLock<P0>(&self, targetname: P0, dwtimeout: u32) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AcquireTargetLock)(::windows_core::Interface::as_raw(self), targetname.into_param().abi(), dwtimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseTargetLock<P0>(&self, pcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).ReleaseTargetLock)(::windows_core::Interface::as_raw(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn TestAndSetServerState<P0, P1>(&self, poolname: P0, serverfqdn: P1, newstate: TARGET_STATE, teststate: TARGET_STATE) -> ::windows_core::Result<TARGET_STATE>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TestAndSetServerState)(::windows_core::Interface::as_raw(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), newstate, teststate, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServerWaitingToStart<P0, P1>(&self, poolname: P0, servername: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServerWaitingToStart)(::windows_core::Interface::as_raw(self), poolname.into_param().abi(), servername.into_param().abi()).ok()
    }
    pub unsafe fn GetServerState<P0, P1>(&self, poolname: P0, serverfqdn: P1) -> ::windows_core::Result<TARGET_STATE>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetServerState)(::windows_core::Interface::as_raw(self), poolname.into_param().abi(), serverfqdn.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServerDrainMode<P0>(&self, serverfqdn: P0, drainmode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServerDrainMode)(::windows_core::Interface::as_raw(self), serverfqdn.into_param().abi(), drainmode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbResourcePluginStore, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbResourcePluginStore {
    type Vtable = ITsSbResourcePluginStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbResourcePluginStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c38f65f_bcf1_4036_a6bf_9e3cccae0b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePluginStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddTargetToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddSessionToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddEnvironmentToStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveEnvironmentFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveEnvironmentFromStore: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub QueryEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumerateEnvironments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveEnvironment: usize,
    pub SaveSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTargetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEnvironmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEnvironmentProperty: usize,
    pub SetTargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows_core::HRESULT,
    pub SetSessionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbsession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumerateTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, envname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFarmProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFarmProperty: usize,
    pub DeleteTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, hostname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTargetPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTargetPropertyWithVersionCheck: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEnvironmentPropertyWithVersionCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEnvironmentPropertyWithVersionCheck: usize,
    pub AcquireTargetLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseTargetLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TestAndSetServerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows_core::HRESULT,
    pub SetServerWaitingToStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, servername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetServerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pstate: *mut TARGET_STATE) -> ::windows_core::HRESULT,
    pub SetServerDrainMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, drainmode: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbServiceNotification(::windows_core::IUnknown);
impl ITsSbServiceNotification {
    pub unsafe fn NotifyServiceFailure(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyServiceFailure)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyServiceSuccess(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyServiceSuccess)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbServiceNotification, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbServiceNotification {
    type Vtable = ITsSbServiceNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbServiceNotification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86cb68ae_86e0_4f57_8a64_bb7406bc5550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbServiceNotification_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NotifyServiceFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyServiceSuccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbSession(::windows_core::IUnknown);
impl ITsSbSession {
    pub unsafe fn SessionId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SessionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TargetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetName<P0>(&self, targetname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetName)(::windows_core::Interface::as_raw(self), targetname.into_param().abi()).ok()
    }
    pub unsafe fn Username(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Username)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<TSSESSION_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: TSSESSION_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetState)(::windows_core::Interface::as_raw(self), state).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreateTime(&self, time: super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreateTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(time)).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisconnectTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisconnectTime(&self, time: super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisconnectTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(time)).ok()
    }
    pub unsafe fn InitialProgram(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InitialProgram)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInitialProgram<P0>(&self, application: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInitialProgram)(::windows_core::Interface::as_raw(self), application.into_param().abi()).ok()
    }
    pub unsafe fn ClientDisplay(&self) -> ::windows_core::Result<CLIENT_DISPLAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ClientDisplay)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClientDisplay(&self, pclientdisplay: CLIENT_DISPLAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientdisplay)).ok()
    }
    pub unsafe fn ProtocolType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProtocolType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtocolType(&self, val: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtocolType)(::windows_core::Interface::as_raw(self), val).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbSession, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbSession {
    type Vtable = ITsSbSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd453aac7_b1d8_4c5e_ba34_9afb4c8c5510);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbSession_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Username: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisconnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisconnectTime: usize,
    pub InitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, app: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInitialProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ClientDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows_core::HRESULT,
    pub SetClientDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows_core::HRESULT,
    pub ProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT,
    pub SetProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbTarget(::windows_core::IUnknown);
impl ITsSbTarget {
    pub unsafe fn TargetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetName<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetName)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    pub unsafe fn FarmName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FarmName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFarmName<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFarmName)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    pub unsafe fn TargetFQDN(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetFQDN)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetFQDN<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetFQDN)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    pub unsafe fn TargetNetbios(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetNetbios)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetNetbios<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTargetNetbios)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    pub unsafe fn get_IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_IpAddresses)(::windows_core::Interface::as_raw(self), sockaddr, numaddresses).ok()
    }
    pub unsafe fn put_IpAddresses(&self, sockaddr: &[TSSD_ConnectionPoint]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_IpAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sockaddr.as_ptr()), sockaddr.len() as _).ok()
    }
    pub unsafe fn TargetState(&self) -> ::windows_core::Result<TARGET_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTargetState(&self, state: TARGET_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetState)(::windows_core::Interface::as_raw(self), state).ok()
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn TargetPropertySet(&self) -> ::windows_core::Result<ITsSbTargetPropertySet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetPropertySet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetTargetPropertySet<P0>(&self, pval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTargetPropertySet>,
    {
        (::windows_core::Interface::vtable(self).SetTargetPropertySet)(::windows_core::Interface::as_raw(self), pval.into_param().abi()).ok()
    }
    pub unsafe fn EnvironmentName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnvironmentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnvironmentName<P0>(&self, val: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEnvironmentName)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
    pub unsafe fn NumSessions(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NumSessions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumPendingConnections(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NumPendingConnections)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TargetLoad(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetLoad)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbTarget, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbTarget {
    type Vtable = ITsSbTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16616ecc_272d_411d_b324_126893033856);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFarmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TargetFQDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetfqdnname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetFQDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TargetNetbios: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTargetNetbios: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_IpAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows_core::HRESULT,
    pub put_IpAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows_core::HRESULT,
    pub TargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows_core::HRESULT,
    pub SetTargetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub TargetPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    TargetPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetTargetPropertySet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetTargetPropertySet: usize,
    pub EnvironmentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEnvironmentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NumSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows_core::HRESULT,
    pub NumPendingConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows_core::HRESULT,
    pub TargetLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbTargetPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTargetPropertySet {
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Variant::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Com::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar, perrorlog.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`, `Win32_System_Variant`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
::windows_core::imp::interface_hierarchy!(ITsSbTargetPropertySet, ::windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::Interface for ITsSbTargetPropertySet {
    type Vtable = ITsSbTargetPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows_core::ComInterface for ITsSbTargetPropertySet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7bda5d6_994c_4e11_a079_2763b61830ac);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTargetPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbTaskInfo(::windows_core::IUnknown);
impl ITsSbTaskInfo {
    pub unsafe fn TargetId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Deadline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Identifier(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Identifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Label(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Label)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Context(&self) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Context)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Plugin(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Plugin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<RDV_TASK_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbTaskInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITsSbTaskInfo {
    type Vtable = ITsSbTaskInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbTaskInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x523d1083_89be_48dd_99ea_04e82ffa7265);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub TargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deadline: usize,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Context: usize,
    pub Plugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplugin: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbTaskPlugin(::windows_core::IUnknown);
impl ITsSbTaskPlugin {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbProvider>,
        P1: ::windows_core::IntoParam<ITsSbPluginNotifySink>,
        P2: ::windows_core::IntoParam<ITsSbPluginPropertySet>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pprovider.into_param().abi(), pnotifysink.into_param().abi(), ppropertyset.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Terminate)(::windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn InitializeTaskPlugin<P0>(&self, pitssbtaskpluginnotifysink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITsSbTaskPluginNotifySink>,
    {
        (::windows_core::Interface::vtable(self).InitializeTaskPlugin)(::windows_core::Interface::as_raw(self), pitssbtaskpluginnotifysink.into_param().abi()).ok()
    }
    pub unsafe fn SetTaskQueue<P0>(&self, pszhostname: P0, pitssbtaskinfo: &[::core::option::Option<ITsSbTaskInfo>]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTaskQueue)(::windows_core::Interface::as_raw(self), pszhostname.into_param().abi(), pitssbtaskinfo.len() as _, ::core::mem::transmute(pitssbtaskinfo.as_ptr())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbTaskPlugin, ::windows_core::IUnknown, ITsSbPlugin);
unsafe impl ::windows_core::Interface for ITsSbTaskPlugin {
    type Vtable = ITsSbTaskPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbTaskPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa22ef0f_8705_41be_93bc_44bdbcf1c9c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub InitializeTaskPlugin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTaskQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhostname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITsSbTaskPluginNotifySink(::windows_core::IUnknown);
impl ITsSbTaskPluginNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnError)(::windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnReportStatus)(::windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnSetTaskTime<P0, P1, P2, P3>(&self, sztargetname: P0, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: P1, sztaskidentifier: P2, sztaskplugin: P3, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnSetTaskTime)(::windows_core::Interface::as_raw(self), sztargetname.into_param().abi(), ::core::mem::transmute(taskstarttime), ::core::mem::transmute(taskendtime), ::core::mem::transmute(taskdeadline), sztasklabel.into_param().abi(), sztaskidentifier.into_param().abi(), sztaskplugin.into_param().abi(), dwtaskstatus, sacontext).ok()
    }
    pub unsafe fn OnDeleteTaskTime<P0, P1>(&self, sztargetname: P0, sztaskidentifier: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnDeleteTaskTime)(::windows_core::Interface::as_raw(self), sztargetname.into_param().abi(), sztaskidentifier.into_param().abi()).ok()
    }
    pub unsafe fn OnUpdateTaskStatus<P0, P1>(&self, sztargetname: P0, taskidentifier: P1, taskstatus: RDV_TASK_STATUS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnUpdateTaskStatus)(::windows_core::Interface::as_raw(self), sztargetname.into_param().abi(), taskidentifier.into_param().abi(), taskstatus).ok()
    }
    pub unsafe fn OnReportTasks<P0>(&self, szhostname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnReportTasks)(::windows_core::Interface::as_raw(self), szhostname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITsSbTaskPluginNotifySink, ::windows_core::IUnknown, ITsSbBaseNotifySink);
unsafe impl ::windows_core::Interface for ITsSbTaskPluginNotifySink {
    type Vtable = ITsSbTaskPluginNotifySink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITsSbTaskPluginNotifySink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aaf899e_c2ec_45ee_aa37_45e60895261a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnSetTaskTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskplugin: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnSetTaskTime: usize,
    pub OnDeleteTaskTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OnUpdateTaskStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows_core::HRESULT,
    pub OnReportTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szhostname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsEnhancedFastReconnectArbitrator(::windows_core::IUnknown);
impl IWRdsEnhancedFastReconnectArbitrator {
    pub unsafe fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSessionForEnhancedFastReconnect)(::windows_core::Interface::as_raw(self), psessionidarray, dwsessioncount, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsEnhancedFastReconnectArbitrator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsEnhancedFastReconnectArbitrator {
    type Vtable = IWRdsEnhancedFastReconnectArbitrator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsEnhancedFastReconnectArbitrator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5718ae9b_47f2_499f_b634_d8175bd51131);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSessionForEnhancedFastReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsGraphicsChannel(::windows_core::IUnknown);
impl IWRdsGraphicsChannel {
    pub unsafe fn Write<P0>(&self, cbsize: u32, pbuffer: *const u8, pcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), cbsize, pbuffer, pcontext.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0, P1>(&self, pchannelevents: P0, popencontext: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWRdsGraphicsChannelEvents>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pchannelevents.into_param().abi(), popencontext.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannel, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsGraphicsChannel {
    type Vtable = IWRdsGraphicsChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsGraphicsChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x684b7a0b_edff_43ad_d5a2_4a8d5388f401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannel_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannelevents: *mut ::core::ffi::c_void, popencontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsGraphicsChannelEvents(::windows_core::IUnknown);
impl IWRdsGraphicsChannelEvents {
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataReceived)(::windows_core::Interface::as_raw(self), cbsize, pbuffer).ok()
    }
    pub unsafe fn OnClose(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnClose)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnChannelOpened<P0>(&self, openresult: ::windows_core::HRESULT, popencontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).OnChannelOpened)(::windows_core::Interface::as_raw(self), openresult, popencontext.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDataSent<P0, P1>(&self, pwritecontext: P0, bcancelled: P1, pbuffer: *const u8, cbbuffer: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnDataSent)(::windows_core::Interface::as_raw(self), pwritecontext.into_param().abi(), bcancelled.into_param().abi(), pbuffer, cbbuffer).ok()
    }
    pub unsafe fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMetricsUpdate)(::windows_core::Interface::as_raw(self), bandwidth, rtt, lastsentbyteindex).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsGraphicsChannelEvents {
    type Vtable = IWRdsGraphicsChannelEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsGraphicsChannelEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67f2368c_d674_4fae_66a5_d20628a640d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnChannelOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openresult: ::windows_core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDataSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDataSent: usize,
    pub OnMetricsUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsGraphicsChannelManager(::windows_core::IUnknown);
impl IWRdsGraphicsChannelManager {
    pub unsafe fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> ::windows_core::Result<IWRdsGraphicsChannel> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateChannel)(::windows_core::Interface::as_raw(self), pszchannelname, channeltype, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsGraphicsChannelManager {
    type Vtable = IWRdsGraphicsChannelManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsGraphicsChannelManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fd57159_e83e_476a_a8b9_4a7976e71e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolConnection(::windows_core::IUnknown);
impl IWRdsProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows_core::Result<IWRdsProtocolLogonErrorRedirector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLogonErrorRedirector)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AcceptConnection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcceptConnection)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientData)(::windows_core::Interface::as_raw(self), pclientdata).ok()
    }
    pub unsafe fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientMonitorData)(::windows_core::Interface::as_raw(self), pnummonitors, pprimarymonitor).ok()
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserCredentials)(::windows_core::Interface::as_raw(self), pusercreds).ok()
    }
    pub unsafe fn GetLicenseConnection(&self) -> ::windows_core::Result<IWRdsProtocolLicenseConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLicenseConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AuthenticateClientToSession)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifySessionId<P0>(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
    {
        (::windows_core::Interface::vtable(self).NotifySessionId)(::windows_core::Interface::as_raw(self), sessionid, sessionhandle.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInputHandles)(::windows_core::Interface::as_raw(self), pkeyboardhandle, pmousehandle, pbeephandle).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVideoHandle(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVideoHandle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConnectNotify)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserAllowedToLogon<P0, P1, P2>(&self, sessionid: u32, usertoken: P0, pdomainname: P1, pusername: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).IsUserAllowedToLogon)(::windows_core::Interface::as_raw(self), sessionid, usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionArbitrationEnumeration<P0, P1>(&self, husertoken: P0, bsinglesessionperuserenabled: P1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(::windows_core::Interface::as_raw(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), psessionidarray, pdwsessionidentifiercount).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogonNotify<P0, P1, P2>(&self, hclienttoken: P0, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LogonNotify)(::windows_core::Interface::as_raw(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), sessionid, pwrdsconnectionsettings).ok()
    }
    pub unsafe fn PreDisconnect(&self, disconnectreason: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreDisconnect)(::windows_core::Interface::as_raw(self), disconnectreason).ok()
    }
    pub unsafe fn DisconnectNotify(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectNotify)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProtocolStatus)(::windows_core::Interface::as_raw(self), pprotocolstatus).ok()
    }
    pub unsafe fn GetLastInputTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastInputTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorInfo)(::windows_core::Interface::as_raw(self), ulerror).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualChannel<P0, P1>(&self, szendpointname: P0, bstatic: P1, requestedpriority: u32) -> ::windows_core::Result<usize>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateVirtualChannel)(::windows_core::Interface::as_raw(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), requestedpriority, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryProperty(&self, querytype: ::windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(querytype), ppropertyentriesin.len() as _, ppropertyentriesout.len() as _, ::core::mem::transmute(ppropertyentriesin.as_ptr()), ::core::mem::transmute(ppropertyentriesout.as_ptr())).ok()
    }
    pub unsafe fn GetShadowConnection(&self) -> ::windows_core::Result<IWRdsProtocolShadowConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetShadowConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyCommandProcessCreated(&self, sessionid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyCommandProcessCreated)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolConnection {
    type Vtable = IWRdsProtocolConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324ed94f_fdaf_4ff6_81a8_42abe755830b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetClientMonitorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows_core::HRESULT,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySessionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputHandles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVideoHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVideoHandle: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_core::PCWSTR, pusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_core::PCWSTR, wszdomainname: ::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    pub PreDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows_core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szendpointname: ::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: ::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyCommandProcessCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolConnectionCallback(::windows_core::IUnknown);
impl IWRdsProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnReady)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BrokenConnection)(::windows_core::Interface::as_raw(self), reason, source).ok()
    }
    pub unsafe fn StopScreenUpdates(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopScreenUpdates)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RedrawWindow)(::windows_core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn GetConnectionId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConnectionId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolConnectionCallback {
    type Vtable = IWRdsProtocolConnectionCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolConnectionCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1d70332_d070_4ef1_a088_78313536c2d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows_core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolConnectionSettings(::windows_core::IUnknown);
impl IWRdsProtocolConnectionSettings {
    pub unsafe fn SetConnectionSetting(&self, propertyid: ::windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectionSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertyid), ppropertyentriesin).ok()
    }
    pub unsafe fn GetConnectionSetting(&self, propertyid: ::windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConnectionSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertyid), ppropertyentriesout).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionSettings, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolConnectionSettings {
    type Vtable = IWRdsProtocolConnectionSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolConnectionSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83fcf5d3_f6f4_ea94_9cd2_32f280e1e510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionSettings_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetConnectionSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT,
    pub GetConnectionSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolLicenseConnection(::windows_core::IUnknown);
impl IWRdsProtocolLicenseConnection {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestLicensingCapabilities)(::windows_core::Interface::as_raw(self), pplicensecapabilities, pcblicensecapabilities).ok()
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendClientLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len() as _).ok()
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestClientLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserve1.as_ptr()), reserve1.len() as _, ppclientlicense, pcbclientlicense).ok()
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProtocolComplete)(::windows_core::Interface::as_raw(self), ulcomplete).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolLicenseConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolLicenseConnection {
    type Vtable = IWRdsProtocolLicenseConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolLicenseConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d6a145f_d095_4424_957a_407fae822d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLicenseConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolListener(::windows_core::IUnknown);
impl IWRdsProtocolListener {
    pub unsafe fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> ::windows_core::Result<WRDS_LISTENER_SETTINGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettings)(::windows_core::Interface::as_raw(self), wrdslistenersettinglevel, &mut result__).from_abi(result__)
    }
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWRdsProtocolListenerCallback>,
    {
        (::windows_core::Interface::vtable(self).StartListen)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn StopListen(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopListen)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolListener, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolListener {
    type Vtable = IWRdsProtocolListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcbc131b_c686_451d_a773_e279e230f540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListener_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows_core::HRESULT,
    pub StartListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolListenerCallback(::windows_core::IUnknown);
impl IWRdsProtocolListenerCallback {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<IWRdsProtocolConnectionCallback>
    where
        P0: ::windows_core::IntoParam<IWRdsProtocolConnection>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OnConnected)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi(), pwrdsconnectionsettings, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolListenerCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolListenerCallback {
    type Vtable = IWRdsProtocolListenerCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolListenerCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ab27e5b_4449_4dc1_b74a_91621d4fe984);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListenerCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnConnected: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolLogonErrorRedirector(::windows_core::IUnknown);
impl IWRdsProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnBeginPainting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectStatus)(::windows_core::Interface::as_raw(self), pszmessage.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectMessage)(::windows_core::Interface::as_raw(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), utype, &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectLogonError<P0, P1>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P0, pszmessage: P1, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectLogonError)(::windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.into_param().abi(), pszmessage.into_param().abi(), utype, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolLogonErrorRedirector, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolLogonErrorRedirector {
    type Vtable = IWRdsProtocolLogonErrorRedirector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolLogonErrorRedirector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x519fe83b_142a_4120_a3d5_a405d315281a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmessage: ::windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolManager(::windows_core::IUnknown);
impl IWRdsProtocolManager {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, piwrdssettings: P0, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWRdsProtocolSettings>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), piwrdssettings.into_param().abi(), pwrdssettings).ok()
    }
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> ::windows_core::Result<IWRdsProtocolListener>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateListener)(::windows_core::Interface::as_raw(self), wszlistenername.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyServiceStateChange)(::windows_core::Interface::as_raw(self), ptsservicestatechange).ok()
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionStateChange)(::windows_core::Interface::as_raw(self), sessionid, eventid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySettingsChange)(::windows_core::Interface::as_raw(self), pwrdssettings).ok()
    }
    pub unsafe fn Uninitialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Uninitialize)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolManager {
    type Vtable = IWRdsProtocolManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc796967_3abb_40cd_a446_105276b58950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwrdssettings: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlistenername: ::windows_core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotifySettingsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifySettingsChange: usize,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolSettings(::windows_core::IUnknown);
impl IWRdsProtocolSettings {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSettings)(::windows_core::Interface::as_raw(self), wrdssettingtype, wrdssettinglevel, pwrdssettings).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MergeSettings)(::windows_core::Interface::as_raw(self), pwrdssettings, wrdsconnectionsettinglevel, pwrdsconnectionsettings).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolSettings, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolSettings {
    type Vtable = IWRdsProtocolSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x654a5a6a_2550_47eb_b6f7_ebd637475265);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolSettings_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MergeSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MergeSettings: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolShadowCallback(::windows_core::IUnknown);
impl IWRdsProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopShadow)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvokeTargetShadow<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).InvokeTargetShadow)(::windows_core::Interface::as_raw(self), ptargetservername.into_param().abi(), targetsessionid, ::core::mem::transmute(pparam1.as_ptr()), pparam1.len() as _, ::core::mem::transmute(pparam2.as_ptr()), pparam2.len() as _, ::core::mem::transmute(pparam3.as_ptr()), pparam3.len() as _, ::core::mem::transmute(pparam4.as_ptr()), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolShadowCallback {
    type Vtable = IWRdsProtocolShadowCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolShadowCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0667ce0_0372_40d6_adb2_a0f3322674d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsProtocolShadowConnection(::windows_core::IUnknown);
impl IWRdsProtocolShadowConnection {
    pub unsafe fn Start<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWRdsProtocolShadowCallback>,
    {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ptargetservername.into_param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoTarget<P0>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DoTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pparam1.as_ptr()), pparam1.len() as _, ::core::mem::transmute(pparam2.as_ptr()), pparam2.len() as _, ::core::mem::transmute(pparam3.as_ptr()), pparam3.len() as _, ::core::mem::transmute(pparam4.as_ptr()), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsProtocolShadowConnection {
    type Vtable = IWRdsProtocolShadowConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsProtocolShadowConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ae85ce6_cade_4548_8feb_99016597f60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWRdsWddmIddProps(::windows_core::IUnknown);
impl IWRdsWddmIddProps {
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHardwareId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisplaydriverhardwareid.as_ptr()), pdisplaydriverhardwareid.len() as _).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDriverLoad<P0>(&self, sessionid: u32, driverhandle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
    {
        (::windows_core::Interface::vtable(self).OnDriverLoad)(::windows_core::Interface::as_raw(self), sessionid, driverhandle.into_param().abi()).ok()
    }
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDriverUnload)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableWddmIdd<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).EnableWddmIdd)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWRdsWddmIddProps, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWRdsWddmIddProps {
    type Vtable = IWRdsWddmIddProps_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWRdsWddmIddProps {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1382df4d_a289_43d1_a184_144726f9af90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsWddmIddProps_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: ::windows_core::PCWSTR, count: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDriverLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDriverLoad: usize,
    pub OnDriverUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableWddmIdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableWddmIdd: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSBitmapRenderService(::windows_core::IUnknown);
impl IWTSBitmapRenderService {
    pub unsafe fn GetMappedRenderer<P0>(&self, mappingid: u64, pmappedrenderercallback: P0) -> ::windows_core::Result<IWTSBitmapRenderer>
    where
        P0: ::windows_core::IntoParam<IWTSBitmapRendererCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMappedRenderer)(::windows_core::Interface::as_raw(self), mappingid, pmappedrenderercallback.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSBitmapRenderService, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSBitmapRenderService {
    type Vtable = IWTSBitmapRenderService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSBitmapRenderService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderService_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetMappedRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: *mut ::core::ffi::c_void, ppmappedrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSBitmapRenderer(::windows_core::IUnknown);
impl IWTSBitmapRenderer {
    pub unsafe fn Render(&self, imageformat: ::windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, pimagebuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Render)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(imageformat), dwwidth, dwheight, cbstride, pimagebuffer.len() as _, ::core::mem::transmute(pimagebuffer.as_ptr())).ok()
    }
    pub unsafe fn GetRendererStatistics(&self) -> ::windows_core::Result<BITMAP_RENDERER_STATISTICS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRendererStatistics)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveMapping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveMapping)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSBitmapRenderer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSBitmapRenderer {
    type Vtable = IWTSBitmapRenderer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSBitmapRenderer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageformat: ::windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows_core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows_core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSBitmapRendererCallback(::windows_core::IUnknown);
impl IWTSBitmapRendererCallback {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTargetSizeChanged(&self, rcnewsize: super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTargetSizeChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rcnewsize)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSBitmapRendererCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSBitmapRendererCallback {
    type Vtable = IWTSBitmapRendererCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSBitmapRendererCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRendererCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTargetSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTargetSizeChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSListener(::windows_core::IUnknown);
impl IWTSListener {
    #[doc = "Required features: `Win32_System_Com_StructuredStorage`"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetConfiguration(&self) -> ::windows_core::Result<super::Com::StructuredStorage::IPropertyBag> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConfiguration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSListener, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSListener {
    type Vtable = IWTSListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListener_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetConfiguration: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSListenerCallback(::windows_core::IUnknown);
impl IWTSListenerCallback {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnNewChannelConnection<P0, P1>(&self, pchannel: P0, data: P1, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::core::option::Option<IWTSVirtualChannelCallback>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWTSVirtualChannel>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnNewChannelConnection)(::windows_core::Interface::as_raw(self), pchannel.into_param().abi(), data.into_param().abi(), pbaccept, ::core::mem::transmute(ppcallback)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSListenerCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSListenerCallback {
    type Vtable = IWTSListenerCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSListenerCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListenerCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNewChannelConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNewChannelConnection: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSPlugin(::windows_core::IUnknown);
impl IWTSPlugin {
    pub unsafe fn Initialize<P0>(&self, pchannelmgr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWTSVirtualChannelManager>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pchannelmgr.into_param().abi()).ok()
    }
    pub unsafe fn Connected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnected)(::windows_core::Interface::as_raw(self), dwdisconnectcode).ok()
    }
    pub unsafe fn Terminated(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminated)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSPlugin, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSPlugin {
    type Vtable = IWTSPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230201_1439_4e62_a414_190d0ac3d40e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPlugin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannelmgr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSPluginServiceProvider(::windows_core::IUnknown);
impl IWTSPluginServiceProvider {
    pub unsafe fn GetService(&self, serviceid: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(serviceid), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSPluginServiceProvider, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSPluginServiceProvider {
    type Vtable = IWTSPluginServiceProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSPluginServiceProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPluginServiceProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows_core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolConnection(::windows_core::IUnknown);
impl IWTSProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> ::windows_core::Result<IWTSProtocolLogonErrorRedirector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLogonErrorRedirector)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendPolicyData)(::windows_core::Interface::as_raw(self), ppolicydata).ok()
    }
    pub unsafe fn AcceptConnection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcceptConnection)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientData)(::windows_core::Interface::as_raw(self), pclientdata).ok()
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserCredentials)(::windows_core::Interface::as_raw(self), pusercreds).ok()
    }
    pub unsafe fn GetLicenseConnection(&self) -> ::windows_core::Result<IWTSProtocolLicenseConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLicenseConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AuthenticateClientToSession)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionId)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProtocolHandles)(::windows_core::Interface::as_raw(self), pkeyboardhandle, pmousehandle, pbeephandle, pvideohandle).ok()
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConnectNotify)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserAllowedToLogon<P0, P1, P2>(&self, sessionid: u32, usertoken: P0, pdomainname: P1, pusername: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).IsUserAllowedToLogon)(::windows_core::Interface::as_raw(self), sessionid, usertoken.into_param().abi(), pdomainname.into_param().abi(), pusername.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionArbitrationEnumeration<P0, P1>(&self, husertoken: P0, bsinglesessionperuserenabled: P1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(::windows_core::Interface::as_raw(self), husertoken.into_param().abi(), bsinglesessionperuserenabled.into_param().abi(), psessionidarray, pdwsessionidentifiercount).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogonNotify<P0, P1, P2>(&self, hclienttoken: P0, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE_PTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LogonNotify)(::windows_core::Interface::as_raw(self), hclienttoken.into_param().abi(), wszusername.into_param().abi(), wszdomainname.into_param().abi(), sessionid).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserData)(::windows_core::Interface::as_raw(self), ppolicydata, pclientdata).ok()
    }
    pub unsafe fn DisconnectNotify(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectNotify)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProtocolStatus)(::windows_core::Interface::as_raw(self), pprotocolstatus).ok()
    }
    pub unsafe fn GetLastInputTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastInputTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorInfo)(::windows_core::Interface::as_raw(self), ulerror).ok()
    }
    pub unsafe fn SendBeep(&self, frequency: u32, duration: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendBeep)(::windows_core::Interface::as_raw(self), frequency, duration).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualChannel<P0, P1>(&self, szendpointname: P0, bstatic: P1, requestedpriority: u32) -> ::windows_core::Result<usize>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateVirtualChannel)(::windows_core::Interface::as_raw(self), szendpointname.into_param().abi(), bstatic.into_param().abi(), requestedpriority, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryProperty(&self, querytype: ::windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(querytype), ppropertyentriesin.len() as _, ppropertyentriesout.len() as _, ::core::mem::transmute(ppropertyentriesin.as_ptr()), ::core::mem::transmute(ppropertyentriesout.as_ptr())).ok()
    }
    pub unsafe fn GetShadowConnection(&self) -> ::windows_core::Result<IWTSProtocolShadowConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetShadowConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolConnection {
    type Vtable = IWTSProtocolConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23083765_9095_4648_98bf_ef81c914032d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SendPolicyData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendPolicyData: usize,
    pub AcceptConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClientData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClientData: usize,
    pub GetUserCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::HRESULT,
    pub NotifySessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProtocolHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProtocolHandles: usize,
    pub ConnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserAllowedToLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_core::PCWSTR, pusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserAllowedToLogon: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SessionArbitrationEnumeration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LogonNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_core::PCWSTR, wszdomainname: ::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogonNotify: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserData: usize,
    pub DisconnectNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows_core::HRESULT,
    pub SendBeep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szendpointname: ::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualChannel: usize,
    pub QueryProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytype: ::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolConnectionCallback(::windows_core::IUnknown);
impl IWTSProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnReady)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BrokenConnection)(::windows_core::Interface::as_raw(self), reason, source).ok()
    }
    pub unsafe fn StopScreenUpdates(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopScreenUpdates)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RedrawWindow)(::windows_core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisplayIOCtl)(::windows_core::Interface::as_raw(self), displayioctl).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolConnectionCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolConnectionCallback {
    type Vtable = IWTSProtocolConnectionCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolConnectionCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23083765_75eb_41fe_b4fb_e086242afa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnectionCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows_core::HRESULT,
    pub DisplayIOCtl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolLicenseConnection(::windows_core::IUnknown);
impl IWTSProtocolLicenseConnection {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestLicensingCapabilities)(::windows_core::Interface::as_raw(self), pplicensecapabilities, pcblicensecapabilities).ok()
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendClientLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len() as _).ok()
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestClientLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserve1.as_ptr()), reserve1.len() as _, ppclientlicense, pcbclientlicense).ok()
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProtocolComplete)(::windows_core::Interface::as_raw(self), ulcomplete).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolLicenseConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolLicenseConnection {
    type Vtable = IWTSProtocolLicenseConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolLicenseConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23083765_178c_4079_8e4a_fea6496a4d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLicenseConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestLicensingCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestLicensingCapabilities: usize,
    pub SendClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolListener(::windows_core::IUnknown);
impl IWTSProtocolListener {
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWTSProtocolListenerCallback>,
    {
        (::windows_core::Interface::vtable(self).StartListen)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn StopListen(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopListen)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolListener, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolListener {
    type Vtable = IWTSProtocolListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23083765_45f0_4394_8f69_32b2bc0ef4ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListener_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub StartListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolListenerCallback(::windows_core::IUnknown);
impl IWTSProtocolListenerCallback {
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0) -> ::windows_core::Result<IWTSProtocolConnectionCallback>
    where
        P0: ::windows_core::IntoParam<IWTSProtocolConnection>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OnConnected)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolListenerCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolListenerCallback {
    type Vtable = IWTSProtocolListenerCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolListenerCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23083765_1a2d_4de2_97de_4a35f260f0b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListenerCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolLogonErrorRedirector(::windows_core::IUnknown);
impl IWTSProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnBeginPainting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectStatus)(::windows_core::Interface::as_raw(self), pszmessage.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectMessage)(::windows_core::Interface::as_raw(self), pszcaption.into_param().abi(), pszmessage.into_param().abi(), utype, &mut result__).from_abi(result__)
    }
    pub unsafe fn RedirectLogonError<P0, P1>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P0, pszmessage: P1, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RedirectLogonError)(::windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.into_param().abi(), pszmessage.into_param().abi(), utype, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolLogonErrorRedirector, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolLogonErrorRedirector {
    type Vtable = IWTSProtocolLogonErrorRedirector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolLogonErrorRedirector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd9b61a7_2916_4627_8dee_4328711ad6cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLogonErrorRedirector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmessage: ::windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolManager(::windows_core::IUnknown);
impl IWTSProtocolManager {
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> ::windows_core::Result<IWTSProtocolListener>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateListener)(::windows_core::Interface::as_raw(self), wszlistenername.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyServiceStateChange)(::windows_core::Interface::as_raw(self), ptsservicestatechange).ok()
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(::windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifySessionStateChange)(::windows_core::Interface::as_raw(self), sessionid, eventid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolManager {
    type Vtable = IWTSProtocolManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9eaf6cc_ed79_4f01_821d_1f881b9f66cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlistenername: ::windows_core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolShadowCallback(::windows_core::IUnknown);
impl IWTSProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopShadow)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvokeTargetShadow<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).InvokeTargetShadow)(::windows_core::Interface::as_raw(self), ptargetservername.into_param().abi(), targetsessionid, ::core::mem::transmute(pparam1.as_ptr()), pparam1.len() as _, ::core::mem::transmute(pparam2.as_ptr()), pparam2.len() as _, ::core::mem::transmute(pparam3.as_ptr()), pparam3.len() as _, ::core::mem::transmute(pparam4.as_ptr()), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolShadowCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolShadowCallback {
    type Vtable = IWTSProtocolShadowCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolShadowCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x503a2504_aae5_4ab1_93e0_6d1c4bc6f71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSProtocolShadowConnection(::windows_core::IUnknown);
impl IWTSProtocolShadowConnection {
    pub unsafe fn Start<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWTSProtocolShadowCallback>,
    {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ptargetservername.into_param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.into_param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoTarget<P0>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DoTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pparam1.as_ptr()), pparam1.len() as _, ::core::mem::transmute(pparam2.as_ptr()), pparam2.len() as _, ::core::mem::transmute(pparam3.as_ptr()), pparam3.len() as _, ::core::mem::transmute(pparam4.as_ptr()), pparam4.len() as _, pclientname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSProtocolShadowConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSProtocolShadowConnection {
    type Vtable = IWTSProtocolShadowConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSProtocolShadowConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee3b0c14_37fb_456b_bab3_6d6cd51e13bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSSBPlugin(::windows_core::IUnknown);
impl IWTSSBPlugin {
    pub unsafe fn Initialize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WTSSBX_MachineChangeNotification)(::windows_core::Interface::as_raw(self), notificationtype, machineid, pmachineinfo).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, sessioninfo: &[WTSSBX_SESSION_INFO]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WTSSBX_SessionChangeNotification)(::windows_core::Interface::as_raw(self), notificationtype, machineid, sessioninfo.len() as _, ::core::mem::transmute(sessioninfo.as_ptr())).ok()
    }
    pub unsafe fn WTSSBX_GetMostSuitableServer<P0, P1, P2, P3>(&self, username: P0, domainname: P1, applicationtype: P2, farmname: P3, pmachineid: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WTSSBX_GetMostSuitableServer)(::windows_core::Interface::as_raw(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), farmname.into_param().abi(), pmachineid).ok()
    }
    pub unsafe fn Terminated(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminated)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WTSSBX_GetUserExternalSession<P0, P1, P2>(&self, username: P0, domainname: P1, applicationtype: P2, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WTSSBX_GetUserExternalSession)(::windows_core::Interface::as_raw(self), username.into_param().abi(), domainname.into_param().abi(), applicationtype.into_param().abi(), redirectorinternalip, psessionid, pmachineconnectinfo).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSSBPlugin, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSSBPlugin {
    type Vtable = IWTSSBPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSSBPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc44be78_b18d_4399_b210_641bf67a002c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSSBPlugin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub WTSSBX_MachineChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WTSSBX_SessionChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WTSSBX_SessionChangeNotification: usize,
    pub WTSSBX_GetMostSuitableServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::windows_core::PCWSTR, domainname: ::windows_core::PCWSTR, applicationtype: ::windows_core::PCWSTR, farmname: ::windows_core::PCWSTR, pmachineid: *mut i32) -> ::windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WTSSBX_GetUserExternalSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::windows_core::PCWSTR, domainname: ::windows_core::PCWSTR, applicationtype: ::windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSVirtualChannel(::windows_core::IUnknown);
impl IWTSVirtualChannel {
    pub unsafe fn Write<P0>(&self, pbuffer: &[u8], preserved: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), preserved.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSVirtualChannel, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSVirtualChannel {
    type Vtable = IWTSVirtualChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSVirtualChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannel_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSVirtualChannelCallback(::windows_core::IUnknown);
impl IWTSVirtualChannelCallback {
    pub unsafe fn OnDataReceived(&self, pbuffer: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataReceived)(::windows_core::Interface::as_raw(self), pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr())).ok()
    }
    pub unsafe fn OnClose(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnClose)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWTSVirtualChannelCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSVirtualChannelCallback {
    type Vtable = IWTSVirtualChannelCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSVirtualChannelCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWTSVirtualChannelManager(::windows_core::IUnknown);
impl IWTSVirtualChannelManager {
    pub unsafe fn CreateListener<P0, P1>(&self, pszchannelname: P0, uflags: u32, plistenercallback: P1) -> ::windows_core::Result<IWTSListener>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<IWTSListenerCallback>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateListener)(::windows_core::Interface::as_raw(self), pszchannelname.into_param().abi(), uflags, plistenercallback.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWTSVirtualChannelManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWTSVirtualChannelManager {
    type Vtable = IWTSVirtualChannelManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWTSVirtualChannelManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszchannelname: ::windows_core::PCSTR, uflags: u32, plistenercallback: *mut ::core::ffi::c_void, pplistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspace(::windows_core::IUnknown);
impl IWorkspace {
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWorkspaceNames)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication<P0>(&self, bstrworkspaceid: P0, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StartRemoteApplication)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProcessId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspace, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspace {
    type Vtable = IWorkspace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWorkspaceNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWorkspaceNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub StartRemoteApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StartRemoteApplication: usize,
    pub GetProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspace2(::windows_core::IUnknown);
impl IWorkspace2 {
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWorkspaceNames)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication<P0>(&self, bstrworkspaceid: P0, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.StartRemoteApplication)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProcessId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplicationEx<P0, P1, P2, P3, P4>(&self, bstrworkspaceid: P0, bstrrequestingappid: P1, bstrrequestingappfamilyname: P2, blaunchintoimmersiveclient: P3, bstrimmersiveclientactivationcontext: P4, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StartRemoteApplicationEx)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrrequestingappid.into_param().abi(), bstrrequestingappfamilyname.into_param().abi(), blaunchintoimmersiveclient.into_param().abi(), bstrimmersiveclientactivationcontext.into_param().abi(), psaparams).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspace2, ::windows_core::IUnknown, IWorkspace);
unsafe impl ::windows_core::Interface for IWorkspace2 {
    type Vtable = IWorkspace2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspace2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96d8d7cf_783e_4286_834c_ebc0e95f783c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace2_Vtbl {
    pub base__: IWorkspace_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub StartRemoteApplicationEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrequestingappid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrequestingappfamilyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: ::std::mem::MaybeUninit<::windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    StartRemoteApplicationEx: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspace3(::windows_core::IUnknown);
impl IWorkspace3 {
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWorkspaceNames)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_System_Com`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication<P0>(&self, bstrworkspaceid: P0, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.StartRemoteApplication)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetProcessId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplicationEx<P0, P1, P2, P3, P4>(&self, bstrworkspaceid: P0, bstrrequestingappid: P1, bstrrequestingappfamilyname: P2, blaunchintoimmersiveclient: P3, bstrimmersiveclientactivationcontext: P4, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.StartRemoteApplicationEx)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrrequestingappid.into_param().abi(), bstrrequestingappfamilyname.into_param().abi(), blaunchintoimmersiveclient.into_param().abi(), bstrimmersiveclientactivationcontext.into_param().abi(), psaparams).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClaimsToken2<P0, P1>(&self, bstrclaimshint: P0, bstruserhint: P1, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClaimsToken2)(::windows_core::Interface::as_raw(self), bstrclaimshint.into_param().abi(), bstruserhint.into_param().abi(), claimcookie, hwndcreduiparent, ::core::mem::transmute(rectcreduiparent), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClaimsToken<P0, P1>(&self, bstraccesstoken: P0, ullaccesstokenexpiration: u64, bstrrefreshtoken: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClaimsToken)(::windows_core::Interface::as_raw(self), bstraccesstoken.into_param().abi(), ullaccesstokenexpiration, bstrrefreshtoken.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspace3, ::windows_core::IUnknown, IWorkspace, IWorkspace2);
unsafe impl ::windows_core::Interface for IWorkspace3 {
    type Vtable = IWorkspace3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspace3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1becbe4a_d654_423b_afeb_be8d532c13c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace3_Vtbl {
    pub base__: IWorkspace2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClaimsToken2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclaimshint: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserhint: ::std::mem::MaybeUninit<::windows_core::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClaimsToken2: usize,
    pub SetClaimsToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccesstoken: ::std::mem::MaybeUninit<::windows_core::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceClientExt(::windows_core::IUnknown);
impl IWorkspaceClientExt {
    pub unsafe fn GetResourceId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResourceId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResourceDisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResourceDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IssueDisconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IssueDisconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspaceClientExt, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspaceClientExt {
    type Vtable = IWorkspaceClientExt_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspaceClientExt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12b952f4_41ca_4f21_a829_a6d07d9a16e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceClientExt_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetResourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IssueDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceRegistration(::windows_core::IUnknown);
impl IWorkspaceRegistration {
    pub unsafe fn AddResource<P0>(&self, punk: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IWorkspaceClientExt>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddResource)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveResource)(::windows_core::Interface::as_raw(self), dwcookieconnection).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspaceRegistration, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspaceRegistration {
    type Vtable = IWorkspaceRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspaceRegistration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceRegistration2(::windows_core::IUnknown);
impl IWorkspaceRegistration2 {
    pub unsafe fn AddResource<P0>(&self, punk: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IWorkspaceClientExt>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddResource)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveResource)(::windows_core::Interface::as_raw(self), dwcookieconnection).ok()
    }
    pub unsafe fn AddResourceEx<P0, P1>(&self, punk: P0, bstreventloguploadaddress: P1, pdwcookie: *mut u32, correlationid: ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWorkspaceClientExt>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddResourceEx)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), bstreventloguploadaddress.into_param().abi(), pdwcookie, ::core::mem::transmute(correlationid)).ok()
    }
    pub unsafe fn RemoveResourceEx(&self, dwcookieconnection: u32, correlationid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveResourceEx)(::windows_core::Interface::as_raw(self), dwcookieconnection, ::core::mem::transmute(correlationid)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspaceRegistration2, ::windows_core::IUnknown, IWorkspaceRegistration);
unsafe impl ::windows_core::Interface for IWorkspaceRegistration2 {
    type Vtable = IWorkspaceRegistration2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspaceRegistration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf59f654_39bb_44d8_94d0_4635728957e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration2_Vtbl {
    pub base__: IWorkspaceRegistration_Vtbl,
    pub AddResourceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, bstreventloguploadaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcookie: *mut u32, correlationid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RemoveResourceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceReportMessage(::windows_core::IUnknown);
impl IWorkspaceReportMessage {
    pub unsafe fn RegisterErrorLogMessage<P0>(&self, bstrmessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterErrorLogMessage)(::windows_core::Interface::as_raw(self), bstrmessage.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsErrorMessageRegistered<P0, P1>(&self, bstrwkspid: P0, dwerrortype: u32, bstrerrormessagetype: P1, dwerrorcode: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsErrorMessageRegistered)(::windows_core::Interface::as_raw(self), bstrwkspid.into_param().abi(), dwerrortype, bstrerrormessagetype.into_param().abi(), dwerrorcode, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterErrorEvent<P0, P1>(&self, bstrwkspid: P0, dwerrortype: u32, bstrerrormessagetype: P1, dwerrorcode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterErrorEvent)(::windows_core::Interface::as_raw(self), bstrwkspid.into_param().abi(), dwerrortype, bstrerrormessagetype.into_param().abi(), dwerrorcode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWorkspaceReportMessage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWorkspaceReportMessage {
    type Vtable = IWorkspaceReportMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWorkspaceReportMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7c06739_500f_4e8c_99a8_2bd6955899eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceReportMessage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterErrorLogMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsErrorMessageRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwkspid: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrorcode: u32, pferrorexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsErrorMessageRegistered: usize,
    pub RegisterErrorEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwkspid: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrorcode: u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceResTypeRegistry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceResTypeRegistry {
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddResourceType<P0, P1, P2>(&self, fmachinewide: P0, bstrfileextension: P1, bstrlauncher: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddResourceType)(::windows_core::Interface::as_raw(self), fmachinewide.into_param().abi(), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteResourceType<P0, P1>(&self, fmachinewide: P0, bstrfileextension: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteResourceType)(::windows_core::Interface::as_raw(self), fmachinewide.into_param().abi(), bstrfileextension.into_param().abi()).ok()
    }
    #[doc = "Required features: `Win32_Foundation`, `Win32_System_Com`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRegisteredFileExtensions<P0>(&self, fmachinewide: P0) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRegisteredFileExtensions)(::windows_core::Interface::as_raw(self), fmachinewide.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceTypeInfo<P0, P1>(&self, fmachinewide: P0, bstrfileextension: P1) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetResourceTypeInfo)(::windows_core::Interface::as_raw(self), fmachinewide.into_param().abi(), bstrfileextension.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModifyResourceType<P0, P1, P2>(&self, fmachinewide: P0, bstrfileextension: P1, bstrlauncher: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ModifyResourceType)(::windows_core::Interface::as_raw(self), fmachinewide.into_param().abi(), bstrfileextension.into_param().abi(), bstrlauncher.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWorkspaceResTypeRegistry, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWorkspaceResTypeRegistry {
    type Vtable = IWorkspaceResTypeRegistry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWorkspaceResTypeRegistry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d428c79_6e2e_4351_a361_c0401a03a0ba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceResTypeRegistry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlauncher: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddResourceType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteResourceType: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRegisteredFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRegisteredFileExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResourceTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrlauncher: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResourceTypeInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModifyResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlauncher: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModifyResourceType: usize,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceScriptable(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable {
    pub unsafe fn DisconnectWorkspace<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DisconnectWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspace<P0, P1, P2, P3>(&self, bstrworkspaceid: P0, bstrusername: P1, bstrpassword: P2, bstrworkspaceparams: P3, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StartWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<P0, P1>(&self, bstrworkspaceid: P0, bcountunauthenticatedcredentials: P1) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsWorkspaceCredentialSpecified)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bcountunauthenticatedcredentials.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsWorkspaceSSOEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearWorkspaceCredential<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ClearWorkspaceCredential)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn OnAuthenticated<P0, P1>(&self, bstrworkspaceid: P0, bstrusername: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OnAuthenticated)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName<P0>(&self, bstrworkspacefriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DisconnectWorkspaceByFriendlyName)(::windows_core::Interface::as_raw(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWorkspaceScriptable, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWorkspaceScriptable {
    type Vtable = IWorkspaceScriptable_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWorkspaceScriptable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefea49a2_dda5_429d_8f42_b23b92c4c347);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisconnectWorkspace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StartWorkspace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkspaceCredentialSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL, pbcredexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkspaceCredentialSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkspaceSSOEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbssoenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkspaceSSOEnabled: usize,
    pub ClearWorkspaceCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OnAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DisconnectWorkspaceByFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceScriptable2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2 {
    pub unsafe fn DisconnectWorkspace<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.DisconnectWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspace<P0, P1, P2, P3>(&self, bstrworkspaceid: P0, bstrusername: P1, bstrpassword: P2, bstrworkspaceparams: P3, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.StartWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<P0, P1>(&self, bstrworkspaceid: P0, bcountunauthenticatedcredentials: P1) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsWorkspaceCredentialSpecified)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bcountunauthenticatedcredentials.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsWorkspaceSSOEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearWorkspaceCredential<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ClearWorkspaceCredential)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn OnAuthenticated<P0, P1>(&self, bstrworkspaceid: P0, bstrusername: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.OnAuthenticated)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName<P0>(&self, bstrworkspacefriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.DisconnectWorkspaceByFriendlyName)(::windows_core::Interface::as_raw(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspaceEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1, bstrredirectorname: P2, bstrusername: P3, bstrpassword: P4, bstrappcontainer: P5, bstrworkspaceparams: P6, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
        P5: ::windows_core::IntoParam<::windows_core::BSTR>,
        P6: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StartWorkspaceEx)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags).ok()
    }
    pub unsafe fn ResourceDismissed<P0, P1>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ResourceDismissed)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWorkspaceScriptable2, ::windows_core::IUnknown, super::Com::IDispatch, IWorkspaceScriptable);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWorkspaceScriptable2 {
    type Vtable = IWorkspaceScriptable2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWorkspaceScriptable2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefea49a2_dda5_429d_8f42_b33ba2c4c348);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable2_Vtbl {
    pub base__: IWorkspaceScriptable_Vtbl,
    pub StartWorkspaceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrredirectorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrappcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_core::HRESULT,
    pub ResourceDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWorkspaceScriptable3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3 {
    pub unsafe fn DisconnectWorkspace<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DisconnectWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspace<P0, P1, P2, P3>(&self, bstrworkspaceid: P0, bstrusername: P1, bstrpassword: P2, bstrworkspaceparams: P3, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.StartWorkspace)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags).ok()
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<P0, P1>(&self, bstrworkspaceid: P0, bcountunauthenticatedcredentials: P1) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsWorkspaceCredentialSpecified)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bcountunauthenticatedcredentials.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `Win32_Foundation`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsWorkspaceSSOEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearWorkspaceCredential<P0>(&self, bstrworkspaceid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ClearWorkspaceCredential)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi()).ok()
    }
    pub unsafe fn OnAuthenticated<P0, P1>(&self, bstrworkspaceid: P0, bstrusername: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.OnAuthenticated)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName<P0>(&self, bstrworkspacefriendlyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.DisconnectWorkspaceByFriendlyName)(::windows_core::Interface::as_raw(self), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspaceEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1, bstrredirectorname: P2, bstrusername: P3, bstrpassword: P4, bstrappcontainer: P5, bstrworkspaceparams: P6, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
        P5: ::windows_core::IntoParam<::windows_core::BSTR>,
        P6: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.StartWorkspaceEx)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags).ok()
    }
    pub unsafe fn ResourceDismissed<P0, P1>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ResourceDismissed)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn StartWorkspaceEx2<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1, bstrredirectorname: P2, bstrusername: P3, bstrpassword: P4, bstrappcontainer: P5, bstrworkspaceparams: P6, ltimeout: i32, lflags: i32, bstreventloguploadaddress: P7, correlationid: ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
        P5: ::windows_core::IntoParam<::windows_core::BSTR>,
        P6: ::windows_core::IntoParam<::windows_core::BSTR>,
        P7: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).StartWorkspaceEx2)(::windows_core::Interface::as_raw(self), bstrworkspaceid.into_param().abi(), bstrworkspacefriendlyname.into_param().abi(), bstrredirectorname.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), bstrappcontainer.into_param().abi(), bstrworkspaceparams.into_param().abi(), ltimeout, lflags, bstreventloguploadaddress.into_param().abi(), ::core::mem::transmute(correlationid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IWorkspaceScriptable3, ::windows_core::IUnknown, super::Com::IDispatch, IWorkspaceScriptable, IWorkspaceScriptable2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWorkspaceScriptable3 {
    type Vtable = IWorkspaceScriptable3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IWorkspaceScriptable3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x531e6512_2cbf_4bd2_80a5_d90a71636a9a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable3_Vtbl {
    pub base__: IWorkspaceScriptable2_Vtbl,
    pub StartWorkspaceEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrredirectorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrappcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltimeout: i32, lflags: i32, bstreventloguploadaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, correlationid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ItsPubPlugin(::windows_core::IUnknown);
impl ItsPubPlugin {
    pub unsafe fn GetResourceList<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetResourceList)(::windows_core::Interface::as_raw(self), userid.into_param().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetResource)(::windows_core::Interface::as_raw(self), alias.into_param().abi(), flags, resource).ok()
    }
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCacheLastUpdateTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn pluginName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).pluginName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn pluginVersion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).pluginVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ResolveResource<P0, P1>(&self, resourcetype: *mut u32, resourcelocation: &mut [u16; 256], endpointname: &mut [u16; 256], userid: P0, alias: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ResolveResource)(::windows_core::Interface::as_raw(self), resourcetype, ::core::mem::transmute(resourcelocation.as_ptr()), ::core::mem::transmute(endpointname.as_ptr()), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ItsPubPlugin, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ItsPubPlugin {
    type Vtable = ItsPubPlugin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItsPubPlugin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70c04b05_f347_412b_822f_36c99c54ca45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetResourceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alias: ::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> ::windows_core::HRESULT,
    pub GetCacheLastUpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows_core::HRESULT,
    pub pluginName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub pluginVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ResolveResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: ::windows_core::PWSTR, endpointname: ::windows_core::PWSTR, userid: ::windows_core::PCWSTR, alias: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ItsPubPlugin2(::windows_core::IUnknown);
impl ItsPubPlugin2 {
    pub unsafe fn GetResourceList<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.GetResourceList)(::windows_core::Interface::as_raw(self), userid.into_param().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.GetResource)(::windows_core::Interface::as_raw(self), alias.into_param().abi(), flags, resource).ok()
    }
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCacheLastUpdateTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn pluginName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.pluginName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn pluginVersion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.pluginVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ResolveResource<P0, P1>(&self, resourcetype: *mut u32, resourcelocation: &mut [u16; 256], endpointname: &mut [u16; 256], userid: P0, alias: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ResolveResource)(::windows_core::Interface::as_raw(self), resourcetype, ::core::mem::transmute(resourcelocation.as_ptr()), ::core::mem::transmute(endpointname.as_ptr()), userid.into_param().abi(), alias.into_param().abi()).ok()
    }
    pub unsafe fn GetResource2List<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetResource2List)(::windows_core::Interface::as_raw(self), userid.into_param().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource2<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetResource2)(::windows_core::Interface::as_raw(self), alias.into_param().abi(), flags, resource).ok()
    }
    pub unsafe fn ResolvePersonalDesktop<P0, P1>(&self, userid: P0, poolid: P1, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: &mut [u16; 256]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ResolvePersonalDesktop)(::windows_core::Interface::as_raw(self), userid.into_param().abi(), poolid.into_param().abi(), epdresolutiontype, ppdassignmenttype, ::core::mem::transmute(endpointname.as_ptr())).ok()
    }
    pub unsafe fn DeletePersonalDesktopAssignment<P0, P1, P2>(&self, userid: P0, poolid: P1, endpointname: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeletePersonalDesktopAssignment)(::windows_core::Interface::as_raw(self), userid.into_param().abi(), poolid.into_param().abi(), endpointname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ItsPubPlugin2, ::windows_core::IUnknown, ItsPubPlugin);
unsafe impl ::windows_core::Interface for ItsPubPlugin2 {
    type Vtable = ItsPubPlugin2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItsPubPlugin2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa4ce418_aad7_4ec6_bad1_0a321ba465d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin2_Vtbl {
    pub base__: ItsPubPlugin_Vtbl,
    pub GetResource2List: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows_core::HRESULT,
    pub GetResource2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alias: ::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows_core::HRESULT,
    pub ResolvePersonalDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, poolid: ::windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub DeletePersonalDesktopAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, poolid: ::windows_core::PCWSTR, endpointname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `Win32_System_Com`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct _ITSWkspEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(_ITSWkspEvents, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for _ITSWkspEvents {
    type Vtable = _ITSWkspEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for _ITSWkspEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb922bbb8_4c55_4fea_8496_beb0b44285e9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ITSWkspEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
pub const AA_AUTH_ANY: AAAuthSchemes = AAAuthSchemes(6i32);
pub const AA_AUTH_BASIC: AAAuthSchemes = AAAuthSchemes(1i32);
pub const AA_AUTH_CONID: AAAuthSchemes = AAAuthSchemes(10i32);
pub const AA_AUTH_COOKIE: AAAuthSchemes = AAAuthSchemes(7i32);
pub const AA_AUTH_DIGEST: AAAuthSchemes = AAAuthSchemes(8i32);
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = AAAuthSchemes(4i32);
pub const AA_AUTH_MAX: AAAuthSchemes = AAAuthSchemes(12i32);
pub const AA_AUTH_MIN: AAAuthSchemes = AAAuthSchemes(0i32);
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = AAAuthSchemes(5i32);
pub const AA_AUTH_NTLM: AAAuthSchemes = AAAuthSchemes(2i32);
pub const AA_AUTH_ORGID: AAAuthSchemes = AAAuthSchemes(9i32);
pub const AA_AUTH_SC: AAAuthSchemes = AAAuthSchemes(3i32);
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = AAAuthSchemes(11i32);
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(3i32);
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(0i32);
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(2i32);
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(1i32);
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = AATrustClassID(2i32);
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = AATrustClassID(1i32);
pub const AA_UNTRUSTED: AATrustClassID = AATrustClassID(0i32);
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2e9cae6_1e7b_4b8e_babd_e9bf6292ac29);
pub const AllowOnlySDRServers: PolicyAttributeType = PolicyAttributeType(7i32);
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
pub const CHANNEL_NAME_LEN: u32 = 7u32;
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
pub const CHANNEL_RC_OK: u32 = 0u32;
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const CLIENTNAME_LENGTH: u32 = 20u32;
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(2i32);
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(0i32);
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(1i32);
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b150580_fea4_4d3c_9de4_7433a66618f7);
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x693f7ff5_0c4e_4d17_b8e0_1f70325e5d58);
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(5i32);
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(2i32);
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(0i32);
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(6i32);
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(8i32);
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(1i32);
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(7i32);
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(4i32);
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(3i32);
pub const ClipboardRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(5i32);
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
pub const DISPID_AX_CONNECTED: u32 = 751u32;
pub const DISPID_AX_CONNECTING: u32 = 750u32;
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
pub const DOMAIN_LENGTH: u32 = 17u32;
pub const DisableAllRedirections: PolicyAttributeType = PolicyAttributeType(1i32);
pub const DriveRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(2i32);
pub const EnableAllRedirections: PolicyAttributeType = PolicyAttributeType(0i32);
pub const FARM: TARGET_TYPE = TARGET_TYPE(1i32);
pub const FORCE_REJOIN: u32 = 2u32;
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub const KeyCombinationDown: KeyCombinationType = KeyCombinationType(4i32);
pub const KeyCombinationHome: KeyCombinationType = KeyCombinationType(0i32);
pub const KeyCombinationLeft: KeyCombinationType = KeyCombinationType(1i32);
pub const KeyCombinationRight: KeyCombinationType = KeyCombinationType(3i32);
pub const KeyCombinationScroll: KeyCombinationType = KeyCombinationType(5i32);
pub const KeyCombinationUp: KeyCombinationType = KeyCombinationType(2i32);
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(4i32);
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
pub const MaxAppName_Len: u32 = 256u32;
pub const MaxDomainName_Len: u32 = 256u32;
pub const MaxFQDN_Len: u32 = 256u32;
pub const MaxFarm_Len: u32 = 256u32;
pub const MaxNetBiosName_Len: u32 = 16u32;
pub const MaxNumOfExposed_IPs: u32 = 12u32;
pub const MaxUserName_Len: u32 = 104u32;
pub const NONFARM: TARGET_TYPE = TARGET_TYPE(2i32);
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(16i32);
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = TARGET_OWNER(1i32);
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = TARGET_OWNER(2i32);
pub const OWNER_UNKNOWN: TARGET_OWNER = TARGET_OWNER(0i32);
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(8i32);
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
pub const POLICY_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(1i32);
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(2i32);
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(1i32);
pub const POSITION_INVALID: AE_POSITION_FLAGS = AE_POSITION_FLAGS(0i32);
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = AE_POSITION_FLAGS(4i32);
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cdfd28e_d0b9_4c1f_a5eb_6d1f6c6535b9);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed2c3fda_338d_4d3f_81a3_e767310d908e);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6212d757_0043_4862_99c3_9f3059ac2a3b);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x197c427a_0135_4b6d_9c5e_e6579a0ab625);
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(32i32);
pub const PasswordEncodingUTF16BE: PasswordEncodingType = PasswordEncodingType(2i32);
pub const PasswordEncodingUTF16LE: PasswordEncodingType = PasswordEncodingType(1i32);
pub const PasswordEncodingUTF8: PasswordEncodingType = PasswordEncodingType(0i32);
pub const PnpRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(6i32);
pub const PortRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(4i32);
pub const PrinterRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(3i32);
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = RDV_TASK_STATUS(3i32);
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = RDV_TASK_STATUS(2i32);
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = RDV_TASK_STATUS(7i32);
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = RDV_TASK_STATUS(5i32);
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = RDV_TASK_STATUS(4i32);
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = RDV_TASK_STATUS(1i32);
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = RDV_TASK_STATUS(6i32);
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = RDV_TASK_STATUS(8i32);
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = RDV_TASK_STATUS(0i32);
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(5i32);
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(3i32);
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(4i32);
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(2i32);
pub const RD_FARM_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(0i32);
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = RD_FARM_TYPE(1i32);
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = RD_FARM_TYPE(-1i32);
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
pub const RENDER_HINT_CLEAR: u32 = 0u32;
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
pub const RENDER_HINT_VIDEO: u32 = 1u32;
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(2i32);
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
pub const RemoteActionAppSwitch: RemoteActionType = RemoteActionType(4i32);
pub const RemoteActionAppbar: RemoteActionType = RemoteActionType(1i32);
pub const RemoteActionCharms: RemoteActionType = RemoteActionType(0i32);
pub const RemoteActionSnap: RemoteActionType = RemoteActionType(2i32);
pub const RemoteActionStartScreen: RemoteActionType = RemoteActionType(3i32);
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(0i32);
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(1i32);
pub const SINGLE_SESSION: u32 = 1u32;
pub const STATE_ACTIVE: TSSESSION_STATE = TSSESSION_STATE(0i32);
pub const STATE_CONNECTED: TSSESSION_STATE = TSSESSION_STATE(1i32);
pub const STATE_CONNECTQUERY: TSSESSION_STATE = TSSESSION_STATE(2i32);
pub const STATE_DISCONNECTED: TSSESSION_STATE = TSSESSION_STATE(4i32);
pub const STATE_DOWN: TSSESSION_STATE = TSSESSION_STATE(8i32);
pub const STATE_IDLE: TSSESSION_STATE = TSSESSION_STATE(5i32);
pub const STATE_INIT: TSSESSION_STATE = TSSESSION_STATE(9i32);
pub const STATE_INVALID: TSSESSION_STATE = TSSESSION_STATE(-1i32);
pub const STATE_LISTEN: TSSESSION_STATE = TSSESSION_STATE(6i32);
pub const STATE_MAX: TSSESSION_STATE = TSSESSION_STATE(10i32);
pub const STATE_RESET: TSSESSION_STATE = TSSESSION_STATE(7i32);
pub const STATE_SHADOW: TSSESSION_STATE = TSSESSION_STATE(3i32);
pub const SnapshotEncodingDataUri: SnapshotEncodingType = SnapshotEncodingType(0i32);
pub const SnapshotFormatBmp: SnapshotFormatType = SnapshotFormatType(2i32);
pub const SnapshotFormatJpeg: SnapshotFormatType = SnapshotFormatType(1i32);
pub const SnapshotFormatPng: SnapshotFormatType = SnapshotFormatType(0i32);
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1i32);
pub const TARGET_CHECKED_OUT: TARGET_STATE = TARGET_STATE(6i32);
pub const TARGET_DOWN: TARGET_STATE = TARGET_STATE(4i32);
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(2i32);
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1024i32);
pub const TARGET_HIBERNATED: TARGET_STATE = TARGET_STATE(5i32);
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(64i32);
pub const TARGET_INITIALIZING: TARGET_STATE = TARGET_STATE(2i32);
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(4i32);
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(256i32);
pub const TARGET_INVALID: TARGET_STATE = TARGET_STATE(8i32);
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(8i32);
pub const TARGET_MAXSTATE: TARGET_STATE = TARGET_STATE(11i32);
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(3i32);
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(4i32);
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = TARGET_PATCH_STATE(2i32);
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(1i32);
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(512i32);
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = TARGET_PATCH_STATE(0i32);
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(128i32);
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(16i32);
pub const TARGET_RUNNING: TARGET_STATE = TARGET_STATE(3i32);
pub const TARGET_STARTING: TARGET_STATE = TARGET_STATE(9i32);
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(32i32);
pub const TARGET_STOPPED: TARGET_STATE = TARGET_STATE(7i32);
pub const TARGET_STOPPING: TARGET_STATE = TARGET_STATE(10i32);
pub const TARGET_UNKNOWN: TARGET_STATE = TARGET_STATE(1i32);
pub const TASK_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(64i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(0i32);
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(0i32);
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(4i32);
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(0i32);
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(2i32);
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(1i32);
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = TSSD_AddrV46Type(4i32);
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = TSSD_AddrV46Type(6i32);
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = TSSD_AddrV46Type(0i32);
pub const TSUserExInterfaces: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0910dd01_df8c_11d1_ae27_00c04fa35813);
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = TS_SB_SORT_BY(1i32);
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = TS_SB_SORT_BY(0i32);
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = TS_SB_SORT_BY(2i32);
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
pub const UNKNOWN: TARGET_TYPE = TARGET_TYPE(0i32);
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(0i32);
pub const USERNAME_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(2i32);
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(3i32);
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(1i32);
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(0i32);
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(4i32);
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(2i32);
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(3i32);
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(1i32);
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(0i32);
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(1i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(0i32);
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(1i32);
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(0i32);
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WRDS_MAX_RESERVED: u32 = 100u32;
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2993f4d_02cf_4280_8c48_1624b44f8706);
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(1i32);
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(0i32);
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(0i32);
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(1i32);
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(-1i32);
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(2i32);
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(0i32);
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(1i32);
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(3i32);
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(2i32);
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(1i32);
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(0i32);
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(0i32);
pub const WTSApplicationName: WTS_INFO_CLASS = WTS_INFO_CLASS(1i32);
pub const WTSClientAddress: WTS_INFO_CLASS = WTS_INFO_CLASS(14i32);
pub const WTSClientBuildNumber: WTS_INFO_CLASS = WTS_INFO_CLASS(9i32);
pub const WTSClientDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(11i32);
pub const WTSClientDisplay: WTS_INFO_CLASS = WTS_INFO_CLASS(15i32);
pub const WTSClientHardwareId: WTS_INFO_CLASS = WTS_INFO_CLASS(13i32);
pub const WTSClientInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(23i32);
pub const WTSClientName: WTS_INFO_CLASS = WTS_INFO_CLASS(10i32);
pub const WTSClientProductId: WTS_INFO_CLASS = WTS_INFO_CLASS(12i32);
pub const WTSClientProtocolType: WTS_INFO_CLASS = WTS_INFO_CLASS(16i32);
pub const WTSConfigInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(26i32);
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(2i32);
pub const WTSConnectState: WTS_INFO_CLASS = WTS_INFO_CLASS(8i32);
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(1i32);
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(4i32);
pub const WTSDomainName: WTS_INFO_CLASS = WTS_INFO_CLASS(7i32);
pub const WTSDown: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(8i32);
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(5i32);
pub const WTSIdleTime: WTS_INFO_CLASS = WTS_INFO_CLASS(17i32);
pub const WTSIncomingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(19i32);
pub const WTSIncomingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(21i32);
pub const WTSInit: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(9i32);
pub const WTSInitialProgram: WTS_INFO_CLASS = WTS_INFO_CLASS(0i32);
pub const WTSIsRemoteSession: WTS_INFO_CLASS = WTS_INFO_CLASS(29i32);
pub const WTSListen: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(6i32);
pub const WTSLogonTime: WTS_INFO_CLASS = WTS_INFO_CLASS(18i32);
pub const WTSOEMId: WTS_INFO_CLASS = WTS_INFO_CLASS(3i32);
pub const WTSOutgoingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(20i32);
pub const WTSOutgoingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(22i32);
pub const WTSReset: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(7i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(1i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(2i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(3i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(4i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(0i32);
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(1i32);
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(2i32);
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(0i32);
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(2i32);
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(1i32);
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(0i32);
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(1i32);
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(2i32);
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(0i32);
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(4i32);
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(2i32);
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(1i32);
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(8i32);
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(1i32);
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(2i32);
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(0i32);
pub const WTSSessionAddressV4: WTS_INFO_CLASS = WTS_INFO_CLASS(28i32);
pub const WTSSessionId: WTS_INFO_CLASS = WTS_INFO_CLASS(4i32);
pub const WTSSessionInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(24i32);
pub const WTSSessionInfoEx: WTS_INFO_CLASS = WTS_INFO_CLASS(25i32);
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(3i32);
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = WTS_TYPE_CLASS(0i32);
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(1i32);
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(2i32);
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(10i32);
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(0i32);
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(13i32);
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(12i32);
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(11i32);
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(14i32);
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = WTS_CONFIG_SOURCE(0i32);
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(16i32);
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(17i32);
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(15i32);
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(4i32);
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(5i32);
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(6i32);
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(19i32);
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(1i32);
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(3i32);
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(9i32);
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(7i32);
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(8i32);
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(2i32);
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(18i32);
pub const WTSUserName: WTS_INFO_CLASS = WTS_INFO_CLASS(5i32);
pub const WTSValidationInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(27i32);
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(0i32);
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(1i32);
pub const WTSWinStationName: WTS_INFO_CLASS = WTS_INFO_CLASS(6i32);
pub const WTSWorkingDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(2i32);
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = WTS_CERT_TYPE(0i32);
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = WTS_CERT_TYPE(1i32);
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = WTS_CERT_TYPE(2i32);
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub const WTS_CURRENT_SERVER: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub const WTS_CURRENT_SERVER_HANDLE: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
pub const WTS_CURRENT_SERVER_NAME: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(1i32);
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(2i32);
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(0i32);
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
pub const WTS_EVENT_CONNECT: u32 = 8u32;
pub const WTS_EVENT_CREATE: u32 = 1u32;
pub const WTS_EVENT_DELETE: u32 = 2u32;
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
pub const WTS_EVENT_LICENSE: u32 = 256u32;
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
pub const WTS_EVENT_LOGON: u32 = 32u32;
pub const WTS_EVENT_NONE: u32 = 0u32;
pub const WTS_EVENT_RENAME: u32 = 4u32;
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub const WTS_LISTENER_CREATE: u32 = 1u32;
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(3i32);
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(4i32);
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(2i32);
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(0i32);
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(1i32);
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WTS_MAX_COUNTERS: u32 = 100u32;
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WTS_MAX_RESERVED: u32 = 100u32;
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
pub const WTS_PROPERTY_DEFAULT_CONFIG: ::windows_core::PCWSTR = ::windows_core::w!("DefaultConfig");
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc77d1b30_5be1_4c6b_a0e1_bd6d2e5c9fcc);
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bf4fa97_c883_4c2a_80ab_5a39c9af00db);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b8e0fe7_0804_4a0e_b279_8660b1df0049);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41869ad0_6332_4dc8_95d5_db749e2f1d94);
pub const WTS_SECURITY_ALL_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(983999u32);
pub const WTS_SECURITY_CONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(256u32);
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(72u32);
pub const WTS_SECURITY_CURRENT_USER_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(590u32);
pub const WTS_SECURITY_DISCONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(512u32);
pub const WTS_SECURITY_GUEST_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(32u32);
pub const WTS_SECURITY_LOGOFF: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(64u32);
pub const WTS_SECURITY_LOGON: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(32u32);
pub const WTS_SECURITY_MESSAGE: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(128u32);
pub const WTS_SECURITY_QUERY_INFORMATION: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(1u32);
pub const WTS_SECURITY_REMOTE_CONTROL: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(16u32);
pub const WTS_SECURITY_RESET: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(4u32);
pub const WTS_SECURITY_SET_INFORMATION: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(2u32);
pub const WTS_SECURITY_USER_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(329u32);
pub const WTS_SECURITY_VIRTUAL_CHANNELS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(8u32);
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(0i32);
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(1i32);
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(2i32);
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
pub const WTS_WSD_LOGOFF: u32 = 1u32;
pub const WTS_WSD_POWEROFF: u32 = 8u32;
pub const WTS_WSD_REBOOT: u32 = 4u32;
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f1dfca6_3aad_48e1_8406_4bc21a501d7c);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AAAccountingDataType(pub i32);
impl ::core::marker::Copy for AAAccountingDataType {}
impl ::core::clone::Clone for AAAccountingDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AAAccountingDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AAAccountingDataType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AAAccountingDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAccountingDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AAAuthSchemes(pub i32);
impl ::core::marker::Copy for AAAuthSchemes {}
impl ::core::clone::Clone for AAAuthSchemes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AAAuthSchemes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AAAuthSchemes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AAAuthSchemes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAuthSchemes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AATrustClassID(pub i32);
impl ::core::marker::Copy for AATrustClassID {}
impl ::core::clone::Clone for AATrustClassID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AATrustClassID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AATrustClassID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AATrustClassID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AATrustClassID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AE_POSITION_FLAGS(pub i32);
impl ::core::marker::Copy for AE_POSITION_FLAGS {}
impl ::core::clone::Clone for AE_POSITION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AE_POSITION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AE_POSITION_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AE_POSITION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AE_POSITION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLIENT_MESSAGE_TYPE(pub i32);
impl ::core::marker::Copy for CLIENT_MESSAGE_TYPE {}
impl ::core::clone::Clone for CLIENT_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLIENT_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CLIENT_MESSAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CLIENT_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIENT_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONNECTION_CHANGE_NOTIFICATION(pub i32);
impl ::core::marker::Copy for CONNECTION_CHANGE_NOTIFICATION {}
impl ::core::clone::Clone for CONNECTION_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONNECTION_CHANGE_NOTIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CONNECTION_CHANGE_NOTIFICATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CONNECTION_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONNECTION_CHANGE_NOTIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCombinationType(pub i32);
impl ::core::marker::Copy for KeyCombinationType {}
impl ::core::clone::Clone for KeyCombinationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCombinationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KeyCombinationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyCombinationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCombinationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PLUGIN_TYPE(pub i32);
impl ::core::marker::Copy for PLUGIN_TYPE {}
impl ::core::clone::Clone for PLUGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PLUGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PLUGIN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PLUGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLUGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PasswordEncodingType(pub i32);
impl ::core::marker::Copy for PasswordEncodingType {}
impl ::core::clone::Clone for PasswordEncodingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PasswordEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PasswordEncodingType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PasswordEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordEncodingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PolicyAttributeType(pub i32);
impl ::core::marker::Copy for PolicyAttributeType {}
impl ::core::clone::Clone for PolicyAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PolicyAttributeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PolicyAttributeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PolicyAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyAttributeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDV_TASK_STATUS(pub i32);
impl ::core::marker::Copy for RDV_TASK_STATUS {}
impl ::core::clone::Clone for RDV_TASK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDV_TASK_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RDV_TASK_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RDV_TASK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDV_TASK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RD_FARM_TYPE(pub i32);
impl ::core::marker::Copy for RD_FARM_TYPE {}
impl ::core::clone::Clone for RD_FARM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RD_FARM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RD_FARM_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RD_FARM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RD_FARM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteActionType(pub i32);
impl ::core::marker::Copy for RemoteActionType {}
impl ::core::clone::Clone for RemoteActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteActionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RemoteActionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RemoteActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteActionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SESSION_TIMEOUT_ACTION_TYPE(pub i32);
impl ::core::marker::Copy for SESSION_TIMEOUT_ACTION_TYPE {}
impl ::core::clone::Clone for SESSION_TIMEOUT_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SESSION_TIMEOUT_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SESSION_TIMEOUT_ACTION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SESSION_TIMEOUT_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_TIMEOUT_ACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SnapshotEncodingType(pub i32);
impl ::core::marker::Copy for SnapshotEncodingType {}
impl ::core::clone::Clone for SnapshotEncodingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapshotEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SnapshotEncodingType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SnapshotEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotEncodingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SnapshotFormatType(pub i32);
impl ::core::marker::Copy for SnapshotFormatType {}
impl ::core::clone::Clone for SnapshotFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapshotFormatType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SnapshotFormatType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SnapshotFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotFormatType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_CHANGE_TYPE(pub i32);
impl ::core::marker::Copy for TARGET_CHANGE_TYPE {}
impl ::core::clone::Clone for TARGET_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TARGET_CHANGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TARGET_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_OWNER(pub i32);
impl ::core::marker::Copy for TARGET_OWNER {}
impl ::core::clone::Clone for TARGET_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_OWNER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TARGET_OWNER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TARGET_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_OWNER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_PATCH_STATE(pub i32);
impl ::core::marker::Copy for TARGET_PATCH_STATE {}
impl ::core::clone::Clone for TARGET_PATCH_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_PATCH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TARGET_PATCH_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TARGET_PATCH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_PATCH_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_STATE(pub i32);
impl ::core::marker::Copy for TARGET_STATE {}
impl ::core::clone::Clone for TARGET_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TARGET_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TARGET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_TYPE(pub i32);
impl ::core::marker::Copy for TARGET_TYPE {}
impl ::core::clone::Clone for TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TARGET_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(pub i32);
impl ::core::marker::Copy for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {}
impl ::core::clone::Clone for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(pub i32);
impl ::core::marker::Copy for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {}
impl ::core::clone::Clone for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_RESOLUTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TSSB_NOTIFICATION_TYPE(pub i32);
impl ::core::marker::Copy for TSSB_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for TSSB_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSB_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TSSB_NOTIFICATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TSSB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TSSD_AddrV46Type(pub i32);
impl ::core::marker::Copy for TSSD_AddrV46Type {}
impl ::core::clone::Clone for TSSD_AddrV46Type {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSD_AddrV46Type {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TSSD_AddrV46Type {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TSSD_AddrV46Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSD_AddrV46Type").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TSSESSION_STATE(pub i32);
impl ::core::marker::Copy for TSSESSION_STATE {}
impl ::core::clone::Clone for TSSESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TSSESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TSSESSION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TSSESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TS_SB_SORT_BY(pub i32);
impl ::core::marker::Copy for TS_SB_SORT_BY {}
impl ::core::clone::Clone for TS_SB_SORT_BY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TS_SB_SORT_BY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TS_SB_SORT_BY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TS_SB_SORT_BY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TS_SB_SORT_BY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VM_HOST_NOTIFY_STATUS(pub i32);
impl ::core::marker::Copy for VM_HOST_NOTIFY_STATUS {}
impl ::core::clone::Clone for VM_HOST_NOTIFY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VM_HOST_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VM_HOST_NOTIFY_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VM_HOST_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_HOST_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VM_NOTIFY_STATUS(pub i32);
impl ::core::marker::Copy for VM_NOTIFY_STATUS {}
impl ::core::clone::Clone for VM_NOTIFY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VM_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VM_NOTIFY_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VM_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRDS_CONNECTION_SETTING_LEVEL(pub i32);
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_CONNECTION_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRDS_CONNECTION_SETTING_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRDS_CONNECTION_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_CONNECTION_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRDS_LISTENER_SETTING_LEVEL(pub i32);
impl ::core::marker::Copy for WRDS_LISTENER_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRDS_LISTENER_SETTING_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_LISTENER_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRDS_SETTING_LEVEL(pub i32);
impl ::core::marker::Copy for WRDS_SETTING_LEVEL {}
impl ::core::clone::Clone for WRDS_SETTING_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRDS_SETTING_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRDS_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRDS_SETTING_STATUS(pub i32);
impl ::core::marker::Copy for WRDS_SETTING_STATUS {}
impl ::core::clone::Clone for WRDS_SETTING_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRDS_SETTING_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRDS_SETTING_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRDS_SETTING_TYPE(pub i32);
impl ::core::marker::Copy for WRDS_SETTING_TYPE {}
impl ::core::clone::Clone for WRDS_SETTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRDS_SETTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRDS_SETTING_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRDS_SETTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRdsGraphicsChannelType(pub i32);
impl ::core::marker::Copy for WRdsGraphicsChannelType {}
impl ::core::clone::Clone for WRdsGraphicsChannelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRdsGraphicsChannelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WRdsGraphicsChannelType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WRdsGraphicsChannelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRdsGraphicsChannelType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_ADDRESS_FAMILY(pub i32);
impl ::core::marker::Copy for WTSSBX_ADDRESS_FAMILY {}
impl ::core::clone::Clone for WTSSBX_ADDRESS_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_ADDRESS_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_ADDRESS_FAMILY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_ADDRESS_FAMILY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_MACHINE_DRAIN(pub i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_DRAIN {}
impl ::core::clone::Clone for WTSSBX_MACHINE_DRAIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_DRAIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_MACHINE_DRAIN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_DRAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_DRAIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_MACHINE_SESSION_MODE(pub i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_SESSION_MODE {}
impl ::core::clone::Clone for WTSSBX_MACHINE_SESSION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_SESSION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_MACHINE_SESSION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_SESSION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_SESSION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_MACHINE_STATE(pub i32);
impl ::core::marker::Copy for WTSSBX_MACHINE_STATE {}
impl ::core::clone::Clone for WTSSBX_MACHINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_MACHINE_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_NOTIFICATION_TYPE(pub i32);
impl ::core::marker::Copy for WTSSBX_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for WTSSBX_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_NOTIFICATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTSSBX_SESSION_STATE(pub i32);
impl ::core::marker::Copy for WTSSBX_SESSION_STATE {}
impl ::core::clone::Clone for WTSSBX_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTSSBX_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTSSBX_SESSION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTSSBX_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_CERT_TYPE(pub i32);
impl ::core::marker::Copy for WTS_CERT_TYPE {}
impl ::core::clone::Clone for WTS_CERT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CERT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_CERT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_CERT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CERT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_CONFIG_CLASS(pub i32);
impl ::core::marker::Copy for WTS_CONFIG_CLASS {}
impl ::core::clone::Clone for WTS_CONFIG_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONFIG_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_CONFIG_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_CONFIG_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_CONFIG_SOURCE(pub i32);
impl ::core::marker::Copy for WTS_CONFIG_SOURCE {}
impl ::core::clone::Clone for WTS_CONFIG_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONFIG_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_CONFIG_SOURCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_CONFIG_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_SOURCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_CONNECTSTATE_CLASS(pub i32);
impl ::core::marker::Copy for WTS_CONNECTSTATE_CLASS {}
impl ::core::clone::Clone for WTS_CONNECTSTATE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_CONNECTSTATE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_CONNECTSTATE_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_CONNECTSTATE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONNECTSTATE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_INFO_CLASS(pub i32);
impl ::core::marker::Copy for WTS_INFO_CLASS {}
impl ::core::clone::Clone for WTS_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_INFO_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(pub i32);
impl ::core::marker::Copy for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {}
impl ::core::clone::Clone for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_LOGON_ERROR_REDIRECTOR_RESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_RCM_DRAIN_STATE(pub i32);
impl ::core::marker::Copy for WTS_RCM_DRAIN_STATE {}
impl ::core::clone::Clone for WTS_RCM_DRAIN_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_RCM_DRAIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_RCM_DRAIN_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_RCM_DRAIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_DRAIN_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_RCM_SERVICE_STATE(pub i32);
impl ::core::marker::Copy for WTS_RCM_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_RCM_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_RCM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_RCM_SERVICE_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_RCM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_SECURITY_FLAGS(pub u32);
impl ::core::marker::Copy for WTS_SECURITY_FLAGS {}
impl ::core::clone::Clone for WTS_SECURITY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_SECURITY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_SECURITY_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_SECURITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_SECURITY_FLAGS").field(&self.0).finish()
    }
}
impl WTS_SECURITY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WTS_SECURITY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WTS_SECURITY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_TYPE_CLASS(pub i32);
impl ::core::marker::Copy for WTS_TYPE_CLASS {}
impl ::core::clone::Clone for WTS_TYPE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_TYPE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_TYPE_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_TYPE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_TYPE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WTS_VIRTUAL_CLASS(pub i32);
impl ::core::marker::Copy for WTS_VIRTUAL_CLASS {}
impl ::core::clone::Clone for WTS_VIRTUAL_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WTS_VIRTUAL_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WTS_VIRTUAL_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WTS_VIRTUAL_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_VIRTUAL_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct AAAccountingData {
    pub userName: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub clientName: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub authType: AAAuthSchemes,
    pub resourceName: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub portNumber: i32,
    pub protocolName: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub mainSessionId: ::windows_core::GUID,
    pub subSessionId: i32,
}
impl ::core::clone::Clone for AAAccountingData {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for AAAccountingData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAAccountingData")
            .field("userName", &self.userName)
            .field("clientName", &self.clientName)
            .field("authType", &self.authType)
            .field("resourceName", &self.resourceName)
            .field("portNumber", &self.portNumber)
            .field("protocolName", &self.protocolName)
            .field("numberOfBytesReceived", &self.numberOfBytesReceived)
            .field("numberOfBytesTransfered", &self.numberOfBytesTransfered)
            .field("reasonForDisconnect", &self.reasonForDisconnect)
            .field("mainSessionId", &self.mainSessionId)
            .field("subSessionId", &self.subSessionId)
            .finish()
    }
}
impl ::windows_core::TypeKind for AAAccountingData {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for AAAccountingData {
    fn eq(&self, other: &Self) -> bool {
        self.userName == other.userName && self.clientName == other.clientName && self.authType == other.authType && self.resourceName == other.resourceName && self.portNumber == other.portNumber && self.protocolName == other.protocolName && self.numberOfBytesReceived == other.numberOfBytesReceived && self.numberOfBytesTransfered == other.numberOfBytesTransfered && self.reasonForDisconnect == other.reasonForDisconnect && self.mainSessionId == other.mainSessionId && self.subSessionId == other.subSessionId
    }
}
impl ::core::cmp::Eq for AAAccountingData {}
impl ::core::default::Default for AAAccountingData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl ::core::marker::Copy for AE_CURRENT_POSITION {}
impl ::core::clone::Clone for AE_CURRENT_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AE_CURRENT_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CURRENT_POSITION").field("u64DevicePosition", &self.u64DevicePosition).field("u64StreamPosition", &self.u64StreamPosition).field("u64PaddingFrames", &self.u64PaddingFrames).field("hnsQPCPosition", &self.hnsQPCPosition).field("f32FramesPerSecond", &self.f32FramesPerSecond).field("Flag", &self.Flag).finish()
    }
}
impl ::windows_core::TypeKind for AE_CURRENT_POSITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for AE_CURRENT_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.u64DevicePosition == other.u64DevicePosition && self.u64StreamPosition == other.u64StreamPosition && self.u64PaddingFrames == other.u64PaddingFrames && self.hnsQPCPosition == other.hnsQPCPosition && self.f32FramesPerSecond == other.f32FramesPerSecond && self.Flag == other.Flag
    }
}
impl ::core::cmp::Eq for AE_CURRENT_POSITION {}
impl ::core::default::Default for AE_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl ::core::marker::Copy for BITMAP_RENDERER_STATISTICS {}
impl ::core::clone::Clone for BITMAP_RENDERER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BITMAP_RENDERER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP_RENDERER_STATISTICS").field("dwFramesDelivered", &self.dwFramesDelivered).field("dwFramesDropped", &self.dwFramesDropped).finish()
    }
}
impl ::windows_core::TypeKind for BITMAP_RENDERER_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for BITMAP_RENDERER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFramesDelivered == other.dwFramesDelivered && self.dwFramesDropped == other.dwFramesDropped
    }
}
impl ::core::cmp::Eq for BITMAP_RENDERER_STATISTICS {}
impl ::core::default::Default for BITMAP_RENDERER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct CHANNEL_DEF {
    pub name: [u8; 8],
    pub options: u32,
}
impl ::core::marker::Copy for CHANNEL_DEF {}
impl ::core::clone::Clone for CHANNEL_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for CHANNEL_DEF {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
impl ::core::marker::Copy for CHANNEL_ENTRY_POINTS {}
impl ::core::clone::Clone for CHANNEL_ENTRY_POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANNEL_ENTRY_POINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_ENTRY_POINTS").field("cbSize", &self.cbSize).field("protocolVersion", &self.protocolVersion).finish()
    }
}
impl ::windows_core::TypeKind for CHANNEL_ENTRY_POINTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for CHANNEL_ENTRY_POINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl ::core::marker::Copy for CHANNEL_PDU_HEADER {}
impl ::core::clone::Clone for CHANNEL_PDU_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANNEL_PDU_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_PDU_HEADER").field("length", &self.length).field("flags", &self.flags).finish()
    }
}
impl ::windows_core::TypeKind for CHANNEL_PDU_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CHANNEL_PDU_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for CHANNEL_PDU_HEADER {}
impl ::core::default::Default for CHANNEL_PDU_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for CLIENT_DISPLAY {}
impl ::core::clone::Clone for CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::windows_core::TypeKind for CLIENT_DISPLAY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for CLIENT_DISPLAY {}
impl ::core::default::Default for CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PRODUCT_INFOA {
    pub CompanyName: [u8; 256],
    pub ProductID: [u8; 4],
}
impl ::core::marker::Copy for PRODUCT_INFOA {}
impl ::core::clone::Clone for PRODUCT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRODUCT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRODUCT_INFOA").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
impl ::windows_core::TypeKind for PRODUCT_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PRODUCT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
impl ::core::cmp::Eq for PRODUCT_INFOA {}
impl ::core::default::Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl ::core::marker::Copy for PRODUCT_INFOW {}
impl ::core::clone::Clone for PRODUCT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRODUCT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRODUCT_INFOW").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
impl ::windows_core::TypeKind for PRODUCT_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PRODUCT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
impl ::core::cmp::Eq for PRODUCT_INFOW {}
impl ::core::default::Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MONITOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for RFX_GFX_MONITOR_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MONITOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for RFX_GFX_MSG_HEADER {}
impl ::core::clone::Clone for RFX_GFX_MSG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl ::core::marker::Copy for RFX_GFX_MSG_RDP_DATA {}
impl ::core::clone::Clone for RFX_GFX_MSG_RDP_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_MSG_RDP_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RFX_GFX_RECT {}
impl ::core::clone::Clone for RFX_GFX_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RFX_GFX_RECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RFX_GFX_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl ::core::marker::Copy for TSSD_ConnectionPoint {}
impl ::core::clone::Clone for TSSD_ConnectionPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TSSD_ConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TSSD_ConnectionPoint").field("ServerAddressB", &self.ServerAddressB).field("AddressType", &self.AddressType).field("PortNumber", &self.PortNumber).field("AddressScope", &self.AddressScope).finish()
    }
}
impl ::windows_core::TypeKind for TSSD_ConnectionPoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TSSD_ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.ServerAddressB == other.ServerAddressB && self.AddressType == other.AddressType && self.PortNumber == other.PortNumber && self.AddressScope == other.AddressScope
    }
}
impl ::core::cmp::Eq for TSSD_ConnectionPoint {}
impl ::core::default::Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl ::core::marker::Copy for VM_NOTIFY_ENTRY {}
impl ::core::clone::Clone for VM_NOTIFY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_ENTRY").field("VmName", &self.VmName).field("VmHost", &self.VmHost).finish()
    }
}
impl ::windows_core::TypeKind for VM_NOTIFY_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VM_NOTIFY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VmName == other.VmName && self.VmHost == other.VmHost
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_ENTRY {}
impl ::core::default::Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl ::core::marker::Copy for VM_NOTIFY_INFO {}
impl ::core::clone::Clone for VM_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_INFO").field("dwNumEntries", &self.dwNumEntries).field("ppVmEntries", &self.ppVmEntries).finish()
    }
}
impl ::windows_core::TypeKind for VM_NOTIFY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VM_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.ppVmEntries == other.ppVmEntries
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_INFO {}
impl ::core::default::Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut ::windows_core::PWSTR,
}
impl ::core::marker::Copy for VM_PATCH_INFO {}
impl ::core::clone::Clone for VM_PATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VM_PATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_PATCH_INFO").field("dwNumEntries", &self.dwNumEntries).field("pVmNames", &self.pVmNames).finish()
    }
}
impl ::windows_core::TypeKind for VM_PATCH_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VM_PATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.pVmNames == other.pVmNames
    }
}
impl ::core::cmp::Eq for VM_PATCH_INFO {}
impl ::core::default::Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_CONNECTION_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_CONNECTION_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: super::super::Foundation::BOOLEAN,
    pub fInheritColorDepth: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOLEAN,
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fResetBroken: super::super::Foundation::BOOLEAN,
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub HRes: u16,
    pub VRes: u16,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub KeyboardLayout: u32,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub PerformanceFlags: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ActiveInputLocale: u32,
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientBuildNumber: u32,
    pub ClientSessionId: u32,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserName: [u16; 256],
    pub Domain: [u16; 256],
    pub Password: [u16; 256],
    pub ProtocolName: [u16; 9],
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub imeFileName: [u16; 33],
    pub AudioDriverName: [u16; 9],
    pub ClientName: [u16; 21],
    pub ClientAddress: [u16; 31],
    pub ClientDirectory: [u16; 257],
    pub ClientDigProductId: [u16; 33],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub WRdsListenerSettings: WRDS_LISTENER_SETTINGS,
    pub EventLogActivityId: ::windows_core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_CONNECTION_SETTINGS_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: u16,
}
impl ::core::marker::Copy for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_DYNAMIC_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).field("TimeZoneKeyName", &self.TimeZoneKeyName).field("DynamicDaylightTimeDisabled", &self.DynamicDaylightTimeDisabled).finish()
    }
}
impl ::windows_core::TypeKind for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias && self.TimeZoneKeyName == other.TimeZoneKeyName && self.DynamicDaylightTimeDisabled == other.DynamicDaylightTimeDisabled
    }
}
impl ::core::cmp::Eq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTING {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WRDS_LISTENER_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WRDS_LISTENER_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WRDS_LISTENER_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_LISTENER_SETTINGS_1").field("MaxProtocolListenerConnectionCount", &self.MaxProtocolListenerConnectionCount).field("SecurityDescriptorSize", &self.SecurityDescriptorSize).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
impl ::windows_core::TypeKind for WRDS_LISTENER_SETTINGS_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxProtocolListenerConnectionCount == other.MaxProtocolListenerConnectionCount && self.SecurityDescriptorSize == other.SecurityDescriptorSize && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_SETTING {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS_1 {
    pub WRdsDisableClipStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableClipValue: u32,
    pub WRdsDisableLPTStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableLPTValue: u32,
    pub WRdsDisableCcmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCcmValue: u32,
    pub WRdsDisableCdmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCdmValue: u32,
    pub WRdsDisableCpmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCpmValue: u32,
    pub WRdsDisablePnpStatus: WRDS_SETTING_STATUS,
    pub WRdsDisablePnpValue: u32,
    pub WRdsEncryptionLevelStatus: WRDS_SETTING_STATUS,
    pub WRdsEncryptionValue: u32,
    pub WRdsColorDepthStatus: WRDS_SETTING_STATUS,
    pub WRdsColorDepthValue: u32,
    pub WRdsDisableAutoReconnecetStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableAutoReconnecetValue: u32,
    pub WRdsDisableEncryptionStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableEncryptionValue: u32,
    pub WRdsResetBrokenStatus: WRDS_SETTING_STATUS,
    pub WRdsResetBrokenValue: u32,
    pub WRdsMaxIdleTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxIdleTimeValue: u32,
    pub WRdsMaxDisconnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxDisconnectTimeValue: u32,
    pub WRdsMaxConnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxConnectTimeValue: u32,
    pub WRdsKeepAliveStatus: WRDS_SETTING_STATUS,
    pub WRdsKeepAliveStartValue: super::super::Foundation::BOOLEAN,
    pub WRdsKeepAliveIntervalValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WRDS_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_SETTINGS_1")
            .field("WRdsDisableClipStatus", &self.WRdsDisableClipStatus)
            .field("WRdsDisableClipValue", &self.WRdsDisableClipValue)
            .field("WRdsDisableLPTStatus", &self.WRdsDisableLPTStatus)
            .field("WRdsDisableLPTValue", &self.WRdsDisableLPTValue)
            .field("WRdsDisableCcmStatus", &self.WRdsDisableCcmStatus)
            .field("WRdsDisableCcmValue", &self.WRdsDisableCcmValue)
            .field("WRdsDisableCdmStatus", &self.WRdsDisableCdmStatus)
            .field("WRdsDisableCdmValue", &self.WRdsDisableCdmValue)
            .field("WRdsDisableCpmStatus", &self.WRdsDisableCpmStatus)
            .field("WRdsDisableCpmValue", &self.WRdsDisableCpmValue)
            .field("WRdsDisablePnpStatus", &self.WRdsDisablePnpStatus)
            .field("WRdsDisablePnpValue", &self.WRdsDisablePnpValue)
            .field("WRdsEncryptionLevelStatus", &self.WRdsEncryptionLevelStatus)
            .field("WRdsEncryptionValue", &self.WRdsEncryptionValue)
            .field("WRdsColorDepthStatus", &self.WRdsColorDepthStatus)
            .field("WRdsColorDepthValue", &self.WRdsColorDepthValue)
            .field("WRdsDisableAutoReconnecetStatus", &self.WRdsDisableAutoReconnecetStatus)
            .field("WRdsDisableAutoReconnecetValue", &self.WRdsDisableAutoReconnecetValue)
            .field("WRdsDisableEncryptionStatus", &self.WRdsDisableEncryptionStatus)
            .field("WRdsDisableEncryptionValue", &self.WRdsDisableEncryptionValue)
            .field("WRdsResetBrokenStatus", &self.WRdsResetBrokenStatus)
            .field("WRdsResetBrokenValue", &self.WRdsResetBrokenValue)
            .field("WRdsMaxIdleTimeStatus", &self.WRdsMaxIdleTimeStatus)
            .field("WRdsMaxIdleTimeValue", &self.WRdsMaxIdleTimeValue)
            .field("WRdsMaxDisconnectTimeStatus", &self.WRdsMaxDisconnectTimeStatus)
            .field("WRdsMaxDisconnectTimeValue", &self.WRdsMaxDisconnectTimeValue)
            .field("WRdsMaxConnectTimeStatus", &self.WRdsMaxConnectTimeStatus)
            .field("WRdsMaxConnectTimeValue", &self.WRdsMaxConnectTimeValue)
            .field("WRdsKeepAliveStatus", &self.WRdsKeepAliveStatus)
            .field("WRdsKeepAliveStartValue", &self.WRdsKeepAliveStartValue)
            .field("WRdsKeepAliveIntervalValue", &self.WRdsKeepAliveIntervalValue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WRDS_SETTINGS_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.WRdsDisableClipStatus == other.WRdsDisableClipStatus
            && self.WRdsDisableClipValue == other.WRdsDisableClipValue
            && self.WRdsDisableLPTStatus == other.WRdsDisableLPTStatus
            && self.WRdsDisableLPTValue == other.WRdsDisableLPTValue
            && self.WRdsDisableCcmStatus == other.WRdsDisableCcmStatus
            && self.WRdsDisableCcmValue == other.WRdsDisableCcmValue
            && self.WRdsDisableCdmStatus == other.WRdsDisableCdmStatus
            && self.WRdsDisableCdmValue == other.WRdsDisableCdmValue
            && self.WRdsDisableCpmStatus == other.WRdsDisableCpmStatus
            && self.WRdsDisableCpmValue == other.WRdsDisableCpmValue
            && self.WRdsDisablePnpStatus == other.WRdsDisablePnpStatus
            && self.WRdsDisablePnpValue == other.WRdsDisablePnpValue
            && self.WRdsEncryptionLevelStatus == other.WRdsEncryptionLevelStatus
            && self.WRdsEncryptionValue == other.WRdsEncryptionValue
            && self.WRdsColorDepthStatus == other.WRdsColorDepthStatus
            && self.WRdsColorDepthValue == other.WRdsColorDepthValue
            && self.WRdsDisableAutoReconnecetStatus == other.WRdsDisableAutoReconnecetStatus
            && self.WRdsDisableAutoReconnecetValue == other.WRdsDisableAutoReconnecetValue
            && self.WRdsDisableEncryptionStatus == other.WRdsDisableEncryptionStatus
            && self.WRdsDisableEncryptionValue == other.WRdsDisableEncryptionValue
            && self.WRdsResetBrokenStatus == other.WRdsResetBrokenStatus
            && self.WRdsResetBrokenValue == other.WRdsResetBrokenValue
            && self.WRdsMaxIdleTimeStatus == other.WRdsMaxIdleTimeStatus
            && self.WRdsMaxIdleTimeValue == other.WRdsMaxIdleTimeValue
            && self.WRdsMaxDisconnectTimeStatus == other.WRdsMaxDisconnectTimeStatus
            && self.WRdsMaxDisconnectTimeValue == other.WRdsMaxDisconnectTimeValue
            && self.WRdsMaxConnectTimeStatus == other.WRdsMaxConnectTimeStatus
            && self.WRdsMaxConnectTimeValue == other.WRdsMaxConnectTimeValue
            && self.WRdsKeepAliveStatus == other.WRdsKeepAliveStatus
            && self.WRdsKeepAliveStartValue == other.WRdsKeepAliveStartValue
            && self.WRdsKeepAliveIntervalValue == other.WRdsKeepAliveIntervalValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSCLIENTA {
    pub ClientName: [u8; 21],
    pub Domain: [u8; 18],
    pub UserName: [u8; 21],
    pub WorkDirectory: [u8; 261],
    pub InitialProgram: [u8; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u8; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u8; 261],
}
impl ::core::marker::Copy for WTSCLIENTA {}
impl ::core::clone::Clone for WTSCLIENTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCLIENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTA")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSCLIENTA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSCLIENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for WTSCLIENTA {}
impl ::core::default::Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSCLIENTW {
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u16; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u16; 261],
}
impl ::core::marker::Copy for WTSCLIENTW {}
impl ::core::clone::Clone for WTSCLIENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCLIENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTW")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSCLIENTW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSCLIENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for WTSCLIENTW {}
impl ::core::default::Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u8; 21],
    pub LogonDomain: [u8; 18],
    pub WorkDirectory: [u8; 261],
    pub InitialProgram: [u8; 261],
    pub ApplicationName: [u8; 261],
}
impl ::core::marker::Copy for WTSCONFIGINFOA {}
impl ::core::clone::Clone for WTSCONFIGINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCONFIGINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOA")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSCONFIGINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSCONFIGINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter && self.ShadowSettings == other.ShadowSettings && self.LogonUserName == other.LogonUserName && self.LogonDomain == other.LogonDomain && self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.ApplicationName == other.ApplicationName
    }
}
impl ::core::cmp::Eq for WTSCONFIGINFOA {}
impl ::core::default::Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSCONFIGINFOW {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub ApplicationName: [u16; 261],
}
impl ::core::marker::Copy for WTSCONFIGINFOW {}
impl ::core::clone::Clone for WTSCONFIGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSCONFIGINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOW")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSCONFIGINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSCONFIGINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter && self.ShadowSettings == other.ShadowSettings && self.LogonUserName == other.LogonUserName && self.LogonDomain == other.LogonDomain && self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.ApplicationName == other.ApplicationName
    }
}
impl ::core::cmp::Eq for WTSCONFIGINFOW {}
impl ::core::default::Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [u8; 32],
    pub Domain: [u8; 17],
    pub UserName: [u8; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl ::core::marker::Copy for WTSINFOA {}
impl ::core::clone::Clone for WTSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOA")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBy", &self.OutgoingCompressedBy)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.SessionId == other.SessionId && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBy == other.OutgoingCompressedBy && self.WinStationName == other.WinStationName && self.Domain == other.Domain && self.UserName == other.UserName && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.LogonTime == other.LogonTime && self.CurrentTime == other.CurrentTime
    }
}
impl ::core::cmp::Eq for WTSINFOA {}
impl ::core::default::Default for WTSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
impl ::core::marker::Copy for WTSINFOEXA {}
impl ::core::clone::Clone for WTSINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTSINFOEXA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl ::core::marker::Copy for WTSINFOEXW {}
impl ::core::clone::Clone for WTSINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTSINFOEXW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u8; 33],
    pub UserName: [u8; 21],
    pub DomainName: [u8; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_A {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_A")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSINFOEX_LEVEL1_A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_A {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.SessionState == other.SessionState && self.SessionFlags == other.SessionFlags && self.WinStationName == other.WinStationName && self.UserName == other.UserName && self.DomainName == other.DomainName && self.LogonTime == other.LogonTime && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.CurrentTime == other.CurrentTime && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_A {}
impl ::core::default::Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_W")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSINFOEX_LEVEL1_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_W {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.SessionState == other.SessionState && self.SessionFlags == other.SessionFlags && self.WinStationName == other.WinStationName && self.UserName == other.UserName && self.DomainName == other.DomainName && self.LogonTime == other.LogonTime && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.CurrentTime == other.CurrentTime && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_W {}
impl ::core::default::Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL_A {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTSINFOEX_LEVEL_A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTSINFOEX_LEVEL_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
    pub WinStationName: [u16; 32],
    pub Domain: [u16; 17],
    pub UserName: [u16; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl ::core::marker::Copy for WTSINFOW {}
impl ::core::clone::Clone for WTSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOW")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.SessionId == other.SessionId && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes && self.WinStationName == other.WinStationName && self.Domain == other.Domain && self.UserName == other.UserName && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.LogonTime == other.LogonTime && self.CurrentTime == other.CurrentTime
    }
}
impl ::core::cmp::Eq for WTSINFOW {}
impl ::core::default::Default for WTSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSLISTENERCONFIGA {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u8; 61],
    pub LogonUserName: [u8; 21],
    pub LogonDomain: [u8; 18],
    pub WorkDirectory: [u8; 261],
    pub InitialProgram: [u8; 261],
}
impl ::core::marker::Copy for WTSLISTENERCONFIGA {}
impl ::core::clone::Clone for WTSLISTENERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSLISTENERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGA")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSLISTENERCONFIGA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
impl ::core::cmp::Eq for WTSLISTENERCONFIGA {}
impl ::core::default::Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSLISTENERCONFIGW {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u16; 61],
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
}
impl ::core::marker::Copy for WTSLISTENERCONFIGW {}
impl ::core::clone::Clone for WTSLISTENERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSLISTENERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGW")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSLISTENERCONFIGW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
impl ::core::cmp::Eq for WTSLISTENERCONFIGW {}
impl ::core::default::Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl ::core::marker::Copy for WTSSBX_IP_ADDRESS {}
impl ::core::clone::Clone for WTSSBX_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_IP_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_IP_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).field("PortNumber", &self.PortNumber).field("dwScope", &self.dwScope).finish()
    }
}
impl ::windows_core::TypeKind for WTSSBX_IP_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSSBX_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address && self.PortNumber == other.PortNumber && self.dwScope == other.dwScope
    }
}
impl ::core::cmp::Eq for WTSSBX_IP_ADDRESS {}
impl ::core::default::Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl ::core::marker::Copy for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_CONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_CONNECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_CONNECT_INFO").field("wczMachineFQDN", &self.wczMachineFQDN).field("wczMachineNetBiosName", &self.wczMachineNetBiosName).field("dwNumOfIPAddr", &self.dwNumOfIPAddr).field("IPaddr", &self.IPaddr).finish()
    }
}
impl ::windows_core::TypeKind for WTSSBX_MACHINE_CONNECT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_CONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wczMachineFQDN == other.wczMachineFQDN && self.wczMachineNetBiosName == other.wczMachineNetBiosName && self.dwNumOfIPAddr == other.dwNumOfIPAddr && self.IPaddr == other.IPaddr
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSSBX_MACHINE_INFO {
    pub ClientConnectInfo: WTSSBX_MACHINE_CONNECT_INFO,
    pub wczFarmName: [u16; 257],
    pub InternalIPAddress: WTSSBX_IP_ADDRESS,
    pub dwMaxSessionsLimit: u32,
    pub ServerWeight: u32,
    pub SingleSessionMode: WTSSBX_MACHINE_SESSION_MODE,
    pub InDrain: WTSSBX_MACHINE_DRAIN,
    pub MachineState: WTSSBX_MACHINE_STATE,
}
impl ::core::marker::Copy for WTSSBX_MACHINE_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_INFO").field("ClientConnectInfo", &self.ClientConnectInfo).field("wczFarmName", &self.wczFarmName).field("InternalIPAddress", &self.InternalIPAddress).field("dwMaxSessionsLimit", &self.dwMaxSessionsLimit).field("ServerWeight", &self.ServerWeight).field("SingleSessionMode", &self.SingleSessionMode).field("InDrain", &self.InDrain).field("MachineState", &self.MachineState).finish()
    }
}
impl ::windows_core::TypeKind for WTSSBX_MACHINE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientConnectInfo == other.ClientConnectInfo && self.wczFarmName == other.wczFarmName && self.InternalIPAddress == other.InternalIPAddress && self.dwMaxSessionsLimit == other.dwMaxSessionsLimit && self.ServerWeight == other.ServerWeight && self.SingleSessionMode == other.SingleSessionMode && self.InDrain == other.InDrain && self.MachineState == other.MachineState
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_INFO {}
impl ::core::default::Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSSBX_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSSBX_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_SESSION_INFO").field("wszUserName", &self.wszUserName).field("wszDomainName", &self.wszDomainName).field("ApplicationType", &self.ApplicationType).field("dwSessionId", &self.dwSessionId).field("CreateTime", &self.CreateTime).field("DisconnectTime", &self.DisconnectTime).field("SessionState", &self.SessionState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTSSBX_SESSION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSSBX_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wszUserName == other.wszUserName && self.wszDomainName == other.wszDomainName && self.ApplicationType == other.ApplicationType && self.dwSessionId == other.dwSessionId && self.CreateTime == other.CreateTime && self.DisconnectTime == other.DisconnectTime && self.SessionState == other.SessionState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl ::core::marker::Copy for WTSSESSION_NOTIFICATION {}
impl ::core::clone::Clone for WTSSESSION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSSESSION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSESSION_NOTIFICATION").field("cbSize", &self.cbSize).field("dwSessionId", &self.dwSessionId).finish()
    }
}
impl ::windows_core::TypeKind for WTSSESSION_NOTIFICATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSSESSION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSessionId == other.dwSessionId
    }
}
impl ::core::cmp::Eq for WTSSESSION_NOTIFICATION {}
impl ::core::default::Default for WTSSESSION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSUSERCONFIGA {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u8; 261],
    pub WorkDirectory: [u8; 261],
    pub TerminalServerProfilePath: [u8; 261],
    pub TerminalServerHomeDir: [u8; 261],
    pub TerminalServerHomeDirDrive: [u8; 4],
}
impl ::core::marker::Copy for WTSUSERCONFIGA {}
impl ::core::clone::Clone for WTSUSERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSUSERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGA")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSUSERCONFIGA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSUSERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
impl ::core::cmp::Eq for WTSUSERCONFIGA {}
impl ::core::default::Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTSUSERCONFIGW {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u16; 261],
    pub WorkDirectory: [u16; 261],
    pub TerminalServerProfilePath: [u16; 261],
    pub TerminalServerHomeDir: [u16; 261],
    pub TerminalServerHomeDirDrive: [u16; 4],
}
impl ::core::marker::Copy for WTSUSERCONFIGW {}
impl ::core::clone::Clone for WTSUSERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTSUSERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGW")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTSUSERCONFIGW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTSUSERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
impl ::core::cmp::Eq for WTSUSERCONFIGW {}
impl ::core::default::Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for WTS_CACHE_STATS {}
impl ::core::clone::Clone for WTS_CACHE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_CACHE_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_CACHE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl ::core::marker::Copy for WTS_CACHE_STATS_UN {}
impl ::core::clone::Clone for WTS_CACHE_STATS_UN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_CACHE_STATS_UN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_CLIENT_ADDRESS {}
impl ::core::clone::Clone for WTS_CLIENT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for WTS_CLIENT_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_CLIENT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_ADDRESS {}
impl ::core::default::Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOL,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fInheritInitialProgram: super::super::Foundation::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: super::super::Foundation::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub ClientName: [u16; 21],
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientDirectory: [u16; 257],
    pub ClientBuildNumber: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 33],
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_CLIENT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for WTS_CLIENT_DISPLAY {}
impl ::core::clone::Clone for WTS_CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::windows_core::TypeKind for WTS_CLIENT_DISPLAY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_DISPLAY {}
impl ::core::default::Default for WTS_CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl ::core::marker::Copy for WTS_DISPLAY_IOCTL {}
impl ::core::clone::Clone for WTS_DISPLAY_IOCTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_DISPLAY_IOCTL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_DISPLAY_IOCTL").field("pDisplayIOCtlData", &self.pDisplayIOCtlData).field("cbDisplayIOCtlData", &self.cbDisplayIOCtlData).finish()
    }
}
impl ::windows_core::TypeKind for WTS_DISPLAY_IOCTL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_DISPLAY_IOCTL {
    fn eq(&self, other: &Self) -> bool {
        self.pDisplayIOCtlData == other.pDisplayIOCtlData && self.cbDisplayIOCtlData == other.cbDisplayIOCtlData
    }
}
impl ::core::cmp::Eq for WTS_DISPLAY_IOCTL {}
impl ::core::default::Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_LICENSE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_LICENSE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_LICENSE_CAPABILITIES").field("KeyExchangeAlg", &self.KeyExchangeAlg).field("ProtocolVer", &self.ProtocolVer).field("fAuthenticateServer", &self.fAuthenticateServer).field("CertType", &self.CertType).field("cbClientName", &self.cbClientName).field("rgbClientName", &self.rgbClientName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_LICENSE_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_LICENSE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.KeyExchangeAlg == other.KeyExchangeAlg && self.ProtocolVer == other.ProtocolVer && self.fAuthenticateServer == other.fAuthenticateServer && self.CertType == other.CertType && self.cbClientName == other.cbClientName && self.rgbClientName == other.rgbClientName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNPRedir: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_POLICY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_POLICY_DATA")
            .field("fDisableEncryption", &self.fDisableEncryption)
            .field("fDisableAutoReconnect", &self.fDisableAutoReconnect)
            .field("ColorDepth", &self.ColorDepth)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("fDisableCpm", &self.fDisableCpm)
            .field("fDisableCdm", &self.fDisableCdm)
            .field("fDisableCcm", &self.fDisableCcm)
            .field("fDisableLPT", &self.fDisableLPT)
            .field("fDisableClip", &self.fDisableClip)
            .field("fDisablePNPRedir", &self.fDisablePNPRedir)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_POLICY_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.fDisableEncryption == other.fDisableEncryption && self.fDisableAutoReconnect == other.fDisableAutoReconnect && self.ColorDepth == other.ColorDepth && self.MinEncryptionLevel == other.MinEncryptionLevel && self.fDisableCpm == other.fDisableCpm && self.fDisableCdm == other.fDisableCdm && self.fDisableCcm == other.fDisableCcm && self.fDisableLPT == other.fDisableLPT && self.fDisableClip == other.fDisableClip && self.fDisablePNPRedir == other.fDisablePNPRedir
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOA").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_PROCESS_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOW").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_PROCESS_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_core::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXA")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_PROCESS_INFO_EXA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid && self.NumberOfThreads == other.NumberOfThreads && self.HandleCount == other.HandleCount && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.WorkingSetSize == other.WorkingSetSize && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.UserTime == other.UserTime && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: ::windows_core::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXW")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for WTS_PROCESS_INFO_EXW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid && self.NumberOfThreads == other.NumberOfThreads && self.HandleCount == other.HandleCount && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.WorkingSetSize == other.WorkingSetSize && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.UserTime == other.UserTime && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_PROPERTY_VALUE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: ::windows_core::GUID,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_PROPERTY_VALUE_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: ::windows_core::PSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_0 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROPERTY_VALUE_0_0").field("size", &self.size).field("pbVal", &self.pbVal).finish()
    }
}
impl ::windows_core::TypeKind for WTS_PROPERTY_VALUE_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.pbVal == other.pbVal
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_0 {}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_1 {}
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROPERTY_VALUE_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROPERTY_VALUE_0_1").field("size", &self.size).field("pstrVal", &self.pstrVal).finish()
    }
}
impl ::windows_core::TypeKind for WTS_PROPERTY_VALUE_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_PROPERTY_VALUE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.pstrVal == other.pstrVal
    }
}
impl ::core::cmp::Eq for WTS_PROPERTY_VALUE_0_1 {}
impl ::core::default::Default for WTS_PROPERTY_VALUE_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl ::core::marker::Copy for WTS_PROTOCOL_CACHE {}
impl ::core::clone::Clone for WTS_PROTOCOL_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_CACHE").field("CacheReads", &self.CacheReads).field("CacheHits", &self.CacheHits).finish()
    }
}
impl ::windows_core::TypeKind for WTS_PROTOCOL_CACHE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_CACHE {
    fn eq(&self, other: &Self) -> bool {
        self.CacheReads == other.CacheReads && self.CacheHits == other.CacheHits
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_CACHE {}
impl ::core::default::Default for WTS_PROTOCOL_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROTOCOL_COUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: u16,
    pub Reserved: [u32; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_COUNTERS {}
impl ::core::clone::Clone for WTS_PROTOCOL_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_PROTOCOL_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_COUNTERS")
            .field("WdBytes", &self.WdBytes)
            .field("WdFrames", &self.WdFrames)
            .field("WaitForOutBuf", &self.WaitForOutBuf)
            .field("Frames", &self.Frames)
            .field("Bytes", &self.Bytes)
            .field("CompressedBytes", &self.CompressedBytes)
            .field("CompressFlushes", &self.CompressFlushes)
            .field("Errors", &self.Errors)
            .field("Timeouts", &self.Timeouts)
            .field("AsyncFramingError", &self.AsyncFramingError)
            .field("AsyncOverrunError", &self.AsyncOverrunError)
            .field("AsyncOverflowError", &self.AsyncOverflowError)
            .field("AsyncParityError", &self.AsyncParityError)
            .field("TdErrors", &self.TdErrors)
            .field("ProtocolType", &self.ProtocolType)
            .field("Length", &self.Length)
            .field("Specific", &self.Specific)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::windows_core::TypeKind for WTS_PROTOCOL_COUNTERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.WdBytes == other.WdBytes && self.WdFrames == other.WdFrames && self.WaitForOutBuf == other.WaitForOutBuf && self.Frames == other.Frames && self.Bytes == other.Bytes && self.CompressedBytes == other.CompressedBytes && self.CompressFlushes == other.CompressFlushes && self.Errors == other.Errors && self.Timeouts == other.Timeouts && self.AsyncFramingError == other.AsyncFramingError && self.AsyncOverrunError == other.AsyncOverrunError && self.AsyncOverflowError == other.AsyncOverflowError && self.AsyncParityError == other.AsyncParityError && self.TdErrors == other.TdErrors && self.ProtocolType == other.ProtocolType && self.Length == other.Length && self.Specific == other.Specific && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_COUNTERS {}
impl ::core::default::Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_STATUS {}
impl ::core::clone::Clone for WTS_PROTOCOL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_PROTOCOL_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOA {}
impl ::core::clone::Clone for WTS_SERVER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOA").field("pServerName", &self.pServerName).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SERVER_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOA {}
impl ::core::default::Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WTS_SERVER_INFOW {}
impl ::core::clone::Clone for WTS_SERVER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOW").field("pServerName", &self.pServerName).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SERVER_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOW {}
impl ::core::default::Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl ::core::marker::Copy for WTS_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVICE_STATE").field("RcmServiceState", &self.RcmServiceState).field("RcmDrainState", &self.RcmDrainState).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SERVICE_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SERVICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.RcmServiceState == other.RcmServiceState && self.RcmDrainState == other.RcmDrainState
    }
}
impl ::core::cmp::Eq for WTS_SERVICE_STATE {}
impl ::core::default::Default for WTS_SERVICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_SESSION_ADDRESS {}
impl ::core::clone::Clone for WTS_SESSION_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ADDRESS {}
impl ::core::default::Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: ::windows_core::GUID,
    pub SessionId: u32,
}
impl ::core::marker::Copy for WTS_SESSION_ID {}
impl ::core::clone::Clone for WTS_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ID").field("SessionUniqueGuid", &self.SessionUniqueGuid).field("SessionId", &self.SessionId).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionUniqueGuid == other.SessionUniqueGuid && self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ID {}
impl ::core::default::Default for WTS_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: ::windows_core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOA {}
impl ::core::clone::Clone for WTS_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOA").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOA {}
impl ::core::default::Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: ::windows_core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl ::core::marker::Copy for WTS_SESSION_INFOW {}
impl ::core::clone::Clone for WTS_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOW").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOW {}
impl ::core::default::Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows_core::PSTR,
    pub pHostName: ::windows_core::PSTR,
    pub pUserName: ::windows_core::PSTR,
    pub pDomainName: ::windows_core::PSTR,
    pub pFarmName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1A {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1A").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_INFO_1A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1A {}
impl ::core::default::Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: ::windows_core::PWSTR,
    pub pHostName: ::windows_core::PWSTR,
    pub pUserName: ::windows_core::PWSTR,
    pub pDomainName: ::windows_core::PWSTR,
    pub pFarmName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WTS_SESSION_INFO_1W {}
impl ::core::clone::Clone for WTS_SESSION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1W").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SESSION_INFO_1W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1W {}
impl ::core::default::Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for WTS_SMALL_RECT {}
impl ::core::clone::Clone for WTS_SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SMALL_RECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for WTS_SMALL_RECT {}
impl ::core::default::Default for WTS_SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl ::core::marker::Copy for WTS_SOCKADDR {}
impl ::core::clone::Clone for WTS_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_SOCKADDR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WTS_SOCKADDR_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WTS_SOCKADDR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SOCKADDR_0_0").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SOCKADDR_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sin_port == other.sin_port && self.IN_ADDR == other.IN_ADDR && self.sin_zero == other.sin_zero
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_0 {}
impl ::core::default::Default for WTS_SOCKADDR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_1 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SOCKADDR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SOCKADDR_0_1").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SOCKADDR_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SOCKADDR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_port == other.sin6_port && self.sin6_flowinfo == other.sin6_flowinfo && self.sin6_addr == other.sin6_addr && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::core::cmp::Eq for WTS_SOCKADDR_0_1 {}
impl ::core::default::Default for WTS_SOCKADDR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WTS_SYSTEMTIME {}
impl ::core::clone::Clone for WTS_SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_SYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
impl ::windows_core::TypeKind for WTS_SYSTEMTIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDayOfWeek == other.wDayOfWeek && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds
    }
}
impl ::core::cmp::Eq for WTS_SYSTEMTIME {}
impl ::core::default::Default for WTS_SYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for WTS_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WTS_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).finish()
    }
}
impl ::windows_core::TypeKind for WTS_TIME_ZONE_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias
    }
}
impl ::core::cmp::Eq for WTS_TIME_ZONE_INFORMATION {}
impl ::core::default::Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl ::core::marker::Copy for WTS_USER_CREDENTIAL {}
impl ::core::clone::Clone for WTS_USER_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_USER_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_CREDENTIAL").field("UserName", &self.UserName).field("Password", &self.Password).field("Domain", &self.Domain).finish()
    }
}
impl ::windows_core::TypeKind for WTS_USER_CREDENTIAL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_USER_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.Password == other.Password && self.Domain == other.Domain
    }
}
impl ::core::cmp::Eq for WTS_USER_CREDENTIAL {}
impl ::core::default::Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl ::core::marker::Copy for WTS_USER_DATA {}
impl ::core::clone::Clone for WTS_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_USER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_DATA").field("WorkDirectory", &self.WorkDirectory).field("InitialProgram", &self.InitialProgram).field("UserTimeZone", &self.UserTimeZone).finish()
    }
}
impl ::windows_core::TypeKind for WTS_USER_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_USER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.UserTimeZone == other.UserTimeZone
    }
}
impl ::core::cmp::Eq for WTS_USER_DATA {}
impl ::core::default::Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONA {}
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONA").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
impl ::windows_core::TypeKind for WTS_VALIDATION_INFORMATIONA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONA {}
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONW {}
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONW").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
impl ::windows_core::TypeKind for WTS_VALIDATION_INFORMATIONW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONW {}
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: ::windows_core::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource {}
impl ::core::clone::Clone for pluginResource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource")
            .field("alias", &self.alias)
            .field("name", &self.name)
            .field("resourceFileContents", &self.resourceFileContents)
            .field("fileExtension", &self.fileExtension)
            .field("resourcePluginType", &self.resourcePluginType)
            .field("isDiscoverable", &self.isDiscoverable)
            .field("resourceType", &self.resourceType)
            .field("pceIconSize", &self.pceIconSize)
            .field("iconContents", &self.iconContents)
            .field("pcePluginBlobSize", &self.pcePluginBlobSize)
            .field("blobContents", &self.blobContents)
            .finish()
    }
}
impl ::windows_core::TypeKind for pluginResource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for pluginResource {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias && self.name == other.name && self.resourceFileContents == other.resourceFileContents && self.fileExtension == other.fileExtension && self.resourcePluginType == other.resourcePluginType && self.isDiscoverable == other.isDiscoverable && self.resourceType == other.resourceType && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents && self.pcePluginBlobSize == other.pcePluginBlobSize && self.blobContents == other.blobContents
    }
}
impl ::core::cmp::Eq for pluginResource {}
impl ::core::default::Default for pluginResource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: ::windows_core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl ::core::marker::Copy for pluginResource2 {}
impl ::core::clone::Clone for pluginResource2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2").field("resourceV1", &self.resourceV1).field("pceFileAssocListSize", &self.pceFileAssocListSize).field("fileAssocList", &self.fileAssocList).field("securityDescriptor", &self.securityDescriptor).field("pceFolderListSize", &self.pceFolderListSize).field("folderList", &self.folderList).finish()
    }
}
impl ::windows_core::TypeKind for pluginResource2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for pluginResource2 {
    fn eq(&self, other: &Self) -> bool {
        self.resourceV1 == other.resourceV1 && self.pceFileAssocListSize == other.pceFileAssocListSize && self.fileAssocList == other.fileAssocList && self.securityDescriptor == other.securityDescriptor && self.pceFolderListSize == other.pceFolderListSize && self.folderList == other.folderList
    }
}
impl ::core::cmp::Eq for pluginResource2 {}
impl ::core::default::Default for pluginResource2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource2FileAssociation {}
impl ::core::clone::Clone for pluginResource2FileAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pluginResource2FileAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2FileAssociation").field("extName", &self.extName).field("primaryHandler", &self.primaryHandler).field("pceIconSize", &self.pceIconSize).field("iconContents", &self.iconContents).finish()
    }
}
impl ::windows_core::TypeKind for pluginResource2FileAssociation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for pluginResource2FileAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.extName == other.extName && self.primaryHandler == other.primaryHandler && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents
    }
}
impl ::core::cmp::Eq for pluginResource2FileAssociation {}
impl ::core::default::Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PCHANNEL_INIT_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32) -> ()>;
pub type PCHANNEL_OPEN_EVENT_FN = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32) -> ()>;
pub type PVIRTUALCHANNELCLOSE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELENTRY = ::core::option::Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> super::super::Foundation::BOOL>;
pub type PVIRTUALCHANNELINIT = ::core::option::Option<unsafe extern "system" fn(ppinithandle: *mut *mut ::core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELOPEN = ::core::option::Option<unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, popenhandle: *mut u32, pchannelname: ::windows_core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELWRITE = ::core::option::Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, puserdata: *mut ::core::ffi::c_void) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
