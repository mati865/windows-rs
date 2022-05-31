#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25de8fd0_1c5b_11e1_bddb_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerSavingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PowerSavingMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PowerSavingModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePowerSavingModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x596236cf_1918_4551_a466_c51aae373bf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
pub struct PowerManager;
impl PowerManager {
    pub fn PowerSavingMode() -> ::windows_core::Result<PowerSavingMode> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PowerSavingMode>::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PowerSavingMode>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PowerSavingModeChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows_core::IInspectable>>>(changehandler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingModeChanged)(::windows_core::Interface::as_raw(this), changehandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePowerSavingModeChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePowerSavingModeChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn PowerSavingModeEnabled() -> ::windows_core::Result<bool> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PowerSavingModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PowerManager, IPowerManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPowerManagerStatics2<R, F: FnOnce(&IPowerManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PowerManager, IPowerManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSavingMode {}
impl ::core::clone::Clone for PowerSavingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSavingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PowerSavingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerSavingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSavingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PowerSavingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}