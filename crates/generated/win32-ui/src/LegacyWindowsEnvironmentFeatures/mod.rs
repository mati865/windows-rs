pub const EVCCBF_LASTNOTIFICATION: u32 = 1u32;
pub const EVCF_DONTSHOWIFZERO: u32 = 16u32;
pub const EVCF_ENABLEBYDEFAULT: u32 = 2u32;
pub const EVCF_ENABLEBYDEFAULT_AUTO: u32 = 8u32;
pub const EVCF_HASSETTINGS: u32 = 1u32;
pub const EVCF_OUTOFDISKSPACE: u32 = 64u32;
pub const EVCF_REMOVEFROMLIST: u32 = 4u32;
pub const EVCF_SETTINGSMODE: u32 = 32u32;
pub const EVCF_SYSTEMAUTORUN: u32 = 256u32;
pub const EVCF_USERCONSENTOBTAINED: u32 = 128u32;
#[repr(transparent)]
pub struct IADesktopP2(::windows_core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn ReReadWallpaper(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReReadWallpaper)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetADObjectFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwflags), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn MakeDynamicChanges<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::Ole::IOleObject>>(&self, poleobj: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MakeDynamicChanges)(::windows_core::Interface::as_raw(self), poleobj.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IADesktopP2> for ::windows_core::IUnknown {
    fn from(value: IADesktopP2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IADesktopP2> for ::windows_core::IUnknown {
    fn from(value: &IADesktopP2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IADesktopP2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IADesktopP2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IADesktopP2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IADesktopP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADesktopP2 {}
impl ::core::fmt::Debug for IADesktopP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADesktopP2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IADesktopP2 {
    type Vtable = IADesktopP2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb22754e2_4574_11d1_9888_006097deacf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ReReadWallpaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub MakeDynamicChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poleobj: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    MakeDynamicChanges: usize,
}
#[repr(transparent)]
pub struct IActiveDesktopP(::windows_core::IUnknown);
impl IActiveDesktopP {
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSafeMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnsureUpdateHTML)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetScheme<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszschemename: Param0, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScheme)(::windows_core::Interface::as_raw(self), pwszschemename.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetScheme(&self, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScheme)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszschemename), ::core::mem::transmute(pdwcchbuffer), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IActiveDesktopP> for ::windows_core::IUnknown {
    fn from(value: IActiveDesktopP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveDesktopP> for ::windows_core::IUnknown {
    fn from(value: &IActiveDesktopP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActiveDesktopP {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActiveDesktopP {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveDesktopP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveDesktopP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveDesktopP {}
impl ::core::fmt::Debug for IActiveDesktopP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveDesktopP").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IActiveDesktopP {
    type Vtable = IActiveDesktopP_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetSafeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IBriefcaseInitiator(::windows_core::IUnknown);
impl IBriefcaseInitiator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsMonikerInBriefcase<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::Com::IMoniker>>(&self, pmk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsMonikerInBriefcase)(::windows_core::Interface::as_raw(self), pmk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBriefcaseInitiator> for ::windows_core::IUnknown {
    fn from(value: IBriefcaseInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBriefcaseInitiator> for ::windows_core::IUnknown {
    fn from(value: &IBriefcaseInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBriefcaseInitiator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBriefcaseInitiator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBriefcaseInitiator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBriefcaseInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBriefcaseInitiator {}
impl ::core::fmt::Debug for IBriefcaseInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBriefcaseInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IBriefcaseInitiator {
    type Vtable = IBriefcaseInitiator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99180164_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBriefcaseInitiator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsMonikerInBriefcase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsMonikerInBriefcase: usize,
}
#[repr(transparent)]
pub struct IEmptyVolumeCache(::windows_core::IUnknown);
impl IEmptyVolumeCache {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows_core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSpaceUsed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    pub unsafe fn Purge<'a, Param1: ::windows_core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Purge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowProperties)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Deactivate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEmptyVolumeCache> for ::windows_core::IUnknown {
    fn from(value: IEmptyVolumeCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache> for ::windows_core::IUnknown {
    fn from(value: &IEmptyVolumeCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEmptyVolumeCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEmptyVolumeCache {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache {}
impl ::core::fmt::Debug for IEmptyVolumeCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEmptyVolumeCache {
    type Vtable = IEmptyVolumeCache_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fce5227_04da_11d1_a004_00805f8abe06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetSpaceUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowProperties: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEmptyVolumeCache2(::windows_core::IUnknown);
impl IEmptyVolumeCache2 {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetSpaceUsed<'a, Param1: ::windows_core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, pdwlspaceused: *mut u64, picb: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSpaceUsed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwlspaceused), picb.into_param().abi()).ok()
    }
    pub unsafe fn Purge<'a, Param1: ::windows_core::IntoParam<'a, IEmptyVolumeCacheCallBack>>(&self, dwlspacetofree: u64, picb: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Purge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlspacetofree), picb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ShowProperties)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Deactivate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn InitializeEx<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::Registry::HKEY>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, hkregkey: Param0, pcwszvolume: Param1, pcwszkeyname: Param2, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeEx)(::windows_core::Interface::as_raw(self), hkregkey.into_param().abi(), pcwszvolume.into_param().abi(), pcwszkeyname.into_param().abi(), ::core::mem::transmute(ppwszdisplayname), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(ppwszbtntext), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for ::windows_core::IUnknown {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for ::windows_core::IUnknown {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: IEmptyVolumeCache2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCache2> for IEmptyVolumeCache {
    fn from(value: &IEmptyVolumeCache2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEmptyVolumeCache> for IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEmptyVolumeCache> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEmptyVolumeCache> for &'a IEmptyVolumeCache2 {
    fn into_param(self) -> ::windows_core::Param<'a, IEmptyVolumeCache> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCache2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache2 {}
impl ::core::fmt::Debug for IEmptyVolumeCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEmptyVolumeCache2 {
    type Vtable = IEmptyVolumeCache2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02b7e3ba_4db3_11d2_b2d9_00c04f8eec8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCache2_Vtbl {
    pub base__: IEmptyVolumeCache_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub InitializeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, pcwszkeyname: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    InitializeEx: usize,
}
#[repr(transparent)]
pub struct IEmptyVolumeCacheCallBack(::windows_core::IUnknown);
impl IEmptyVolumeCacheCallBack {
    pub unsafe fn ScanProgress<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScanProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlspaceused), ::core::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
    pub unsafe fn PurgeProgress<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PurgeProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlspacefreed), ::core::mem::transmute(dwlspacetofree), ::core::mem::transmute(dwflags), pcwszstatus.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEmptyVolumeCacheCallBack> for ::windows_core::IUnknown {
    fn from(value: IEmptyVolumeCacheCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEmptyVolumeCacheCallBack> for ::windows_core::IUnknown {
    fn from(value: &IEmptyVolumeCacheCallBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEmptyVolumeCacheCallBack {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEmptyVolumeCacheCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCacheCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCacheCallBack {}
impl ::core::fmt::Debug for IEmptyVolumeCacheCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCacheCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEmptyVolumeCacheCallBack {
    type Vtable = IEmptyVolumeCacheCallBack_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e793361_73c6_11d0_8469_00aa00442901);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmptyVolumeCacheCallBack_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ScanProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub PurgeProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReconcilableObject(::windows_core::IUnknown);
impl IReconcilableObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Reconcile<'a, Param0: ::windows_core::IntoParam<'a, IReconcileInitiator>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>, Param7: ::windows_core::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>>(&self, pinitiator: Param0, dwflags: u32, hwndowner: Param2, hwndprogressfeedback: Param3, rgpmkotherinput: &mut [::core::option::Option<super::super::System::Com::IMoniker>], ploutindex: *mut i32, pstgnewresidues: Param7, pvreserved: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reconcile)(::windows_core::Interface::as_raw(self), pinitiator.into_param().abi(), ::core::mem::transmute(dwflags), hwndowner.into_param().abi(), hwndprogressfeedback.into_param().abi(), rgpmkotherinput.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(rgpmkotherinput)), ::core::mem::transmute(ploutindex), pstgnewresidues.into_param().abi(), ::core::mem::transmute(pvreserved)).ok()
    }
    pub unsafe fn GetProgressFeedbackMaxEstimate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProgressFeedbackMaxEstimate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IReconcilableObject> for ::windows_core::IUnknown {
    fn from(value: IReconcilableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcilableObject> for ::windows_core::IUnknown {
    fn from(value: &IReconcilableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReconcilableObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReconcilableObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReconcilableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReconcilableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcilableObject {}
impl ::core::fmt::Debug for IReconcilableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcilableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReconcilableObject {
    type Vtable = IReconcilableObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99180162_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcilableObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Reconcile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiator: ::windows_core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows_core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows_core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Reconcile: usize,
    pub GetProgressFeedbackMaxEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReconcileInitiator(::windows_core::IUnknown);
impl IReconcileInitiator {
    pub unsafe fn SetAbortCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkforabort: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAbortCallback)(::windows_core::Interface::as_raw(self), punkforabort.into_param().abi()).ok()
    }
    pub unsafe fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProgressFeedback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax)).ok()
    }
}
impl ::core::convert::From<IReconcileInitiator> for ::windows_core::IUnknown {
    fn from(value: IReconcileInitiator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReconcileInitiator> for ::windows_core::IUnknown {
    fn from(value: &IReconcileInitiator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReconcileInitiator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReconcileInitiator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReconcileInitiator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReconcileInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcileInitiator {}
impl ::core::fmt::Debug for IReconcileInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcileInitiator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReconcileInitiator {
    type Vtable = IReconcileInitiator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99180161_da16_101a_935c_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconcileInitiator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetAbortCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetProgressFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::HRESULT,
}
pub const REC_E_ABORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147217408i32);
pub const REC_E_INEEDTODOTHEUPDATES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147217404i32);
pub const REC_E_NOCALLBACK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147217407i32);
pub const REC_E_NORESIDUES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147217406i32);
pub const REC_E_TOODIFFERENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147217405i32);
pub const REC_S_IDIDTHEUPDATES: ::windows_core::HRESULT = ::windows_core::HRESULT(266240i32);
pub const REC_S_NOTCOMPLETE: ::windows_core::HRESULT = ::windows_core::HRESULT(266241i32);
pub const REC_S_NOTCOMPLETEBUTPROPAGATE: ::windows_core::HRESULT = ::windows_core::HRESULT(266242i32);
pub const STATEBITS_FLAT: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _reconcilef(pub i32);
pub const RECONCILEF_MAYBOTHERUSER: _reconcilef = _reconcilef(1i32);
pub const RECONCILEF_FEEDBACKWINDOWVALID: _reconcilef = _reconcilef(2i32);
pub const RECONCILEF_NORESIDUESOK: _reconcilef = _reconcilef(4i32);
pub const RECONCILEF_OMITSELFRESIDUE: _reconcilef = _reconcilef(8i32);
pub const RECONCILEF_RESUMERECONCILIATION: _reconcilef = _reconcilef(16i32);
pub const RECONCILEF_YOUMAYDOTHEUPDATES: _reconcilef = _reconcilef(32i32);
pub const RECONCILEF_ONLYYOUWERECHANGED: _reconcilef = _reconcilef(64i32);
pub const ALL_RECONCILE_FLAGS: _reconcilef = _reconcilef(127i32);
impl ::core::marker::Copy for _reconcilef {}
impl ::core::clone::Clone for _reconcilef {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _reconcilef {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _reconcilef {
    type Abi = Self;
}
impl ::core::fmt::Debug for _reconcilef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_reconcilef").field(&self.0).finish()
    }
}