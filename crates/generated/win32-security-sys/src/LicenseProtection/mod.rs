#[link(name = "windows")]
extern "system" {
    pub fn RegisterLicenseKeyWithExpiration(licensekey: ::windows_sys_core::PCWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_sys_core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateLicenseKeyProtection(licensekey: ::windows_sys_core::PCWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_sys_core::HRESULT;
}
pub type LicenseProtectionStatus = i32;
pub const Success: LicenseProtectionStatus = 0i32;
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;