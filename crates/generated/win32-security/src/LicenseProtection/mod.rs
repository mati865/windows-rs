#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LicenseProtectionStatus(pub i32);
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
impl ::core::marker::Copy for LicenseProtectionStatus {}
impl ::core::clone::Clone for LicenseProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LicenseProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LicenseProtectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LicenseProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseProtectionStatus").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(licensekey: Param0, validityindays: u32) -> ::windows_core::Result<LicenseProtectionStatus> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterLicenseKeyWithExpiration(licensekey: ::windows_core::PCWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<LicenseProtectionStatus>::zeroed();
        RegisterLicenseKeyWithExpiration(licensekey.into_param().abi(), ::core::mem::transmute(validityindays), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<LicenseProtectionStatus>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(licensekey: Param0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidateLicenseKeyProtection(licensekey: ::windows_core::PCWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_core::HRESULT;
        }
        ValidateLicenseKeyProtection(licensekey.into_param().abi(), ::core::mem::transmute(notvalidbefore), ::core::mem::transmute(notvalidafter), ::core::mem::transmute(status)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}