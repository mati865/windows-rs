#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6daa9fc_bc5b_401a_a8b2_6e754d14daa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPreallocatedWorkItemFactory {
    type Vtable = IPreallocatedWorkItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d32b45_dfea_469b_82c5_f6e3cefdeafb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItem: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, priority: super::WorkItemPriority, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriority: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriorityAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, priority: super::WorkItemPriority, options: super::WorkItemOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriorityAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14285e06_63a7_4713_b6d9_62f64b56fb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifierStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISignalNotifierStatics {
    type Vtable = ISignalNotifierStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c4e4566_8400_46d3_a115_7d0c0dfc9f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifierStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AttachToEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, handler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToEventWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, handler: ::windows_core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToEventWithTimeout: usize,
    pub AttachToSemaphore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, handler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToSemaphoreWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, handler: ::windows_core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToSemaphoreWithTimeout: usize,
}
#[repr(transparent)]
pub struct PreallocatedWorkItem(::windows_core::IUnknown);
impl PreallocatedWorkItem {
    #[cfg(feature = "Foundation")]
    pub fn RunAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RunAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItem<'a, Param0: ::windows_core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0) -> ::windows_core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWorkItem)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriority<'a, Param0: ::windows_core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0, priority: super::WorkItemPriority) -> ::windows_core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWorkItemWithPriority)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), priority, result__.as_mut_ptr()).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriorityAndOptions<'a, Param0: ::windows_core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> ::windows_core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWorkItemWithPriorityAndOptions)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), priority, options, result__.as_mut_ptr()).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    pub fn IPreallocatedWorkItemFactory<R, F: FnOnce(&IPreallocatedWorkItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PreallocatedWorkItem, IPreallocatedWorkItemFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PreallocatedWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreallocatedWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreallocatedWorkItem {}
impl ::core::fmt::Debug for PreallocatedWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreallocatedWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PreallocatedWorkItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.Core.PreallocatedWorkItem;{b6daa9fc-bc5b-401a-a8b2-6e754d14daa6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
    const IID: ::windows_core::GUID = <IPreallocatedWorkItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PreallocatedWorkItem {
    const NAME: &'static str = "Windows.System.Threading.Core.PreallocatedWorkItem";
}
impl ::core::convert::From<PreallocatedWorkItem> for ::windows_core::IUnknown {
    fn from(value: PreallocatedWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreallocatedWorkItem> for ::windows_core::IUnknown {
    fn from(value: &PreallocatedWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PreallocatedWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PreallocatedWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PreallocatedWorkItem> for ::windows_core::IInspectable {
    fn from(value: PreallocatedWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreallocatedWorkItem> for ::windows_core::IInspectable {
    fn from(value: &PreallocatedWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PreallocatedWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PreallocatedWorkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PreallocatedWorkItem {}
unsafe impl ::core::marker::Sync for PreallocatedWorkItem {}
#[repr(transparent)]
pub struct SignalHandler(pub ::windows_core::IUnknown);
impl SignalHandler {
    pub fn new<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SignalHandlerBox::<F> { vtable: &SignalHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, SignalNotifier>>(&self, signalnotifier: Param0, timedout: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), signalnotifier.into_param().abi(), timedout).ok() }
    }
}
#[repr(C)]
struct SignalHandlerBox<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SignalHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SignalHandlerBox<F> {
    const VTABLE: SignalHandler_Vtbl = SignalHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<SignalHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, signalnotifier: ::windows_core::RawPtr, timedout: bool) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&signalnotifier), timedout).into()
    }
}
impl ::core::clone::Clone for SignalHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignalHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignalHandler {}
impl ::core::fmt::Debug for SignalHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for SignalHandler {
    type Vtable = SignalHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x923c402e_4721_440e_9dda_55b6f2e07710);
}
unsafe impl ::windows_core::RuntimeType for SignalHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{923c402e-4721-440e-9dda-55b6f2e07710}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SignalHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalnotifier: ::windows_core::RawPtr, timedout: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SignalNotifier(::windows_core::IUnknown);
impl SignalNotifier {
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Terminate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Terminate)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AttachToEvent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SignalHandler>>(name: Param0, handler: Param1) -> ::windows_core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachToEvent)(::windows_core::Interface::as_raw(this), name.into_param().abi(), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<SignalNotifier>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AttachToEventWithTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SignalHandler>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(name: Param0, handler: Param1, timeout: Param2) -> ::windows_core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachToEventWithTimeout)(::windows_core::Interface::as_raw(this), name.into_param().abi(), handler.into_param().abi(), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<SignalNotifier>(result__)
        })
    }
    pub fn AttachToSemaphore<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SignalHandler>>(name: Param0, handler: Param1) -> ::windows_core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachToSemaphore)(::windows_core::Interface::as_raw(this), name.into_param().abi(), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<SignalNotifier>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AttachToSemaphoreWithTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SignalHandler>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(name: Param0, handler: Param1, timeout: Param2) -> ::windows_core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachToSemaphoreWithTimeout)(::windows_core::Interface::as_raw(this), name.into_param().abi(), handler.into_param().abi(), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<SignalNotifier>(result__)
        })
    }
    pub fn ISignalNotifierStatics<R, F: FnOnce(&ISignalNotifierStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SignalNotifier, ISignalNotifierStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SignalNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignalNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignalNotifier {}
impl ::core::fmt::Debug for SignalNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalNotifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SignalNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.Core.SignalNotifier;{14285e06-63a7-4713-b6d9-62f64b56fb8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
    const IID: ::windows_core::GUID = <ISignalNotifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SignalNotifier {
    const NAME: &'static str = "Windows.System.Threading.Core.SignalNotifier";
}
impl ::core::convert::From<SignalNotifier> for ::windows_core::IUnknown {
    fn from(value: SignalNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignalNotifier> for ::windows_core::IUnknown {
    fn from(value: &SignalNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SignalNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SignalNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SignalNotifier> for ::windows_core::IInspectable {
    fn from(value: SignalNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignalNotifier> for ::windows_core::IInspectable {
    fn from(value: &SignalNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SignalNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SignalNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SignalNotifier {}
unsafe impl ::core::marker::Sync for SignalNotifier {}