#[cfg(feature = "Power")]
pub mod Power;
#[cfg(feature = "Profile")]
pub mod Profile;
#[cfg(feature = "UserProfile")]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemProtectionStatics {
    type Vtable = ISystemProtectionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScreenLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemProtectionUnlockStatics {
    type Vtable = ISystemProtectionUnlockStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub struct SystemProtection;
impl SystemProtection {
    pub fn ScreenLocked() -> ::windows_core::Result<bool> {
        Self::ISystemProtectionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenLocked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn RequestScreenUnlock() -> ::windows_core::Result<()> {
        Self::ISystemProtectionUnlockStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RequestScreenUnlock)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ISystemProtectionStatics<R, F: FnOnce(&ISystemProtectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemProtection, ISystemProtectionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISystemProtectionUnlockStatics<R, F: FnOnce(&ISystemProtectionUnlockStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemProtection, ISystemProtectionUnlockStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}