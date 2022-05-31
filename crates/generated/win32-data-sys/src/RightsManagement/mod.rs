#[link(name = "windows")]
extern "system" {
    pub fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows_core_sys::PCWSTR, wszurl: ::windows_core_sys::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *const ::windows_core_sys::PWSTR, wszurl: ::windows_core_sys::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows_core_sys::PCWSTR, wszrequestedrights: ::windows_core_sys::PCWSTR, wszcustomdata: ::windows_core_sys::PCWSTR, wszurl: ::windows_core_sys::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    pub fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMAttest(henablingprincipal: u32, wszdata: ::windows_core_sys::PCWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMClearAllRights(hissuancelicense: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCloseHandle(handle: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMClosePubHandle(hpub: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCloseQueryHandle(hquery: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCloseSession(hsession: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *const ::windows_core_sys::PWSTR, pcchain: *mut u32, wszchain: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows_core_sys::PCWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateClientSession(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: ::windows_core_sys::PCWSTR, wszgroupid: ::windows_core_sys::PCWSTR, phclient: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows_core_sys::PCWSTR, hauxlib: u32, wszauxplug: ::windows_core_sys::PCWSTR, phdecryptor: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows_core_sys::PCWSTR, hauxlib: u32, wszauxplug: ::windows_core_sys::PCWSTR, phencryptor: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows_core_sys::PCWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows_core_sys::PCWSTR, phenablingprincipal: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateIssuanceLicense(psttimefrom: *mut ::win32_foundation_sys::SYSTEMTIME, psttimeuntil: *mut ::win32_foundation_sys::SYSTEMTIME, wszreferralinfoname: ::windows_core_sys::PCWSTR, wszreferralinfourl: ::windows_core_sys::PCWSTR, howner: u32, wszissuancelicense: ::windows_core_sys::PCWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows_core_sys::PCWSTR, phlicensestorage: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateRight(wszrightname: ::windows_core_sys::PCWSTR, pstfrom: *mut ::win32_foundation_sys::SYSTEMTIME, pstuntil: *mut ::win32_foundation_sys::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows_core_sys::PWSTR, pwszextendedinfovalue: *const ::windows_core_sys::PWSTR, phright: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMCreateUser(wszusername: ::windows_core_sys::PCWSTR, wszuserid: ::windows_core_sys::PCWSTR, wszuseridtype: ::windows_core_sys::PCWSTR, phuser: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMDecode(wszalgid: ::windows_core_sys::PCWSTR, wszencodedstring: ::windows_core_sys::PCWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMDeconstructCertificateChain(wszchain: ::windows_core_sys::PCWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMEncode(wszalgid: ::windows_core_sys::PCWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut ::win32_foundation_sys::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows_core_sys::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows_core_sys::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows_core_sys::PCWSTR, pcattributes: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows_core_sys::PCWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows_core_sys::PCWSTR, pcsubobjects: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetCertificateChainCount(wszchain: ::windows_core_sys::PCWSTR, pccertcount: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows_core_sys::PCWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetInfo(handle: u32, wszattribute: ::windows_core_sys::PCWSTR, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut ::win32_foundation_sys::SYSTEMTIME, psttimeuntil: *mut ::win32_foundation_sys::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows_core_sys::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows_core_sys::PWSTR, phowner: *mut u32, pfofficial: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows_core_sys::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows_core_sys::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows_core_sys::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows_core_sys::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows_core_sys::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows_core_sys::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows_core_sys::PCWSTR, ppfnprocaddress: *mut ::win32_foundation_sys::FARPROC) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows_core_sys::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows_core_sys::PWSTR, puurllength: *mut u32, wszrl: ::windows_core_sys::PWSTR, pstfrequency: *mut ::win32_foundation_sys::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows_core_sys::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows_core_sys::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows_core_sys::PWSTR, pstfrom: *mut ::win32_foundation_sys::SYSTEMTIME, pstuntil: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows_core_sys::PWSTR, pupathlen: *mut u32, wszpath: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows_core_sys::PCWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows_core_sys::PCWSTR, wszclientlicensorcertificate: ::windows_core_sys::PCWSTR, pfncallback: DRMCALLBACK, wszurl: ::windows_core_sys::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: ::windows_core_sys::PCWSTR, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut ::win32_foundation_sys::SYSTEMTIME) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows_core_sys::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows_core_sys::PCWSTR, pcattributes: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows_core_sys::PCWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows_core_sys::PCWSTR, pcsubobjects: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut ::win32_foundation_sys::BOOL, punamelength: *mut u32, wszname: ::windows_core_sys::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows_core_sys::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows_core_sys::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows_core_sys::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows_core_sys::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows_core_sys::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows_core_sys::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows_core_sys::PCWSTR, wszmanifestcredentials: ::windows_core_sys::PCWSTR, wszmachinecredentials: ::windows_core_sys::PCWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows_core_sys::HRESULT;
    pub fn DRMIsWindowProtected(hwnd: ::win32_foundation_sys::HWND, pfprotected: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows_core_sys::PCWSTR, wszcredentials: ::windows_core_sys::PCWSTR, phlibrary: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMParseUnboundLicense(wszcertificate: ::windows_core_sys::PCWSTR, phqueryroot: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMRegisterContent(fregister: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn DRMRegisterProtectedWindow(henv: u32, hwnd: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
    pub fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMRepair() -> ::windows_core_sys::HRESULT;
    pub fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: ::win32_foundation_sys::BOOL, wszname: ::windows_core_sys::PCWSTR, wszvalue: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows_core_sys::PCWSTR, wszcontentidtype: ::windows_core_sys::PCWSTR, wszskuid: ::windows_core_sys::PCWSTR, wszskuidtype: ::windows_core_sys::PCWSTR, wszcontenttype: ::windows_core_sys::PCWSTR, wszcontentname: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: ::win32_foundation_sys::BOOL, lcid: u32, wszname: ::windows_core_sys::PCWSTR, wszdescription: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: ::win32_foundation_sys::BOOL, wszid: ::windows_core_sys::PCWSTR, wszidtype: ::windows_core_sys::PCWSTR, wszurl: ::windows_core_sys::PCWSTR, pstfrequency: *mut ::win32_foundation_sys::SYSTEMTIME, wszname: ::windows_core_sys::PCWSTR, wszpublickey: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: ::win32_foundation_sys::BOOL, fexclusion: ::win32_foundation_sys::BOOL, wszname: ::windows_core_sys::PCWSTR, wszminversion: ::windows_core_sys::PCWSTR, wszmaxversion: ::windows_core_sys::PCWSTR, wszpublickey: ::windows_core_sys::PCWSTR, wszdigestalgorithm: ::windows_core_sys::PCWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows_core_sys::HRESULT;
    pub fn DRMVerify(wszdata: ::windows_core_sys::PCWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows_core_sys::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows_core_sys::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
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
    pub wszRightsRequested: ::windows_core_sys::PWSTR,
    pub wszRightsGroup: ::windows_core_sys::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: ::windows_core_sys::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DRMBOUNDLICENSEPARAMS {}
impl ::core::clone::Clone for DRMBOUNDLICENSEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
pub type DRMCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: ::windows_core_sys::HRESULT, param2: *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT>;
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
    pub wszIDType: ::windows_core_sys::PWSTR,
    pub wszID: ::windows_core_sys::PWSTR,
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
    pub wszPubKey: ::windows_core_sys::PWSTR,
    pub wszURL: ::windows_core_sys::PWSTR,
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
    pub wszURL: ::windows_core_sys::PWSTR,
    pub wszLocalFilename: ::windows_core_sys::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: ::windows_core_sys::PWSTR,
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
