#[repr(transparent)]
pub struct CoreFrameworkInputView(::windows_core::IUnknown);
impl CoreFrameworkInputView {
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForUIContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows_core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreFrameworkInputView>(result__)
        })
    }
    pub fn ICoreFrameworkInputViewStatics<R, F: FnOnce(&ICoreFrameworkInputViewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreFrameworkInputView, ICoreFrameworkInputViewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputView {}
impl ::core::fmt::Debug for CoreFrameworkInputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreFrameworkInputView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputView;{d77c94ae-46b8-5d4a-9489-8ddec3d639a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_Vtbl;
    const IID: ::windows_core::GUID = <ICoreFrameworkInputView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows_core::IUnknown {
    fn from(value: CoreFrameworkInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows_core::IUnknown {
    fn from(value: &CoreFrameworkInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreFrameworkInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputView> for ::windows_core::IInspectable {
    fn from(value: CoreFrameworkInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputView> for ::windows_core::IInspectable {
    fn from(value: &CoreFrameworkInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreFrameworkInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreFrameworkInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputView {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputView {}
#[repr(transparent)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(::windows_core::IUnknown);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occlusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    pub fn FrameworkAnimationRecommended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkAnimationRecommended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AnimationDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputViewAnimationStartingEventArgs {}
impl ::core::fmt::Debug for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputViewAnimationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs;{c0ec901c-bba4-501b-ae8b-65c9e756a719})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreFrameworkInputViewAnimationStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewAnimationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreFrameworkInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreFrameworkInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(::windows_core::IUnknown);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occlusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
impl ::core::fmt::Debug for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreFrameworkInputViewOcclusionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs;{f36f4949-c82c-53d1-a75d-2b2baf0d9b0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreFrameworkInputViewOcclusionsChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreFrameworkInputViewOcclusionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreFrameworkInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreFrameworkInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
pub struct CoreInputView(::windows_core::IUnknown);
impl CoreInputView {
    #[cfg(feature = "Foundation")]
    pub fn OcclusionsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOcclusionsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCoreInputViewOcclusions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCoreInputViewOcclusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    pub fn TryShowPrimaryView(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryShowPrimaryView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryHidePrimaryView(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryHidePrimaryView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferringFromPrimaryView<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusTransferringFromPrimaryView)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferringFromPrimaryView<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveXYFocusTransferringFromPrimaryView)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn XYFocusTransferredToPrimaryView<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusTransferredToPrimaryView)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveXYFocusTransferredToPrimaryView<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveXYFocusTransferredToPrimaryView)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryTransferXYFocusToPrimaryView<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, origin: Param0, direction: CoreInputViewXYFocusTransferDirection) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryTransferXYFocusToPrimaryView)(::windows_core::Interface::as_raw(this), origin.into_param().abi(), direction, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryShow(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryShow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryShowWithKind)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryHide(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryHide)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewShowing<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryViewShowing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewShowing<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrimaryViewShowing)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewHiding<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryViewHiding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewHiding<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrimaryViewHiding)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsKindSupported)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SupportedKindsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedKindsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSupportedKindsChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSupportedKindsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrimaryViewAnimationStarting<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrimaryViewAnimationStarting<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreInputView>(result__)
        })
    }
    pub fn GetForUIContext<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIContext>>(context: Param0) -> ::windows_core::Result<CoreInputView> {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreInputView>(result__)
        })
    }
    pub fn ICoreInputViewStatics<R, F: FnOnce(&ICoreInputViewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreInputView, ICoreInputViewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreInputViewStatics2<R, F: FnOnce(&ICoreInputViewStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreInputView, ICoreInputViewStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreInputView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputView {}
impl ::core::fmt::Debug for CoreInputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputView;{c770cd7a-7001-4c32-bf94-25c1f554cbf1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputView {
    type Vtable = ICoreInputView_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputView";
}
impl ::core::convert::From<CoreInputView> for ::windows_core::IUnknown {
    fn from(value: CoreInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows_core::IUnknown {
    fn from(value: &CoreInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputView> for ::windows_core::IInspectable {
    fn from(value: CoreInputView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputView> for ::windows_core::IInspectable {
    fn from(value: &CoreInputView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputView {}
unsafe impl ::core::marker::Sync for CoreInputView {}
#[repr(transparent)]
pub struct CoreInputViewAnimationStartingEventArgs(::windows_core::IUnknown);
impl CoreInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occlusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnimationDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).AnimationDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewAnimationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewAnimationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewAnimationStartingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewAnimationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewAnimationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs;{a9144af2-b55c-5ea1-b8ab-5340f3e94897})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewAnimationStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewAnimationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewAnimationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewAnimationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewAnimationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewAnimationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
pub struct CoreInputViewHidingEventArgs(::windows_core::IUnknown);
impl CoreInputViewHidingEventArgs {
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewHidingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewHidingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewHidingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewHidingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewHidingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewHidingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs;{eada47bd-bac5-5336-848d-41083584daad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewHidingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewHidingEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewHidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewHidingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewHidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewHidingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewHidingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewHidingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreInputViewKind {}
impl ::core::clone::Clone for CoreInputViewKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreInputViewKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CoreInputViewKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CoreInputViewOcclusion(::windows_core::IUnknown);
impl CoreInputViewOcclusion {
    #[cfg(feature = "Foundation")]
    pub fn OccludingRect(&self) -> ::windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).OccludingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn OcclusionKind(&self) -> ::windows_core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CoreInputViewOcclusionKind>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreInputViewOcclusionKind>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewOcclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewOcclusion {}
impl ::core::fmt::Debug for CoreInputViewOcclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewOcclusion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusion;{cc36ce06-3865-4177-b5f5-8b65e0b9ce84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewOcclusion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewOcclusion> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusion> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusion {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusion {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreInputViewOcclusionKind {}
impl ::core::clone::Clone for CoreInputViewOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreInputViewOcclusionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CoreInputViewOcclusionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewOcclusionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CoreInputViewOcclusionsChangedEventArgs(::windows_core::IUnknown);
impl CoreInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Occlusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CoreInputViewOcclusionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewOcclusionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewOcclusionsChangedEventArgs {}
impl ::core::fmt::Debug for CoreInputViewOcclusionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs;{be1027e8-b3ee-4df7-9554-89cdc66082c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewOcclusionsChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewOcclusionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewOcclusionsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewOcclusionsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewOcclusionsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewOcclusionsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
pub struct CoreInputViewShowingEventArgs(::windows_core::IUnknown);
impl CoreInputViewShowingEventArgs {
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewShowingEventArgs {}
impl ::core::fmt::Debug for CoreInputViewShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewShowingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewShowingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs;{ca52261b-fb9e-5daf-a98c-262b8b76af50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewShowingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewShowingEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewShowingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewShowingEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewShowingEventArgs {}
#[repr(transparent)]
pub struct CoreInputViewTransferringXYFocusEventArgs(::windows_core::IUnknown);
impl CoreInputViewTransferringXYFocusEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Origin(&self) -> ::windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).Origin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows_core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CoreInputViewXYFocusTransferDirection>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreInputViewXYFocusTransferDirection>(result__)
        }
    }
    pub fn SetTransferHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransferHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransferHandled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TransferHandled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeepPrimaryViewVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepPrimaryViewVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).KeepPrimaryViewVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreInputViewTransferringXYFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreInputViewTransferringXYFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreInputViewTransferringXYFocusEventArgs {}
impl ::core::fmt::Debug for CoreInputViewTransferringXYFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewTransferringXYFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs;{04de169f-ba02-4850-8b55-d82d03ba6d7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICoreInputViewTransferringXYFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreInputViewTransferringXYFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: CoreInputViewTransferringXYFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreInputViewTransferringXYFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CoreInputViewTransferringXYFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreInputViewTransferringXYFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl ::core::marker::Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreInputViewXYFocusTransferDirection {}
impl ::core::clone::Clone for CoreInputViewXYFocusTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreInputViewXYFocusTransferDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CoreInputViewXYFocusTransferDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for CoreInputViewXYFocusTransferDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputViewXYFocusTransferDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub FrameworkAnimationRecommended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreFrameworkInputViewStatics {
    type Vtable = ICoreFrameworkInputViewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eebd9b6_eac2_5f8b_975f_772ee3e42eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputView {
    type Vtable = ICoreInputView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub OcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OcclusionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOcclusionsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCoreInputViewOcclusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCoreInputViewOcclusions: usize,
    pub TryShowPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryHidePrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputView2 {
    type Vtable = ICoreInputView2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ed726c1_e09a_4ae8_aedf_dfa4857d1a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferringFromPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferringFromPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub XYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXYFocusTransferredToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXYFocusTransferredToPrimaryView: usize,
    #[cfg(feature = "Foundation")]
    pub TryTransferXYFocusToPrimaryView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTransferXYFocusToPrimaryView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputView3 {
    type Vtable = ICoreInputView3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc941653_3ab9_4849_8f58_46e7f0353cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputView4 {
    type Vtable = ICoreInputView4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x002863d6_d9ef_57eb_8cef_77f6ce1b7ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewShowing: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewHiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewHiding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputView5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputView5 {
    type Vtable = ICoreInputView5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x136316e0_c6d5_5c57_811e_1ad8a99ba6ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CoreInputViewKind, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SupportedKindsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSupportedKindsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSupportedKindsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryViewAnimationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrimaryViewAnimationStarting: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewAnimationStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnimationDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewHidingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeada47bd_bac5_5336_848d_41083584daad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
    pub OcclusionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewOcclusionKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewOcclusionsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewShowingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca52261b_fb9e_5daf_a98c_262b8b76af50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewStatics {
    type Vtable = ICoreInputViewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9b97cd_edbe_49cf_a54f_337de052907f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewStatics2 {
    type Vtable = ICoreInputViewStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ebc0862_d049_4e52_87b0_1e90e98c49ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreInputViewTransferringXYFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Origin: usize,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CoreInputViewXYFocusTransferDirection) -> ::windows_core::HRESULT,
    pub SetTransferHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub TransferHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetKeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub KeepPrimaryViewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUISettingsController {
    type Vtable = IUISettingsController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAdvancedEffectsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetAnimationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetAutoHideScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetMessageDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub SetTextScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUISettingsControllerStatics {
    type Vtable = IUISettingsControllerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb3c68cc_c220_578c_8119_7db324ed26a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDefaultAsync: usize,
}
#[repr(transparent)]
pub struct UISettingsController(::windows_core::IUnknown);
impl UISettingsController {
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAdvancedEffectsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAnimationsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAnimationsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAutoHideScrollBars(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoHideScrollBars)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMessageDuration(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessageDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetTextScaleFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextScaleFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDefaultAsync() -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestDefaultAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<UISettingsController>>(result__)
        })
    }
    pub fn IUISettingsControllerStatics<R, F: FnOnce(&IUISettingsControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UISettingsController, IUISettingsControllerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UISettingsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettingsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsController {}
impl ::core::fmt::Debug for UISettingsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UISettingsController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.Core.UISettingsController;{78a51ac4-15c0-5a1b-a75b-acbf9cb8bb9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UISettingsController {
    type Vtable = IUISettingsController_Vtbl;
    const IID: ::windows_core::GUID = <IUISettingsController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.UISettingsController";
}
impl ::core::convert::From<UISettingsController> for ::windows_core::IUnknown {
    fn from(value: UISettingsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows_core::IUnknown {
    fn from(value: &UISettingsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UISettingsController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UISettingsController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettingsController> for ::windows_core::IInspectable {
    fn from(value: UISettingsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsController> for ::windows_core::IInspectable {
    fn from(value: &UISettingsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UISettingsController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UISettingsController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsController {}
unsafe impl ::core::marker::Sync for UISettingsController {}