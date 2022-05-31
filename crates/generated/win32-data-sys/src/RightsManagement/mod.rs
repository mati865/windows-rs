#[link(name = "windows")]
extern "system" {
    pub fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows_sys_core::PCWSTR, wszurl: ::windows_sys_core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys_core::HRESULT;
    pub fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *const ::windows_sys_core::PWSTR, wszurl: ::windows_sys_core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys_core::HRESULT;
    pub fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows_sys_core::PCWSTR, wszrequestedrights: ::windows_sys_core::PCWSTR, wszcustomdata: ::windows_sys_core::PCWSTR, wszurl: ::windows_sys_core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND) -> ::windows_sys_core::HRESULT;
    pub fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMAttest(henablingprincipal: u32, wszdata: ::windows_sys_core::PCWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMClearAllRights(hissuancelicense: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCloseHandle(handle: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMClosePubHandle(hpub: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCloseQueryHandle(hquery: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCloseSession(hsession: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *const ::windows_sys_core::PWSTR, pcchain: *mut u32, wszchain: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows_sys_core::PCWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateClientSession(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: ::windows_sys_core::PCWSTR, wszgroupid: ::windows_sys_core::PCWSTR, phclient: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows_sys_core::PCWSTR, hauxlib: u32, wszauxplug: ::windows_sys_core::PCWSTR, phdecryptor: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows_sys_core::PCWSTR, hauxlib: u32, wszauxplug: ::windows_sys_core::PCWSTR, phencryptor: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows_sys_core::PCWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows_sys_core::PCWSTR, phenablingprincipal: *mut u32) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: ::windows_sys_core::PCWSTR, wszreferralinfourl: ::windows_sys_core::PCWSTR, howner: u32, wszissuancelicense: ::windows_sys_core::PCWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows_sys_core::PCWSTR, phlicensestorage: *mut u32) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateRight(wszrightname: ::windows_sys_core::PCWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows_sys_core::PWSTR, pwszextendedinfovalue: *const ::windows_sys_core::PWSTR, phright: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMCreateUser(wszusername: ::windows_sys_core::PCWSTR, wszuserid: ::windows_sys_core::PCWSTR, wszuseridtype: ::windows_sys_core::PCWSTR, phuser: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMDecode(wszalgid: ::windows_sys_core::PCWSTR, wszencodedstring: ::windows_sys_core::PCWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMDeconstructCertificateChain(wszchain: ::windows_sys_core::PCWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMEncode(wszalgid: ::windows_sys_core::PCWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows_sys_core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows_sys_core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows_sys_core::PCWSTR, pcattributes: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows_sys_core::PCWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows_sys_core::PCWSTR, pcsubobjects: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetCertificateChainCount(wszchain: ::windows_sys_core::PCWSTR, pccertcount: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows_sys_core::PCWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetInfo(handle: u32, wszattribute: ::windows_sys_core::PCWSTR, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows_sys_core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows_sys_core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows_sys_core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows_sys_core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows_sys_core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows_sys_core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows_sys_core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows_sys_core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows_sys_core::PCWSTR, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows_sys_core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows_sys_core::PWSTR, puurllength: *mut u32, wszrl: ::windows_sys_core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows_sys_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows_sys_core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows_sys_core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows_sys_core::PWSTR, pupathlen: *mut u32, wszpath: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows_sys_core::PCWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows_sys_core::PCWSTR, wszclientlicensorcertificate: ::windows_sys_core::PCWSTR, pfncallback: DRMCALLBACK, wszurl: ::windows_sys_core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: ::windows_sys_core::PCWSTR, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows_sys_core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows_sys_core::PCWSTR, pcattributes: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows_sys_core::PCWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows_sys_core::PCWSTR, pcsubobjects: *mut u32) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows_sys_core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows_sys_core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows_sys_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows_sys_core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows_sys_core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows_sys_core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows_sys_core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows_sys_core::PCWSTR, wszmanifestcredentials: ::windows_sys_core::PCWSTR, wszmachinecredentials: ::windows_sys_core::PCWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMIsWindowProtected(hwnd: super::super::Foundation::HWND, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows_sys_core::HRESULT;
    pub fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows_sys_core::PCWSTR, wszcredentials: ::windows_sys_core::PCWSTR, phlibrary: *mut u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMParseUnboundLicense(wszcertificate: ::windows_sys_core::PCWSTR, phqueryroot: *mut u32) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterContent(fregister: super::super::Foundation::BOOL) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterProtectedWindow(henv: u32, hwnd: super::super::Foundation::HWND) -> ::windows_sys_core::HRESULT;
    pub fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMRepair() -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszname: ::windows_sys_core::PCWSTR, wszvalue: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    pub fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows_sys_core::PCWSTR, wszcontentidtype: ::windows_sys_core::PCWSTR, wszskuid: ::windows_sys_core::PCWSTR, wszskuidtype: ::windows_sys_core::PCWSTR, wszcontenttype: ::windows_sys_core::PCWSTR, wszcontentname: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, lcid: u32, wszname: ::windows_sys_core::PCWSTR, wszdescription: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszid: ::windows_sys_core::PCWSTR, wszidtype: ::windows_sys_core::PCWSTR, wszurl: ::windows_sys_core::PCWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: ::windows_sys_core::PCWSTR, wszpublickey: ::windows_sys_core::PCWSTR) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: super::super::Foundation::BOOL, fexclusion: super::super::Foundation::BOOL, wszname: ::windows_sys_core::PCWSTR, wszminversion: ::windows_sys_core::PCWSTR, wszmaxversion: ::windows_sys_core::PCWSTR, wszpublickey: ::windows_sys_core::PCWSTR, wszdigestalgorithm: ::windows_sys_core::PCWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows_sys_core::HRESULT;
    pub fn DRMVerify(wszdata: ::windows_sys_core::PCWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows_sys_core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows_sys_core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows_sys_core::PWSTR) -> ::windows_sys_core::HRESULT;
}
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
pub type DRMATTESTTYPE = i32;
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = 0i32;
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = 1i32;
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
#[repr(C)]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: ::windows_sys_core::PWSTR,
    pub wszRightsGroup: ::windows_sys_core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: ::windows_sys_core::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DRMBOUNDLICENSEPARAMS {}
impl ::core::clone::Clone for DRMBOUNDLICENSEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
pub type DRMCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: ::windows_sys_core::HRESULT, param2: *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_sys_core::HRESULT>;
pub const DRMCALLBACKVERSION: u32 = 1u32;
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
pub type DRMENCODINGTYPE = i32;
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = 0i32;
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = 1i32;
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = 2i32;
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = 3i32;
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = 4i32;
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = 5i32;
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
pub type DRMGLOBALOPTIONS = i32;
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = 0i32;
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = 1i32;
pub const DRMHANDLE_INVALID: u32 = 0u32;
pub const DRMHSESSION_INVALID: u32 = 0u32;
#[repr(C)]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: ::windows_sys_core::PWSTR,
    pub wszID: ::windows_sys_core::PWSTR,
}
impl ::core::marker::Copy for DRMID {}
impl ::core::clone::Clone for DRMID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRMIDVERSION: u32 = 0u32;
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
pub type DRMSECURITYPROVIDERTYPE = i32;
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = 0i32;
pub type DRMSPECTYPE = i32;
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = 0i32;
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = 1i32;
pub type DRMTIMETYPE = i32;
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = 0i32;
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = 1i32;
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
#[repr(C)]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: ::windows_sys_core::PWSTR,
    pub wszURL: ::windows_sys_core::PWSTR,
}
impl ::core::marker::Copy for DRM_ACTSERV_INFO {}
impl ::core::clone::Clone for DRM_ACTSERV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
pub const DRM_AILT_CANCEL: u32 = 4u32;
pub const DRM_AILT_NONSILENT: u32 = 1u32;
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
pub const DRM_AL_CANCEL: u32 = 4u32;
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
pub const DRM_AL_NONSILENT: u32 = 1u32;
pub const DRM_AL_NOPERSIST: u32 = 2u32;
pub const DRM_AL_NOUI: u32 = 16u32;
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
#[repr(C)]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl ::core::marker::Copy for DRM_CLIENT_VERSION_INFO {}
impl ::core::clone::Clone for DRM_CLIENT_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: &str = "PassportAuthProvider";
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: &str = "WindowsAuthProvider";
pub type DRM_DISTRIBUTION_POINT_INFO = i32;
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = 0i32;
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = 1i32;
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = 2i32;
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
pub const DRM_EL_EUL: u32 = 32u32;
pub const DRM_EL_EUL_LID: u32 = 64u32;
pub const DRM_EL_EXPIRED: u32 = 4096u32;
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
pub const DRM_EL_MACHINE: u32 = 1u32;
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
#[repr(C)]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: ::windows_sys_core::PWSTR,
    pub wszLocalFilename: ::windows_sys_core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: ::windows_sys_core::PWSTR,
}
impl ::core::marker::Copy for DRM_LICENSE_ACQ_DATA {}
impl ::core::clone::Clone for DRM_LICENSE_ACQ_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
pub const DRM_REUSE_KEY: u32 = 64u32;
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
pub const DRM_SIGN_CANCEL: u32 = 4u32;
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
pub const DRM_SIGN_ONLINE: u32 = 1u32;
pub type DRM_STATUS_MSG = i32;
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = 0i32;
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = 1i32;
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = 2i32;
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = 3i32;
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = 4i32;
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = 5i32;
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = 6i32;
pub type DRM_USAGEPOLICY_TYPE = i32;
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = 0i32;
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = 1i32;
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = 2i32;
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = 3i32;
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;