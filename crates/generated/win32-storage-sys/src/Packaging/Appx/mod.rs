#[link(name = "windows")]
extern "system" {
    pub fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows_core_sys::HRESULT;
    pub fn AddPackageDependency(packagedependencyid: ::windows_core_sys::PCWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn AppPolicyGetClrCompat(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyClrCompat) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetCreateFileAccess(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetLifecycleManagement(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetMediaFoundationCodecLoading(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetProcessTerminationMethod(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetShowDeveloperDiagnostic(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetThreadInitializationType(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn AppPolicyGetWindowingModel(processtoken: ::win32_foundation_sys::HANDLE, policy: *mut AppPolicyWindowingModel) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn CheckIsMSIXPackage(packagefullname: ::windows_core_sys::PCWSTR, ismsixpackage: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn CreatePackageVirtualizationContext(packagefamilyname: ::windows_core_sys::PCWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_core_sys::HRESULT;
    pub fn DeactivatePackageVirtualizationContext(cookie: usize);
    pub fn DeletePackageDependency(packagedependencyid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_core_sys::HRESULT;
    pub fn FindPackagesByPackageFamily(packagefamilyname: ::windows_core_sys::PCWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut ::windows_core_sys::PWSTR, bufferlength: *mut u32, buffer: ::windows_core_sys::PWSTR, packageproperties: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn FormatApplicationUserModelId(packagefamilyname: ::windows_core_sys::PCWSTR, packagerelativeapplicationid: ::windows_core_sys::PCWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetApplicationUserModelId(hprocess: ::win32_foundation_sys::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetApplicationUserModelIdFromToken(token: ::win32_foundation_sys::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackagePath(pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
    pub fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageFamilyName(hprocess: ::win32_foundation_sys::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageFamilyNameFromToken(token: ::win32_foundation_sys::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageFullName(hprocess: ::win32_foundation_sys::HANDLE, packagefullnamelength: *mut u32, packagefullname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageFullNameFromToken(token: ::win32_foundation_sys::HANDLE, packagefullnamelength: *mut u32, packagefullname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageId(hprocess: ::win32_foundation_sys::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackagePathByFullName(packagefullname: ::windows_core_sys::PCWSTR, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackagePathByFullName2(packagefullname: ::windows_core_sys::PCWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetPackagesByPackageFamily(packagefamilyname: ::windows_core_sys::PCWSTR, count: *mut u32, packagefullnames: *mut ::windows_core_sys::PWSTR, bufferlength: *mut u32, buffer: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetProcessesInVirtualizationContext(packagefamilyname: ::windows_core_sys::PCWSTR, count: *mut u32, processes: *mut *mut ::win32_foundation_sys::HANDLE) -> ::windows_core_sys::HRESULT;
    pub fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: ::windows_core_sys::PCWSTR, packagefullname: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn GetStagedPackageOrigin(packagefullname: ::windows_core_sys::PCWSTR, origin: *mut PackageOrigin) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetStagedPackagePathByFullName(packagefullname: ::windows_core_sys::PCWSTR, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn GetStagedPackagePathByFullName2(packagefullname: ::windows_core_sys::PCWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn OpenPackageInfoByFullName(packagefullname: ::windows_core_sys::PCWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn OpenPackageInfoByFullNameForUser(usersid: ::win32_foundation_sys::PSID, packagefullname: ::windows_core_sys::PCWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn PackageFamilyNameFromFullName(packagefullname: ::windows_core_sys::PCWSTR, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn PackageIdFromFullName(packagefullname: ::windows_core_sys::PCWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: ::windows_core_sys::PCWSTR, packagenamelength: *mut u32, packagename: ::windows_core_sys::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn ParseApplicationUserModelId(applicationusermodelid: ::windows_core_sys::PCWSTR, packagefamilynamelength: *mut u32, packagefamilyname: ::windows_core_sys::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: ::windows_core_sys::PWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
    pub fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows_core_sys::HRESULT;
    pub fn TryCreatePackageDependency(user: ::win32_foundation_sys::PSID, packagefamilyname: ::windows_core_sys::PCWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: ::windows_core_sys::PCWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn VerifyApplicationUserModelId(applicationusermodelid: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn VerifyPackageFamilyName(packagefamilyname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn VerifyPackageFullName(packagefullname: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn VerifyPackageId(packageid: *const PACKAGE_ID) -> ::win32_foundation_sys::WIN32_ERROR;
    pub fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: ::windows_core_sys::PCWSTR) -> ::win32_foundation_sys::WIN32_ERROR;
}
pub type APPX_BUNDLE_FOOTPRINT_FILE_TYPE = i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 1i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = 2i32;
pub type APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 0i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = 1i32;
pub type APPX_CAPABILITIES = u32;
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = 1u32;
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = 2u32;
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = 4u32;
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = 8u32;
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = 16u32;
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = 32u32;
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = 64u32;
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = 128u32;
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = 256u32;
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = 512u32;
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = 1024u32;
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = 2048u32;
pub type APPX_CAPABILITY_CLASS_TYPE = i32;
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = 0i32;
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = 1i32;
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = 2i32;
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = 4i32;
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = 7i32;
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = 8i32;
pub type APPX_COMPRESSION_OPTION = i32;
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = 0i32;
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = 1i32;
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = 2i32;
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = 3i32;
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = 4i32;
#[repr(C)]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for APPX_ENCRYPTED_EXEMPTIONS {}
impl ::core::clone::Clone for APPX_ENCRYPTED_EXEMPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APPX_ENCRYPTED_PACKAGE_OPTIONS = u32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = 0u32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = 1u32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows_core_sys::PCWSTR,
    pub useDiffusion: ::win32_foundation_sys::BOOL,
    pub blockMapHashAlgorithm: ::win32_system_sys::Com::IUri,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows_core_sys::PCWSTR,
    pub blockMapHashAlgorithm: ::win32_system_sys::Com::IUri,
    pub options: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APPX_FOOTPRINT_FILE_TYPE = i32;
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = 0i32;
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = 1i32;
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = 2i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = 3i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = 4i32;
#[repr(C)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl ::core::marker::Copy for APPX_KEY_INFO {}
impl ::core::clone::Clone for APPX_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APPX_PACKAGE_ARCHITECTURE = i32;
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = 0i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = 12i32;
pub type APPX_PACKAGE_ARCHITECTURE2 = i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = 0i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 12i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = 14i32;
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = 65535i32;
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 0u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 1u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = 2u32;
pub type APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: ::win32_foundation_sys::BOOL,
    pub hashMethod: ::win32_system_sys::Com::IUri,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_PACKAGE_SETTINGS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: ::win32_system_sys::Com::IStream,
    pub fileName: ::windows_core_sys::PCWSTR,
    pub contentType: ::windows_core_sys::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
pub type APPX_PACKAGING_CONTEXT_CHANGE_TYPE = i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 0i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 1i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 2i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = 3i32;
pub type AddPackageDependencyOptions = i32;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0i32;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1i32;
pub type AppPolicyClrCompat = i32;
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = 0i32;
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = 1i32;
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = 2i32;
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = 3i32;
pub type AppPolicyCreateFileAccess = i32;
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = 0i32;
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = 1i32;
pub type AppPolicyLifecycleManagement = i32;
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = 0i32;
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = 1i32;
pub type AppPolicyMediaFoundationCodecLoading = i32;
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = 0i32;
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = 1i32;
pub type AppPolicyProcessTerminationMethod = i32;
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = 0i32;
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = 1i32;
pub type AppPolicyShowDeveloperDiagnostic = i32;
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = 0i32;
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = 1i32;
pub type AppPolicyThreadInitializationType = i32;
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = 0i32;
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = 1i32;
pub type AppPolicyWindowingModel = i32;
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = 0i32;
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = 1i32;
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = 2i32;
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = 3i32;
pub const AppxBundleFactory: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 932054086, data2: 21380, data3: 17335, data4: [136, 119, 231, 219, 221, 136, 52, 70] };
pub const AppxEncryptionFactory: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3697692637, data2: 55400, data3: 18158, data4: [135, 128, 141, 25, 108, 183, 57, 247] };
pub const AppxFactory: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1480761664, data2: 65439, data3: 16742, data4: [143, 92, 98, 245, 183, 176, 199, 129] };
pub const AppxPackageEditor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 4026856138, data2: 44732, data3: 19213, data4: [191, 88, 229, 22, 213, 188, 192, 171] };
pub const AppxPackagingDiagnosticEventSinkManager: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1355418182, data2: 5512, data3: 16737, data4: [142, 210, 239, 158, 70, 156, 237, 93] };
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = 1i32;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2i32;
pub type DX_FEATURE_LEVEL = i32;
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = 0i32;
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = 1i32;
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = 2i32;
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = 3i32;
pub type IAppxBlockMapBlock = *mut ::core::ffi::c_void;
pub type IAppxBlockMapBlocksEnumerator = *mut ::core::ffi::c_void;
pub type IAppxBlockMapFile = *mut ::core::ffi::c_void;
pub type IAppxBlockMapFilesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxBlockMapReader = *mut ::core::ffi::c_void;
pub type IAppxBundleFactory = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestOptionalBundleInfo = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestOptionalBundleInfoEnumerator = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestPackageInfo = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestPackageInfo2 = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestPackageInfo3 = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestPackageInfo4 = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestPackageInfoEnumerator = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestReader = *mut ::core::ffi::c_void;
pub type IAppxBundleManifestReader2 = *mut ::core::ffi::c_void;
pub type IAppxBundleReader = *mut ::core::ffi::c_void;
pub type IAppxBundleWriter = *mut ::core::ffi::c_void;
pub type IAppxBundleWriter2 = *mut ::core::ffi::c_void;
pub type IAppxBundleWriter3 = *mut ::core::ffi::c_void;
pub type IAppxBundleWriter4 = *mut ::core::ffi::c_void;
pub type IAppxContentGroup = *mut ::core::ffi::c_void;
pub type IAppxContentGroupFilesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxContentGroupMapReader = *mut ::core::ffi::c_void;
pub type IAppxContentGroupMapWriter = *mut ::core::ffi::c_void;
pub type IAppxContentGroupsEnumerator = *mut ::core::ffi::c_void;
pub type IAppxEncryptedBundleWriter = *mut ::core::ffi::c_void;
pub type IAppxEncryptedBundleWriter2 = *mut ::core::ffi::c_void;
pub type IAppxEncryptedBundleWriter3 = *mut ::core::ffi::c_void;
pub type IAppxEncryptedPackageWriter = *mut ::core::ffi::c_void;
pub type IAppxEncryptedPackageWriter2 = *mut ::core::ffi::c_void;
pub type IAppxEncryptionFactory = *mut ::core::ffi::c_void;
pub type IAppxEncryptionFactory2 = *mut ::core::ffi::c_void;
pub type IAppxEncryptionFactory3 = *mut ::core::ffi::c_void;
pub type IAppxEncryptionFactory4 = *mut ::core::ffi::c_void;
pub type IAppxFactory = *mut ::core::ffi::c_void;
pub type IAppxFactory2 = *mut ::core::ffi::c_void;
pub type IAppxFile = *mut ::core::ffi::c_void;
pub type IAppxFilesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestApplication = *mut ::core::ffi::c_void;
pub type IAppxManifestApplicationsEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestCapabilitiesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestDeviceCapabilitiesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestDriverConstraint = *mut ::core::ffi::c_void;
pub type IAppxManifestDriverConstraintsEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestDriverDependenciesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestDriverDependency = *mut ::core::ffi::c_void;
pub type IAppxManifestHostRuntimeDependenciesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestHostRuntimeDependency = *mut ::core::ffi::c_void;
pub type IAppxManifestHostRuntimeDependency2 = *mut ::core::ffi::c_void;
pub type IAppxManifestMainPackageDependenciesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestMainPackageDependency = *mut ::core::ffi::c_void;
pub type IAppxManifestOSPackageDependenciesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestOSPackageDependency = *mut ::core::ffi::c_void;
pub type IAppxManifestOptionalPackageInfo = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageDependenciesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageDependency = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageDependency2 = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageDependency3 = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageId = *mut ::core::ffi::c_void;
pub type IAppxManifestPackageId2 = *mut ::core::ffi::c_void;
pub type IAppxManifestProperties = *mut ::core::ffi::c_void;
pub type IAppxManifestQualifiedResource = *mut ::core::ffi::c_void;
pub type IAppxManifestQualifiedResourcesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestReader = *mut ::core::ffi::c_void;
pub type IAppxManifestReader2 = *mut ::core::ffi::c_void;
pub type IAppxManifestReader3 = *mut ::core::ffi::c_void;
pub type IAppxManifestReader4 = *mut ::core::ffi::c_void;
pub type IAppxManifestReader5 = *mut ::core::ffi::c_void;
pub type IAppxManifestReader6 = *mut ::core::ffi::c_void;
pub type IAppxManifestReader7 = *mut ::core::ffi::c_void;
pub type IAppxManifestResourcesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestTargetDeviceFamiliesEnumerator = *mut ::core::ffi::c_void;
pub type IAppxManifestTargetDeviceFamily = *mut ::core::ffi::c_void;
pub type IAppxPackageEditor = *mut ::core::ffi::c_void;
pub type IAppxPackageReader = *mut ::core::ffi::c_void;
pub type IAppxPackageWriter = *mut ::core::ffi::c_void;
pub type IAppxPackageWriter2 = *mut ::core::ffi::c_void;
pub type IAppxPackageWriter3 = *mut ::core::ffi::c_void;
pub type IAppxPackagingDiagnosticEventSink = *mut ::core::ffi::c_void;
pub type IAppxPackagingDiagnosticEventSinkManager = *mut ::core::ffi::c_void;
pub type IAppxSourceContentGroupMapReader = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::clone::Clone for PACKAGEDEPENDENCY_CONTEXT__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[repr(C, packed(4))]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: ::windows_core_sys::PWSTR,
    pub publisher: ::windows_core_sys::PWSTR,
    pub resourceId: ::windows_core_sys::PWSTR,
    pub publisherId: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PACKAGE_ID {}
impl ::core::clone::Clone for PACKAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: ::windows_core_sys::PWSTR,
    pub packageFullName: ::windows_core_sys::PWSTR,
    pub packageFamilyName: ::windows_core_sys::PWSTR,
    pub packageId: PACKAGE_ID,
}
impl ::core::marker::Copy for PACKAGE_INFO {}
impl ::core::clone::Clone for PACKAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[repr(C)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION {}
impl ::core::clone::Clone for PACKAGE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::clone::Clone for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1i32;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2i32;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0i32;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = 1i32;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2i32;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8i32;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = 16i32;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = 32i32;
pub type PackageOrigin = i32;
pub const PackageOrigin_Unknown: PackageOrigin = 0i32;
pub const PackageOrigin_Unsigned: PackageOrigin = 1i32;
pub const PackageOrigin_Inbox: PackageOrigin = 2i32;
pub const PackageOrigin_Store: PackageOrigin = 3i32;
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = 4i32;
pub const PackageOrigin_DeveloperSigned: PackageOrigin = 5i32;
pub const PackageOrigin_LineOfBusiness: PackageOrigin = 6i32;
pub type PackagePathType = i32;
pub const PackagePathType_Install: PackagePathType = 0i32;
pub const PackagePathType_Mutable: PackagePathType = 1i32;
pub const PackagePathType_Effective: PackagePathType = 2i32;
pub const PackagePathType_MachineExternal: PackagePathType = 3i32;
pub const PackagePathType_UserExternal: PackagePathType = 4i32;
pub const PackagePathType_EffectiveExternal: PackagePathType = 5i32;
#[repr(C)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for _PACKAGE_INFO_REFERENCE {}
impl ::core::clone::Clone for _PACKAGE_INFO_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
