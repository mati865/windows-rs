#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskDataProviderConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskDataProviderConnection {
    type Vtable = IUserDataTaskDataProviderConnection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ff39d1d_a447_428b_afe9_e5402bdeb041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskDataProviderConnection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SkipOccurrenceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSkipOccurrenceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CompleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteTaskRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskDataProviderTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskDataProviderTriggerDetails {
    type Vtable = IUserDataTaskDataProviderTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae273202_b1c9_453e_afc5_b30af3bd217d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskDataProviderTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListCompleteTaskRequest {
    type Vtable = IUserDataTaskListCompleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf65e14a3_1a42_49da_8552_2873e52c55eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCompleteTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completedtaskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListCompleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd77c393d_4cf2_48ad_87fd_963f0eaa7a95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListCreateOrUpdateTaskRequest {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2133772c_55c2_4300_8279_04326e07cce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, createdorupdateduserdatatask: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12c55a52_e378_419b_ae38_a5e9e604476e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListDeleteTaskRequest {
    type Vtable = IUserDataTaskListDeleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b863c68_7657_4f3d_b074_d47ec8df07e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListDeleteTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListDeleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6063dad9_f562_4145_8efe_d50078c92b7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListSkipOccurrenceRequest {
    type Vtable = IUserDataTaskListSkipOccurrenceRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab87e34d_1cd3_431c_9f58_089aa4338d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSkipOccurrenceRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListSkipOccurrenceRequestEventArgs {
    type Vtable = IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a3b924a_cc2f_4e7b_aacd_a5b9d29cfa4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListSyncManagerSyncRequest {
    type Vtable = IUserDataTaskListSyncManagerSyncRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40a73807_7590_4149_ae19_b211431a9f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManagerSyncRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataTaskListSyncManagerSyncRequestEventArgs {
    type Vtable = IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ead1c12_768e_43bd_8385_5cdc351ffdea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(transparent)]
pub struct UserDataTaskDataProviderConnection(::windows_core::IUnknown);
impl UserDataTaskDataProviderConnection {
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCreateOrUpdateTaskRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOrUpdateTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCreateOrUpdateTaskRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SyncRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SkipOccurrenceRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSkipOccurrenceRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SkipOccurrenceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSkipOccurrenceRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSkipOccurrenceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CompleteTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCompleteTaskRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleteTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleteTaskRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListDeleteTaskRequestEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteTaskRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDeleteTaskRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskDataProviderConnection {}
impl ::core::fmt::Debug for UserDataTaskDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskDataProviderConnection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection;{9ff39d1d-a447-428b-afe9-e5402bdeb041})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskDataProviderConnection {
    type Vtable = IUserDataTaskDataProviderConnection_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskDataProviderConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection";
}
impl ::core::convert::From<UserDataTaskDataProviderConnection> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskDataProviderConnection> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskDataProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskDataProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskDataProviderConnection> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskDataProviderConnection> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskDataProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskDataProviderConnection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskDataProviderConnection {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderConnection {}
#[repr(transparent)]
pub struct UserDataTaskDataProviderTriggerDetails(::windows_core::IUnknown);
impl UserDataTaskDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows_core::Result<UserDataTaskDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskDataProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskDataProviderTriggerDetails {}
impl ::core::fmt::Debug for UserDataTaskDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskDataProviderTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails;{ae273202-b1c9-453e-afc5-b30af3bd217d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskDataProviderTriggerDetails {
    type Vtable = IUserDataTaskDataProviderTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskDataProviderTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails";
}
impl ::core::convert::From<UserDataTaskDataProviderTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskDataProviderTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskDataProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskDataProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskDataProviderTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskDataProviderTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskDataProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskDataProviderTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderTriggerDetails {}
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequest(::windows_core::IUnknown);
impl UserDataTaskListCompleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, completedtaskid: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), completedtaskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCompleteTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCompleteTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListCompleteTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCompleteTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListCompleteTaskRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest;{f65e14a3-1a42-49da-8552-2873e52c55eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListCompleteTaskRequest {
    type Vtable = IUserDataTaskListCompleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCompleteTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCompleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest";
}
impl ::core::convert::From<UserDataTaskListCompleteTaskRequest> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListCompleteTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCompleteTaskRequest> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListCompleteTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListCompleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListCompleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListCompleteTaskRequest> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListCompleteTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCompleteTaskRequest> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListCompleteTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListCompleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListCompleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequest {}
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequestEventArgs(::windows_core::IUnknown);
impl UserDataTaskListCompleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListCompleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListCompleteTaskRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCompleteTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCompleteTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListCompleteTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCompleteTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListCompleteTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs;{d77c393d-4cf2-48ad-87fd-963f0eaa7a95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListCompleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCompleteTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCompleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs";
}
impl ::core::convert::From<UserDataTaskListCompleteTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListCompleteTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCompleteTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListCompleteTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListCompleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListCompleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListCompleteTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListCompleteTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCompleteTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListCompleteTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListCompleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListCompleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequestEventArgs {}
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequest(::windows_core::IUnknown);
impl UserDataTaskListCreateOrUpdateTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Task(&self) -> ::windows_core::Result<super::UserDataTask> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataTask>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows_core::IntoParam<'a, super::UserDataTask>>(&self, createdorupdateduserdatatask: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), createdorupdateduserdatatask.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCreateOrUpdateTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCreateOrUpdateTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListCreateOrUpdateTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCreateOrUpdateTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest;{2133772c-55c2-4300-8279-04326e07cce4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListCreateOrUpdateTaskRequest {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest";
}
impl ::core::convert::From<UserDataTaskListCreateOrUpdateTaskRequest> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListCreateOrUpdateTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCreateOrUpdateTaskRequest> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListCreateOrUpdateTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListCreateOrUpdateTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListCreateOrUpdateTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListCreateOrUpdateTaskRequest> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListCreateOrUpdateTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCreateOrUpdateTaskRequest> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListCreateOrUpdateTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListCreateOrUpdateTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListCreateOrUpdateTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequest {}
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(::windows_core::IUnknown);
impl UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListCreateOrUpdateTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListCreateOrUpdateTaskRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCreateOrUpdateTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs;{12c55a52-e378-419b-ae38-a5e9e604476e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs";
}
impl ::core::convert::From<UserDataTaskListCreateOrUpdateTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListCreateOrUpdateTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCreateOrUpdateTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListCreateOrUpdateTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListCreateOrUpdateTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListCreateOrUpdateTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListCreateOrUpdateTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListCreateOrUpdateTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequest(::windows_core::IUnknown);
impl UserDataTaskListDeleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListDeleteTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListDeleteTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListDeleteTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListDeleteTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListDeleteTaskRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest;{4b863c68-7657-4f3d-b074-d47ec8df07e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListDeleteTaskRequest {
    type Vtable = IUserDataTaskListDeleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListDeleteTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListDeleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest";
}
impl ::core::convert::From<UserDataTaskListDeleteTaskRequest> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListDeleteTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListDeleteTaskRequest> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListDeleteTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListDeleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListDeleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListDeleteTaskRequest> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListDeleteTaskRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListDeleteTaskRequest> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListDeleteTaskRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListDeleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListDeleteTaskRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequest {}
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequestEventArgs(::windows_core::IUnknown);
impl UserDataTaskListDeleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListDeleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListDeleteTaskRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListDeleteTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListDeleteTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListDeleteTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListDeleteTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListDeleteTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs;{6063dad9-f562-4145-8efe-d50078c92b7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListDeleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListDeleteTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListDeleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs";
}
impl ::core::convert::From<UserDataTaskListDeleteTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListDeleteTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListDeleteTaskRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListDeleteTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListDeleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListDeleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListDeleteTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListDeleteTaskRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListDeleteTaskRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListDeleteTaskRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListDeleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListDeleteTaskRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequestEventArgs {}
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequest(::windows_core::IUnknown);
impl UserDataTaskListSkipOccurrenceRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSkipOccurrenceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSkipOccurrenceRequest {}
impl ::core::fmt::Debug for UserDataTaskListSkipOccurrenceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSkipOccurrenceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSkipOccurrenceRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest;{ab87e34d-1cd3-431c-9f58-089aa4338d85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListSkipOccurrenceRequest {
    type Vtable = IUserDataTaskListSkipOccurrenceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSkipOccurrenceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSkipOccurrenceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest";
}
impl ::core::convert::From<UserDataTaskListSkipOccurrenceRequest> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListSkipOccurrenceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSkipOccurrenceRequest> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListSkipOccurrenceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListSkipOccurrenceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListSkipOccurrenceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSkipOccurrenceRequest> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListSkipOccurrenceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSkipOccurrenceRequest> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListSkipOccurrenceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListSkipOccurrenceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListSkipOccurrenceRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequest {}
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(::windows_core::IUnknown);
impl UserDataTaskListSkipOccurrenceRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListSkipOccurrenceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListSkipOccurrenceRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSkipOccurrenceRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSkipOccurrenceRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs;{7a3b924a-cc2f-4e7b-aacd-a5b9d29cfa4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListSkipOccurrenceRequestEventArgs {
    type Vtable = IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSkipOccurrenceRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs";
}
impl ::core::convert::From<UserDataTaskListSkipOccurrenceRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListSkipOccurrenceRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSkipOccurrenceRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListSkipOccurrenceRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSkipOccurrenceRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListSkipOccurrenceRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSkipOccurrenceRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListSkipOccurrenceRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequestEventArgs {}
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequest(::windows_core::IUnknown);
impl UserDataTaskListSyncManagerSyncRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManagerSyncRequest {}
impl ::core::fmt::Debug for UserDataTaskListSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSyncManagerSyncRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest;{40a73807-7590-4149-ae19-b211431a9f48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListSyncManagerSyncRequest {
    type Vtable = IUserDataTaskListSyncManagerSyncRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSyncManagerSyncRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest";
}
impl ::core::convert::From<UserDataTaskListSyncManagerSyncRequest> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManagerSyncRequest> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSyncManagerSyncRequest> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManagerSyncRequest> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequest {}
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(::windows_core::IUnknown);
impl UserDataTaskListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataTaskListSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs;{8ead1c12-768e-43bd-8385-5cdc351ffdea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataTaskListSyncManagerSyncRequestEventArgs {
    type Vtable = IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSyncManagerSyncRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs";
}
impl ::core::convert::From<UserDataTaskListSyncManagerSyncRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataTaskListSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManagerSyncRequestEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataTaskListSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSyncManagerSyncRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataTaskListSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManagerSyncRequestEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataTaskListSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequestEventArgs {}