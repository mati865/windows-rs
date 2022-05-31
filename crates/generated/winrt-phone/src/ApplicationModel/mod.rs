pub struct ApplicationProfile;
impl ApplicationProfile {
    pub fn Modes() -> ::windows_core::Result<ApplicationProfileModes> {
        Self::IApplicationProfileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationProfileModes>::zeroed();
            (::windows_core::Interface::vtable(this).Modes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationProfileModes>(result__)
        })
    }
    pub fn IApplicationProfileStatics<R, F: FnOnce(&IApplicationProfileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ApplicationProfile, IApplicationProfileStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl ::core::marker::Copy for ApplicationProfileModes {}
impl ::core::clone::Clone for ApplicationProfileModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationProfileModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ApplicationProfileModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationProfileModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationProfileModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ApplicationProfileModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationProfileModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationProfileModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationProfileModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationProfileModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationProfileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationProfileStatics {
    type Vtable = IApplicationProfileStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationProfileStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Modes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows_core::HRESULT,
}