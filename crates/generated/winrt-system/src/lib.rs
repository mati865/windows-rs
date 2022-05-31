
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Display")]
pub mod Display;
#[cfg(feature = "Implementation")]
pub mod Implementation;
#[cfg(feature = "Inventory")]
pub mod Inventory;
#[cfg(feature = "Power")]
pub mod Power;
#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Profile")]
pub mod Profile;
#[cfg(feature = "RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "RemoteSystems")]
pub mod RemoteSystems;
#[cfg(feature = "Threading")]
pub mod Threading;
#[cfg(feature = "Update")]
pub mod Update;
#[cfg(feature = "UserProfile")]
pub mod UserProfile;
#[repr(transparent)]
pub struct AppActivationResult(::windows_core::IUnknown);
impl AppActivationResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppActivationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppActivationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppActivationResult {}
impl ::core::fmt::Debug for AppActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppActivationResult;{6b528900-f46e-4eb0-aa6c-38af557cf9ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
    const IID: ::windows_core::GUID = <IAppActivationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
impl ::core::convert::From<AppActivationResult> for ::windows_core::IUnknown {
    fn from(value: AppActivationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows_core::IUnknown {
    fn from(value: &AppActivationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppActivationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppActivationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppActivationResult> for ::windows_core::IInspectable {
    fn from(value: AppActivationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationResult> for ::windows_core::IInspectable {
    fn from(value: &AppActivationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppActivationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppActivationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[repr(transparent)]
pub struct AppDiagnosticInfo(::windows_core::IUnknown);
impl AppDiagnosticInfo {
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows_core::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResourceGroups(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows_core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetResourceGroups)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupInfo>>(result__)
        }
    }
    pub fn CreateResourceGroupWatcher(&self) -> ::windows_core::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows_core::Interface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceGroupWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupInfoWatcher>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows_core::Interface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<AppActivationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppDiagnosticInfoWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefamilyname: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForPackageAsync)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppUserModelId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appusermodelid: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForAppUserModelId)(::windows_core::Interface::as_raw(this), appusermodelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>(result__)
        })
    }
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfo {}
impl ::core::fmt::Debug for AppDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfo;{e348a69a-8889-4ca3-be07-d5ffff5f0804})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppDiagnosticInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: AppDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows_core::IUnknown {
    fn from(value: &AppDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: AppDiagnosticInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfo> for ::windows_core::IInspectable {
    fn from(value: &AppDiagnosticInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppDiagnosticInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcher(::windows_core::IUnknown);
impl AppDiagnosticInfoWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppDiagnosticInfoWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppDiagnosticInfoWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcher {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcher;{75575070-01d3-489a-9325-52f9cc6ede0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IAppDiagnosticInfoWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows_core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows_core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcher> for ::windows_core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcher> for ::windows_core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppDiagnosticInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[repr(transparent)]
pub struct AppDiagnosticInfoWatcherEventArgs(::windows_core::IUnknown);
impl AppDiagnosticInfoWatcherEventArgs {
    pub fn AppDiagnosticInfo(&self) -> ::windows_core::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppDiagnosticInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDiagnosticInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDiagnosticInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppDiagnosticInfoWatcherEventArgs;{7017c716-e1da-4c65-99df-046dff5be71a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppDiagnosticInfoWatcherEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDiagnosticInfoWatcherEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppDiagnosticInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDiagnosticInfoWatcherEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppDiagnosticInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppDiagnosticInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppDiagnosticInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppDiagnosticInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppExecutionStateChangeResult(::windows_core::IUnknown);
impl AppExecutionStateChangeResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExecutionStateChangeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExecutionStateChangeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExecutionStateChangeResult {}
impl ::core::fmt::Debug for AppExecutionStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExecutionStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppExecutionStateChangeResult;{6f039bf0-f91b-4df8-ae77-3033ccb69114})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
    const IID: ::windows_core::GUID = <IAppExecutionStateChangeResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows_core::IUnknown {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows_core::IUnknown {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExecutionStateChangeResult> for ::windows_core::IInspectable {
    fn from(value: AppExecutionStateChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExecutionStateChangeResult> for ::windows_core::IInspectable {
    fn from(value: &AppExecutionStateChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExecutionStateChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[repr(transparent)]
pub struct AppMemoryReport(::windows_core::IUnknown);
impl AppMemoryReport {
    pub fn PrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PrivateCommitUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn PeakPrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PeakPrivateCommitUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpectedTotalCommitLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryReport {}
impl ::core::fmt::Debug for AppMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryReport;{6d65339b-4d6f-45bc-9c5e-e49b3ff2758d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = <IAppMemoryReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
impl ::core::convert::From<AppMemoryReport> for ::windows_core::IUnknown {
    fn from(value: AppMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows_core::IUnknown {
    fn from(value: &AppMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppMemoryReport> for ::windows_core::IInspectable {
    fn from(value: AppMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryReport> for ::windows_core::IInspectable {
    fn from(value: &AppMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppMemoryUsageLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppMemoryUsageLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppMemoryUsageLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppMemoryUsageLimitChangingEventArgs(::windows_core::IUnknown);
impl AppMemoryUsageLimitChangingEventArgs {
    pub fn OldLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).OldLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn NewLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).NewLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppMemoryUsageLimitChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppMemoryUsageLimitChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppMemoryUsageLimitChangingEventArgs {}
impl ::core::fmt::Debug for AppMemoryUsageLimitChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLimitChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppMemoryUsageLimitChangingEventArgs;{79f86664-feca-4da5-9e40-2bc63efdc979})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppMemoryUsageLimitChangingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppMemoryUsageLimitChangingEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppMemoryUsageLimitChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppMemoryUsageLimitChangingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppMemoryUsageLimitChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppMemoryUsageLimitChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[repr(transparent)]
pub struct AppResourceGroupBackgroundTaskReport(::windows_core::IUnknown);
impl AppResourceGroupBackgroundTaskReport {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Trigger(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EntryPoint(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EntryPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupBackgroundTaskReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupBackgroundTaskReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupBackgroundTaskReport {}
impl ::core::fmt::Debug for AppResourceGroupBackgroundTaskReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupBackgroundTaskReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupBackgroundTaskReport;{2566e74e-b05d-40c2-9dc1-1a4f039ea120})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupBackgroundTaskReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupBackgroundTaskReport> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupBackgroundTaskReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupBackgroundTaskReport> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupBackgroundTaskReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupBackgroundTaskReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupEnergyQuotaState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppResourceGroupEnergyQuotaState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupEnergyQuotaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupEnergyQuotaState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppResourceGroupExecutionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupExecutionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppResourceGroupInfo(::windows_core::IUnknown);
impl AppResourceGroupInfo {
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn IsShared(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsShared)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBackgroundTaskReports(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBackgroundTaskReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>(result__)
        }
    }
    pub fn GetMemoryReport(&self) -> ::windows_core::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMemoryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupMemoryReport>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProcessDiagnosticInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>(result__)
        }
    }
    pub fn GetStateReport(&self) -> ::windows_core::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStateReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupStateReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartSuspendAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartSuspendAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartResumeAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartResumeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTerminateAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::Interface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartTerminateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfo {}
impl ::core::fmt::Debug for AppResourceGroupInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfo;{b913f77a-e807-49f4-845e-7b8bdcfe8ee7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfo> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfo> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcher(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExecutionStateChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecutionStateChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveExecutionStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppResourceGroupInfoWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupInfoWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcher {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcher;{d9b0a0fd-6e5a-4c72-8b17-09fec4a212bd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcher> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcher> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupInfoWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherEventArgs(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcherEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherEventArgs;{7a787637-6302-4d2f-bf89-1c12d0b2a6b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcherEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupInfoWatcherEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[repr(transparent)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherExecutionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs;{1bdbedd7-fee6-4fd4-98dd-e92a2cc299f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupInfoWatcherExecutionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppResourceGroupInfoWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppResourceGroupMemoryReport(::windows_core::IUnknown);
impl AppResourceGroupMemoryReport {
    pub fn CommitUsageLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).CommitUsageLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn CommitUsageLevel(&self) -> ::windows_core::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppMemoryUsageLevel>::zeroed();
            (::windows_core::Interface::vtable(this).CommitUsageLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppMemoryUsageLevel>(result__)
        }
    }
    pub fn PrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PrivateCommitUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupMemoryReport {}
impl ::core::fmt::Debug for AppResourceGroupMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupMemoryReport;{2c8c06b1-7db1-4c51-a225-7fae2d49e431})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupMemoryReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupMemoryReport> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupMemoryReport> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[repr(transparent)]
pub struct AppResourceGroupStateReport(::windows_core::IUnknown);
impl AppResourceGroupStateReport {
    pub fn ExecutionState(&self) -> ::windows_core::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppResourceGroupExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupExecutionState>(result__)
        }
    }
    pub fn EnergyQuotaState(&self) -> ::windows_core::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppResourceGroupEnergyQuotaState>::zeroed();
            (::windows_core::Interface::vtable(this).EnergyQuotaState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppResourceGroupEnergyQuotaState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppResourceGroupStateReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppResourceGroupStateReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppResourceGroupStateReport {}
impl ::core::fmt::Debug for AppResourceGroupStateReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupStateReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppResourceGroupStateReport;{52849f18-2f70-4236-ab40-d04db0c7b931})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
    const IID: ::windows_core::GUID = <IAppResourceGroupStateReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows_core::IUnknown {
    fn from(value: AppResourceGroupStateReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows_core::IUnknown {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppResourceGroupStateReport> for ::windows_core::IInspectable {
    fn from(value: AppResourceGroupStateReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppResourceGroupStateReport> for ::windows_core::IInspectable {
    fn from(value: &AppResourceGroupStateReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppResourceGroupStateReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppResourceGroupStateReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[repr(transparent)]
pub struct AppUriHandlerHost(::windows_core::IUnknown);
impl AppUriHandlerHost {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppUriHandlerHost, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppUriHandlerHost>(result__)
        })
    }
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppUriHandlerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerHost {}
impl ::core::fmt::Debug for AppUriHandlerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerHost;{5d50cac5-92d2-5409-b56f-7f73e10ea4c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
    const IID: ::windows_core::GUID = <IAppUriHandlerHost as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows_core::IUnknown {
    fn from(value: AppUriHandlerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows_core::IUnknown {
    fn from(value: &AppUriHandlerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppUriHandlerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerHost> for ::windows_core::IInspectable {
    fn from(value: AppUriHandlerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerHost> for ::windows_core::IInspectable {
    fn from(value: &AppUriHandlerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppUriHandlerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppUriHandlerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[repr(transparent)]
pub struct AppUriHandlerRegistration(::windows_core::IUnknown);
impl AppUriHandlerRegistration {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppAddedHostsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAppAddedHostsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows_core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAppAddedHostsAsync)(::windows_core::Interface::as_raw(this), hosts.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllHosts(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllHosts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<AppUriHandlerHost>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateHosts<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Collections::IIterable<AppUriHandlerHost>>>(&self, hosts: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UpdateHosts)(::windows_core::Interface::as_raw(this), hosts.into_param().abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistration {}
impl ::core::fmt::Debug for AppUriHandlerRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistration;{6f73aeb1-4569-5c3f-9ba0-99123eea32c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
    const IID: ::windows_core::GUID = <IAppUriHandlerRegistration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows_core::IUnknown {
    fn from(value: AppUriHandlerRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows_core::IUnknown {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerRegistration> for ::windows_core::IInspectable {
    fn from(value: AppUriHandlerRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistration> for ::windows_core::IInspectable {
    fn from(value: &AppUriHandlerRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppUriHandlerRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppUriHandlerRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[repr(transparent)]
pub struct AppUriHandlerRegistrationManager(::windows_core::IUnknown);
impl AppUriHandlerRegistrationManager {
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    pub fn TryGetRegistration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetRegistration)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppUriHandlerRegistration>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, User>>(user: Param0) -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn GetForPackage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefamilyname: Param0) -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForPackage)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn GetForPackageForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, User>>(packagefamilyname: Param0, user: Param1) -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForPackageForUser)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppUriHandlerRegistrationManager>(result__)
        })
    }
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppUriHandlerRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUriHandlerRegistrationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUriHandlerRegistrationManager {}
impl ::core::fmt::Debug for AppUriHandlerRegistrationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUriHandlerRegistrationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.AppUriHandlerRegistrationManager;{e62c9a52-ac94-5750-ac1b-6cfb6f250263})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppUriHandlerRegistrationManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows_core::IUnknown {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows_core::IUnknown {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppUriHandlerRegistrationManager> for ::windows_core::IInspectable {
    fn from(value: AppUriHandlerRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppUriHandlerRegistrationManager> for ::windows_core::IInspectable {
    fn from(value: &AppUriHandlerRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppUriHandlerRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateTimeZoneStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoUpdateTimeZoneStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateTimeZoneStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateTimeZoneStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct DateTimeSettings;
impl DateTimeSettings {
    #[cfg(feature = "Foundation")]
    pub fn SetSystemDateTime<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::DateTime>>(utcdatetime: Param0) -> ::windows_core::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetSystemDateTime)(::windows_core::Interface::as_raw(this), utcdatetime.into_param().abi()).ok() })
    }
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DiagnosticAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DiagnosticAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DiagnosticAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DispatcherQueue(::windows_core::IUnknown);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows_core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTimer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DispatcherQueueTimer>(result__)
        }
    }
    pub fn TryEnqueue<'a, Param0: ::windows_core::IntoParam<'a, DispatcherQueueHandler>>(&self, callback: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueue)(::windows_core::Interface::as_raw(this), callback.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryEnqueueWithPriority<'a, Param1: ::windows_core::IntoParam<'a, DispatcherQueueHandler>>(&self, priority: DispatcherQueuePriority, callback: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueueWithPriority)(::windows_core::Interface::as_raw(this), priority, callback.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownStarting<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownStarting<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShutdownStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueue, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShutdownCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn HasThreadAccess(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasThreadAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetForCurrentThread() -> ::windows_core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentThread)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DispatcherQueue>(result__)
        })
    }
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DispatcherQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueue {}
impl ::core::fmt::Debug for DispatcherQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueue;{603e88e4-a338-4ffe-a457-a5cfb9ceb899})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
    const IID: ::windows_core::GUID = <IDispatcherQueue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
impl ::core::convert::From<DispatcherQueue> for ::windows_core::IUnknown {
    fn from(value: DispatcherQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows_core::IUnknown {
    fn from(value: &DispatcherQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DispatcherQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueue> for ::windows_core::IInspectable {
    fn from(value: DispatcherQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueue> for ::windows_core::IInspectable {
    fn from(value: &DispatcherQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DispatcherQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DispatcherQueue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[repr(transparent)]
pub struct DispatcherQueueController(::windows_core::IUnknown);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ShutdownQueueAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownQueueAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows_core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnDedicatedThread)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DispatcherQueueController>(result__)
        })
    }
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DispatcherQueueController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueController {}
impl ::core::fmt::Debug for DispatcherQueueController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueController;{22f34e66-50db-4e36-a98d-61c01b384d20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
    const IID: ::windows_core::GUID = <IDispatcherQueueController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
impl ::core::convert::From<DispatcherQueueController> for ::windows_core::IUnknown {
    fn from(value: DispatcherQueueController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows_core::IUnknown {
    fn from(value: &DispatcherQueueController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DispatcherQueueController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueController> for ::windows_core::IInspectable {
    fn from(value: DispatcherQueueController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueController> for ::windows_core::IInspectable {
    fn from(value: &DispatcherQueueController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DispatcherQueueController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DispatcherQueueController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[repr(transparent)]
pub struct DispatcherQueueHandler(pub ::windows_core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandlerBox::<F> { vtable: &DispatcherQueueHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DispatcherQueueHandlerBox<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DispatcherQueueHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DispatcherQueueHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for DispatcherQueueHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueHandler {}
impl ::core::fmt::Debug for DispatcherQueueHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8);
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{dfa2dc9c-1a2d-4917-98f2-939af1d6e0c8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherQueuePriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DispatcherQueuePriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for DispatcherQueuePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueuePriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
impl DispatcherQueueShutdownStartingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for DispatcherQueueShutdownStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueShutdownStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueShutdownStartingEventArgs {}
impl ::core::fmt::Debug for DispatcherQueueShutdownStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueShutdownStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueShutdownStartingEventArgs;{c4724c4c-ff97-40c0-a226-cc0aaa545e89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDispatcherQueueShutdownStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueShutdownStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: DispatcherQueueShutdownStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueShutdownStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DispatcherQueueShutdownStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DispatcherQueueShutdownStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
pub struct DispatcherQueueTimer(::windows_core::IUnknown);
impl DispatcherQueueTimer {
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetInterval<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRunning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRepeating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeating)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRepeating)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Tick<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Tick)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTick<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTick)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DispatcherQueueTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DispatcherQueueTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatcherQueueTimer {}
impl ::core::fmt::Debug for DispatcherQueueTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueueTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.DispatcherQueueTimer;{5feabb1d-a31c-4727-b1ac-37454649d56a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
    const IID: ::windows_core::GUID = <IDispatcherQueueTimer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows_core::IUnknown {
    fn from(value: DispatcherQueueTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows_core::IUnknown {
    fn from(value: &DispatcherQueueTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DispatcherQueueTimer> for ::windows_core::IInspectable {
    fn from(value: DispatcherQueueTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DispatcherQueueTimer> for ::windows_core::IInspectable {
    fn from(value: &DispatcherQueueTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DispatcherQueueTimer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DispatcherQueueTimer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[repr(transparent)]
pub struct FolderLauncherOptions(::windows_core::IUnknown);
impl FolderLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FolderLauncherOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ItemsToSelect(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemsToSelect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows_core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::UI::ViewManagement::ViewSizePreference>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FolderLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderLauncherOptions {}
impl ::core::fmt::Debug for FolderLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.FolderLauncherOptions;{bb91c27d-6b87-432a-bd04-776c6f5fb2ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFolderLauncherOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: FolderLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: &FolderLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FolderLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: FolderLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: &FolderLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderLauncherOptions) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderLauncherOptions> for ILauncherViewOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderLauncherOptions) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILauncherViewOptions> for FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ILauncherViewOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILauncherViewOptions> for &FolderLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderLauncherOptions {}
unsafe impl ::core::marker::Sync for FolderLauncherOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppActivationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf46fbd7_191a_446c_9473_8fbc2374a354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResourceGroups: usize,
    pub CreateResourceGroupWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfo3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc895c63d_dd61_4c65_babd_81a10b4f9815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce6925bf_10ca_40c8_a9ca_c5c96501866e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05b24b86_1000_4c90_bb9f_7235071c50fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppUserModelId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDiagnosticInfoWatcherEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExecutionStateChangeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakPrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryReport2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f7f3738_51b7_42dc_b7ed_79ba46d28857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpectedTotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppMemoryUsageLimitChangingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub NewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupBackgroundTaskReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IsShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBackgroundTaskReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBackgroundTaskReports: usize,
    pub GetMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub GetProcessDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))]
    GetProcessDiagnosticInfos: usize,
    pub GetStateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee9b236d_d305_4d6b_92f7_6afdad72dedc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartSuspendAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSuspendAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartResumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTerminateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub ExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecutionStateChanged: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CommitUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub CommitUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows_core::HRESULT,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppResourceGroupStateReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows_core::HRESULT,
    pub EnergyQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHost2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a0bee95_29e4_51bf_8095_a3c068e3c72a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerHostFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x257c3c96_ce04_5f98_96bb_3ebd3e9275bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAddedHostsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAppAddedHostsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd54dac97_cb39_5f1f_883e_01853730bd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllHosts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateHosts: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbddfcaf1_b51a_5e69_aefd_7088d9f2b123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5cedd9f_5729_5b76_a1d4_0285f295c124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUriHandlerRegistrationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14f78379_6890_5080_90a7_98824a7f079e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForPackageForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d2150d1_47ee_48ab_a52b_9f1954278d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemDateTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueue2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc822c647_30ef_506e_bd1e_a647ae6675ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownQueueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a6c98e0_5198_49a2_a313_3f70d1f13c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa96d83d7_9371_4517_9245_d0824ac12c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDispatcherQueueTimer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFolderLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ItemsToSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ItemsToSelect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7755911a_70c5_48e5_b637_5ba3441e4ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GuestHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PrincipalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SessionInitiationProtocolUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownUserPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b450782_f620_577e_b1b3_dd56644d79b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AgeEnforcementRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILaunchUriResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetTreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub UI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ba08eb4_6e40_4dce_a1a3_2f53950afb49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
    #[cfg(feature = "Storage_Search")]
    pub SetNeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SetNeighboringFilesQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0770655_4b63_4e3a_9107_4e687841923a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherOptions4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6fd10e_e6fb_4814_a44e_57e8b9d9a01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics {
    type Vtable = ILauncherStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x277151c3_9e3e_42f6_91a4_5dfdeb232451);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59ba2fbb_24cb_4c02_a4c4_8294569d54f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, inputdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, inputdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersWithLaunchUriTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersWithLaunchUriTypeAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindFileHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindFileHandlersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x234261a8_9db3_4683_aa42_dc6f51d33847);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9ec819f_b5a5_41c6_b3b3_dd1b3178bcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindAppUriHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindAppUriHandlersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, inputdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, inputdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b24ef84_d895_5fea_9153_1ac49aed9ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILauncherUIOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetInvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
    #[cfg(feature = "UI_Popups")]
    pub PreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PreferredPlacement: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetPreferredPlacement: usize,
}
#[repr(transparent)]
pub struct ILauncherViewOptions(::windows_core::IUnknown);
impl ILauncherViewOptions {
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::UI::ViewManagement::ViewSizePreference>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows_core::IUnknown {
    fn from(value: ILauncherViewOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows_core::IUnknown {
    fn from(value: &ILauncherViewOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILauncherViewOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILauncherViewOptions> for ::windows_core::IInspectable {
    fn from(value: ILauncherViewOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILauncherViewOptions> for ::windows_core::IInspectable {
    fn from(value: &ILauncherViewOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILauncherViewOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILauncherViewOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILauncherViewOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILauncherViewOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILauncherViewOptions {}
impl ::core::fmt::Debug for ILauncherViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILauncherViewOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8a9b29f1-7ca7-49de-9bd3-3c5b7184f616}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a9b29f1_7ca7_49de_9bd3_3c5b7184f616);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c6c279c_d7ca_4779_9188_4057219ce64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AppMemoryUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageLimitChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageLimitChanging: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eee351f_6d62_423f_9479_b01f9c9f7669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAppMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetProcessMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x149b59ce_92ad_4e35_89eb_50dfb4c0d91c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TrySetAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5a94828_e84e_4886_8a0d_44b3190e3b72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpectedAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardError: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardError: usize,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessLauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ab66e7_2d0e_448b_a6a0_c13c3836d09c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsyncWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrivateWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtocolForResultsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredAppIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredAppIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteLauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7db7a93_a30c_48b7_9f21_051026a4e517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, inputdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    LaunchUriWithDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72e247ed_dd5b_4d6c_b1d0_c57a7bbb5f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub BeginShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginShutdown: usize,
    pub CancelShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShutdownManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f69a02f_9c34_43c7_a8c3_70b30a7f7504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPowerStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnterPowerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterPowerStateWithTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterPowerStateWithTimeSpan: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b3b2bea_a101_41ae_9fbd_028728bab73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentTimeZoneDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTimeZoneDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTimeZoneDisplayNames: usize,
    pub CanChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ChangeTimeZoneByDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezonedisplayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeZoneSettingsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x555c0db8_39a8_49fa_b4f6_a2c7fc2842ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AutoUpdateTimeZoneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoUpdateTimeZoneAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUser {
    type Vtable = IUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUser2 {
    type Vtable = IUser2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98ba5628_a6e3_518e_89d9_d3b2b1991a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CheckUserAgeConsentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUserAgeConsentGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangeDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserAuthenticationStatusChangingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NewStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b2ccb44_6f01_560c_97ad_fc7f32ec581f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedPropertyKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedPropertyKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NewUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OldUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDeviceAssociationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e491e14_f85a_4c07_8da9_7fe3d0542343);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FindUserFromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserDeviceAssociationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserDeviceAssociationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserPicker {
    type Vtable = IUserPicker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSingleUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserPickerStatics {
    type Vtable = IUserPickerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde3290dc_7e73_4df6_a1ae_4d7eca82b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserStatics {
    type Vtable = IUserStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByTypeAndStatus: usize,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonroamableid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserStatics2 {
    type Vtable = IUserStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74a37e11_2eb5_4487_b0d5_2c6790e013e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserWatcher {
    type Vtable = IUserWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
pub struct KnownUserProperties;
impl KnownUserProperties {
    pub fn DisplayName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FirstName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn LastName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LastName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ProviderName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AccountName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn GuestHost() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GuestHost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn PrincipalName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PrincipalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DomainName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DomainName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SessionInitiationProtocolUri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInitiationProtocolUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AgeEnforcementRegion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AgeEnforcementRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LaunchQuerySupportStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LaunchQuerySupportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchQuerySupportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LaunchUriResult(::windows_core::IUnknown);
impl LaunchUriResult {
    pub fn Status(&self) -> ::windows_core::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LaunchUriStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LaunchUriStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Result(&self) -> ::windows_core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchUriResult {}
impl ::core::fmt::Debug for LaunchUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.LaunchUriResult;{ec27a8df-f6d5-45ca-913a-70a40c5c8221})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
    const IID: ::windows_core::GUID = <ILaunchUriResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
impl ::core::convert::From<LaunchUriResult> for ::windows_core::IUnknown {
    fn from(value: LaunchUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows_core::IUnknown {
    fn from(value: &LaunchUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LaunchUriResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LaunchUriResult> for ::windows_core::IInspectable {
    fn from(value: LaunchUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchUriResult> for ::windows_core::IInspectable {
    fn from(value: &LaunchUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LaunchUriResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LaunchUriResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct Launcher;
impl Launcher {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::IStorageFile>, Param1: ::windows_core::IntoParam<'a, LauncherOptions>>(file: Param0, options: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, LauncherOptions>>(uri: Param0, options: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, LauncherOptions>, Param2: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsWithDataAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, LauncherOptions>, Param2: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(uri: Param0, options: Param1, inputdata: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryUriSupportAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), launchquerysupporttype, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uri: Param0, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryUriSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), launchquerysupporttype, packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::StorageFile>>(file: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryFileSupportAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportWithPackageFamilyNameAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::StorageFile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(file: Param0, packagefamilyname: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryFileSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUriSchemeHandlersAsync)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUriSchemeHandlersWithLaunchUriTypeAsync)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), launchquerysupporttype, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindFileHandlersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(extension: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindFileHandlersAsync)(::windows_core::Interface::as_raw(this), extension.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::IStorageFolder>>(folder: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderAsync)(::windows_core::Interface::as_raw(this), folder.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, FolderLauncherOptions>>(folder: Param0, options: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), folder.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryAppUriSupportAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uri: Param0, packagefamilyname: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryAppUriSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindAppUriHandlersAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppUriHandlersAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(user: Param0, uri: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, LauncherOptions>, Param3: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, LauncherOptions>>(user: Param0, uri: Param1, options: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, LauncherOptions>, Param3: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(user: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>> {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsWithDataForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<LaunchUriResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(path: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathAsync)(::windows_core::Interface::as_raw(this), path.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, FolderLauncherOptions>>(path: Param0, options: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathWithOptionsAsync)(::windows_core::Interface::as_raw(this), path.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, path: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), path.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, FolderLauncherOptions>>(user: Param0, path: Param1, options: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), path.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Launcher, ILauncherStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Launcher, ILauncherStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Launcher, ILauncherStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Launcher, ILauncherStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Launcher, ILauncherStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[repr(transparent)]
pub struct LauncherOptions(::windows_core::IUnknown);
impl LauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LauncherOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TreatAsUntrusted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TreatAsUntrusted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTreatAsUntrusted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayApplicationPicker(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayApplicationPicker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayApplicationPicker)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UI(&self) -> ::windows_core::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LauncherUIOptions>(result__)
        }
    }
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPreferredApplicationPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PreferredApplicationDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredApplicationDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPreferredApplicationDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredApplicationDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows_core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FallbackUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetApplicationPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn SetNeighboringFilesQuery<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::Search::StorageFileQueryResult>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNeighboringFilesQuery)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IgnoreAppUriHandlers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IgnoreAppUriHandlers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIgnoreAppUriHandlers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).LimitPickerToCurrentAppAndAppUriHandlers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLimitPickerToCurrentAppAndAppUriHandlers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows_core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::UI::ViewManagement::ViewSizePreference>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherOptions {}
impl ::core::fmt::Debug for LauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherOptions;{bafa21d8-b071-4cd8-853e-341203e557d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = <ILauncherOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
impl ::core::convert::From<LauncherOptions> for ::windows_core::IUnknown {
    fn from(value: LauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows_core::IUnknown {
    fn from(value: &LauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LauncherOptions> for ::windows_core::IInspectable {
    fn from(value: LauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherOptions> for ::windows_core::IInspectable {
    fn from(value: &LauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: LauncherOptions) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LauncherOptions> for ILauncherViewOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &LauncherOptions) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILauncherViewOptions> for LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ILauncherViewOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILauncherViewOptions> for &LauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ILauncherViewOptions> {
        ::core::convert::TryInto::<ILauncherViewOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LauncherOptions {}
unsafe impl ::core::marker::Sync for LauncherOptions {}
#[repr(transparent)]
pub struct LauncherUIOptions(::windows_core::IUnknown);
impl LauncherUIOptions {
    #[cfg(feature = "Foundation")]
    pub fn InvocationPoint(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InvocationPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IReference<super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetInvocationPoint<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Point>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInvocationPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IReference<super::Foundation::Rect>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::IReference<super::Foundation::Rect>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn PreferredPlacement(&self) -> ::windows_core::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::UI::Popups::Placement>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredPlacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UI::Popups::Placement>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredPlacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for LauncherUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LauncherUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LauncherUIOptions {}
impl ::core::fmt::Debug for LauncherUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LauncherUIOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.LauncherUIOptions;{1b25da6e-8aa6-41e9-8251-4165f5985f49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
    const IID: ::windows_core::GUID = <ILauncherUIOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
impl ::core::convert::From<LauncherUIOptions> for ::windows_core::IUnknown {
    fn from(value: LauncherUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows_core::IUnknown {
    fn from(value: &LauncherUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LauncherUIOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LauncherUIOptions> for ::windows_core::IInspectable {
    fn from(value: LauncherUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LauncherUIOptions> for ::windows_core::IInspectable {
    fn from(value: &LauncherUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LauncherUIOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LauncherUIOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
pub struct MemoryManager;
impl MemoryManager {
    pub fn AppMemoryUsage() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        })
    }
    pub fn AppMemoryUsageLimit() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        })
    }
    pub fn AppMemoryUsageLevel() -> ::windows_core::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppMemoryUsageLevel>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppMemoryUsageLevel>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageIncreased<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageIncreased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageIncreased<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageIncreased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageDecreased<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageDecreased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageDecreased<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageDecreased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageLimitChanging<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>>(handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLimitChanging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageLimitChanging<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageLimitChanging)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn GetAppMemoryReport() -> ::windows_core::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppMemoryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppMemoryReport>(result__)
        })
    }
    pub fn GetProcessMemoryReport() -> ::windows_core::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProcessMemoryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProcessMemoryReport>(result__)
        })
    }
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows_core::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetAppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpectedAppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        })
    }
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PowerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct ProcessLauncher;
impl ProcessLauncher {
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(filename: Param0, args: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RunToCompletionAsync)(::windows_core::Interface::as_raw(this), filename.into_param().abi(), args.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsyncWithOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProcessLauncherOptions>>(filename: Param0, args: Param1, options: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RunToCompletionAsyncWithOptions)(::windows_core::Interface::as_raw(this), filename.into_param().abi(), args.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<ProcessLauncherResult>>(result__)
        })
    }
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[repr(transparent)]
pub struct ProcessLauncherOptions(::windows_core::IUnknown);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProcessLauncherOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardInput(&self) -> ::windows_core::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StandardInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardInput<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardOutput(&self) -> ::windows_core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StandardOutput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardOutput<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardOutput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardError(&self) -> ::windows_core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StandardError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardError<'a, Param0: ::windows_core::IntoParam<'a, super::Storage::Streams::IOutputStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WorkingDirectory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetWorkingDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWorkingDirectory)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ProcessLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherOptions {}
impl ::core::fmt::Debug for ProcessLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherOptions;{3080b9cf-f444-4a83-beaf-a549a0f3229c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = <IProcessLauncherOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: ProcessLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: &ProcessLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: ProcessLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: &ProcessLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[repr(transparent)]
pub struct ProcessLauncherResult(::windows_core::IUnknown);
impl ProcessLauncherResult {
    pub fn ExitCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessLauncherResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessLauncherResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessLauncherResult {}
impl ::core::fmt::Debug for ProcessLauncherResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessLauncherResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessLauncherResult;{544c8934-86d8-4991-8e75-ece8a43b6b6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
    const IID: ::windows_core::GUID = <IProcessLauncherResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows_core::IUnknown {
    fn from(value: ProcessLauncherResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows_core::IUnknown {
    fn from(value: &ProcessLauncherResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessLauncherResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessLauncherResult> for ::windows_core::IInspectable {
    fn from(value: ProcessLauncherResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessLauncherResult> for ::windows_core::IInspectable {
    fn from(value: &ProcessLauncherResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessLauncherResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessLauncherResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[repr(transparent)]
pub struct ProcessMemoryReport(::windows_core::IUnknown);
impl ProcessMemoryReport {
    pub fn PrivateWorkingSetUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PrivateWorkingSetUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalWorkingSetUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalWorkingSetUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProcessMemoryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryReport {}
impl ::core::fmt::Debug for ProcessMemoryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.ProcessMemoryReport;{087305a8-9b70-4782-8741-3a982b6ce5e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
    const IID: ::windows_core::GUID = <IProcessMemoryReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows_core::IUnknown {
    fn from(value: ProcessMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows_core::IUnknown {
    fn from(value: &ProcessMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessMemoryReport> for ::windows_core::IInspectable {
    fn from(value: ProcessMemoryReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessMemoryReport> for ::windows_core::IInspectable {
    fn from(value: &ProcessMemoryReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessMemoryReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProcessorArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProcessorArchitecture {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProcessorArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessorArchitecture").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProtocolForResultsOperation(::windows_core::IUnknown);
impl ProtocolForResultsOperation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ProtocolForResultsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsOperation {}
impl ::core::fmt::Debug for ProtocolForResultsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.ProtocolForResultsOperation;{d581293a-6de9-4d28-9378-f86782e182bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
    const IID: ::windows_core::GUID = <IProtocolForResultsOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows_core::IUnknown {
    fn from(value: ProtocolForResultsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows_core::IUnknown {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolForResultsOperation> for ::windows_core::IInspectable {
    fn from(value: ProtocolForResultsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsOperation> for ::windows_core::IInspectable {
    fn from(value: &ProtocolForResultsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtocolForResultsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtocolForResultsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteLaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemoteLaunchUriStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteLaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLaunchUriStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct RemoteLauncher;
impl RemoteLauncher {
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriAsync<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(remotesystemconnectionrequest: Param0, uri: Param1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, RemoteLauncherOptions>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithDataAsync<'a, Param0: ::windows_core::IntoParam<'a, RemoteSystems::RemoteSystemConnectionRequest>, Param1: ::windows_core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows_core::IntoParam<'a, RemoteLauncherOptions>, Param3: ::windows_core::IntoParam<'a, super::Foundation::Collections::ValueSet>>(remotesystemconnectionrequest: Param0, uri: Param1, options: Param2, inputdata: Param3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>> {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>(result__)
        })
    }
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[repr(transparent)]
pub struct RemoteLauncherOptions(::windows_core::IUnknown);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RemoteLauncherOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows_core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FallbackUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredAppIds(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredAppIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteLauncherOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteLauncherOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteLauncherOptions {}
impl ::core::fmt::Debug for RemoteLauncherOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLauncherOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteLauncherOptions;{9e3a2788-2891-4cdf-a2d6-9dff7d02e693})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
    const IID: ::windows_core::GUID = <IRemoteLauncherOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: RemoteLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows_core::IUnknown {
    fn from(value: &RemoteLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: RemoteLauncherOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteLauncherOptions> for ::windows_core::IInspectable {
    fn from(value: &RemoteLauncherOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RemoteLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RemoteLauncherOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ShutdownKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct ShutdownManager;
impl ShutdownManager {
    #[cfg(feature = "Foundation")]
    pub fn BeginShutdown<'a, Param1: ::windows_core::IntoParam<'a, super::Foundation::TimeSpan>>(shutdownkind: ShutdownKind, timeout: Param1) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).BeginShutdown)(::windows_core::Interface::as_raw(this), shutdownkind, timeout.into_param().abi()).ok() })
    }
    pub fn CancelShutdown() -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).CancelShutdown)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows_core::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPowerStateSupported)(::windows_core::Interface::as_raw(this), powerstate, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnterPowerState)(::windows_core::Interface::as_raw(this), powerstate).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn EnterPowerStateWithTimeSpan<'a, Param1: ::windows_core::IntoParam<'a, super::Foundation::TimeSpan>>(powerstate: PowerState, wakeupafter: Param1) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnterPowerStateWithTimeSpan)(::windows_core::Interface::as_raw(this), powerstate, wakeupafter.into_param().abi()).ok() })
    }
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
pub struct TimeZoneSettings;
impl TimeZoneSettings {
    pub fn CurrentTimeZoneDisplayName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentTimeZoneDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTimeZoneDisplayNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn CanChangeTimeZone() -> ::windows_core::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanChangeTimeZone)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ChangeTimeZoneByDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(timezonedisplayname: Param0) -> ::windows_core::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ChangeTimeZoneByDisplayName)(::windows_core::Interface::as_raw(this), timezonedisplayname.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn AutoUpdateTimeZoneAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TimeSpan>>(timeout: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoUpdateTimeZoneAsync)(::windows_core::Interface::as_raw(this), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>(result__)
        })
    }
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[repr(transparent)]
pub struct User(::windows_core::IUnknown);
impl User {
    pub fn NonRoamableId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NonRoamableId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AuthenticationStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserAuthenticationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>>(&self, values: Param0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), values.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPictureAsync)(::windows_core::Interface::as_raw(this), desiredsize, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows_core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows_core::Interface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckUserAgeConsentGroupAsync)(::windows_core::Interface::as_raw(this), consentgroup, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<UserAgeConsentResult>>(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows_core::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncByType)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncByTypeAndStatus)(::windows_core::Interface::as_raw(this), r#type, status, result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>(result__)
        })
    }
    pub fn GetFromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(nonroamableid: Param0) -> ::windows_core::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromId)(::windows_core::Interface::as_raw(this), nonroamableid.into_param().abi(), result__.as_mut_ptr()).from_abi::<User>(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        })
    }
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<User, IUserStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<User, IUserStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for User {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for User {}
impl ::core::fmt::Debug for User {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("User").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for User {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.User;{df9a26c6-e746-4bcd-b5d4-120103c4209b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for User {
    type Vtable = IUser_Vtbl;
    const IID: ::windows_core::GUID = <IUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
impl ::core::convert::From<User> for ::windows_core::IUnknown {
    fn from(value: User) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&User> for ::windows_core::IUnknown {
    fn from(value: &User) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for User {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a User {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<User> for ::windows_core::IInspectable {
    fn from(value: User) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&User> for ::windows_core::IInspectable {
    fn from(value: &User) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for User {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a User {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentGroup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserAgeConsentGroup {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserAgeConsentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAgeConsentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserAuthenticationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserAuthenticationStatusChangeDeferral(::windows_core::IUnknown);
impl UserAuthenticationStatusChangeDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangeDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangeDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangeDeferral {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangeDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangeDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangeDeferral;{88b59568-bb30-42fb-a270-e9902e40efa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IUserAuthenticationStatusChangeDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows_core::IUnknown {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows_core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangeDeferral> for ::windows_core::IInspectable {
    fn from(value: UserAuthenticationStatusChangeDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangeDeferral> for ::windows_core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangeDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserAuthenticationStatusChangeDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[repr(transparent)]
pub struct UserAuthenticationStatusChangingEventArgs(::windows_core::IUnknown);
impl UserAuthenticationStatusChangingEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserAuthenticationStatusChangeDeferral>(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    pub fn NewStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserAuthenticationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).NewStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
    pub fn CurrentStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserAuthenticationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserAuthenticationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for UserAuthenticationStatusChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserAuthenticationStatusChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserAuthenticationStatusChangingEventArgs {}
impl ::core::fmt::Debug for UserAuthenticationStatusChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatusChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserAuthenticationStatusChangingEventArgs;{8c030f28-a711-4c1e-ab48-04179c15938f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserAuthenticationStatusChangingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserAuthenticationStatusChangingEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserAuthenticationStatusChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserAuthenticationStatusChangingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserAuthenticationStatusChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserAuthenticationStatusChangingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[repr(transparent)]
pub struct UserChangedEventArgs(::windows_core::IUnknown);
impl UserChangedEventArgs {
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedPropertyKinds(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows_core::Interface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangedPropertyKinds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserChangedEventArgs {}
impl ::core::fmt::Debug for UserChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserChangedEventArgs;{086459dc-18c6-48db-bc99-724fb9203ccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
pub struct UserDeviceAssociation;
impl UserDeviceAssociation {
    pub fn FindUserFromDeviceId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUserFromDeviceId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<User>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn UserDeviceAssociationChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>>(handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UserDeviceAssociationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserDeviceAssociationChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveUserDeviceAssociationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[repr(transparent)]
pub struct UserDeviceAssociationChangedEventArgs(::windows_core::IUnknown);
impl UserDeviceAssociationChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NewUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    pub fn OldUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDeviceAssociationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDeviceAssociationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDeviceAssociationChangedEventArgs {}
impl ::core::fmt::Debug for UserDeviceAssociationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDeviceAssociationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserDeviceAssociationChangedEventArgs;{bd1f6f6c-bb5d-4d7b-a5f0-c8cd11a38d42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDeviceAssociationChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDeviceAssociationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDeviceAssociationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDeviceAssociationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDeviceAssociationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDeviceAssociationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[repr(transparent)]
pub struct UserPicker(::windows_core::IUnknown);
impl UserPicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserPicker, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowGuestAccounts(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowGuestAccounts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowGuestAccounts)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuggestedSelectedUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedSelectedUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<User>(result__)
        }
    }
    pub fn SetSuggestedSelectedUser<'a, Param0: ::windows_core::IntoParam<'a, User>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedSelectedUser)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PickSingleUserAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleUserAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Foundation::IAsyncOperation<User>>(result__)
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserPicker, IUserPickerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserPicker {}
impl ::core::fmt::Debug for UserPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPicker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserPicker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserPicker;{7d548008-f1e3-4a6c-8ddc-a9bb0f488aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserPicker {
    type Vtable = IUserPicker_Vtbl;
    const IID: ::windows_core::GUID = <IUserPicker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
impl ::core::convert::From<UserPicker> for ::windows_core::IUnknown {
    fn from(value: UserPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserPicker> for ::windows_core::IUnknown {
    fn from(value: &UserPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserPicker> for ::windows_core::IInspectable {
    fn from(value: UserPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserPicker> for ::windows_core::IInspectable {
    fn from(value: &UserPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserPictureSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserWatcher(::windows_core::IUnknown);
impl UserWatcher {
    pub fn Status(&self) -> ::windows_core::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAuthenticationStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanging<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatusChanging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanging<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAuthenticationStatusChanging)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::TypedEventHandler<UserWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserWatcher {}
impl ::core::fmt::Debug for UserWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.UserWatcher;{155eb23b-242a-45e0-a2e9-3171fc6a7fbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserWatcher {
    type Vtable = IUserWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IUserWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
impl ::core::convert::From<UserWatcher> for ::windows_core::IUnknown {
    fn from(value: UserWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows_core::IUnknown {
    fn from(value: &UserWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserWatcher> for ::windows_core::IInspectable {
    fn from(value: UserWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserWatcher> for ::windows_core::IInspectable {
    fn from(value: &UserWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserWatcherUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserWatcherUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKey {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VirtualKey {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKey").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKeyModifiers {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VirtualKeyModifiers {
    type Abi = Self;
}
impl ::core::fmt::Debug for VirtualKeyModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKeyModifiers").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}