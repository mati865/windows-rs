#[cfg(feature = "DataProvider")]
pub mod DataProvider;
#[repr(transparent)]
pub struct EmailAttachment(::windows_core::IUnknown);
impl EmailAttachment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailAttachment, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FileName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetFileName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFileName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ContentId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContentId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentLocation(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentLocation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetContentLocation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DownloadState(&self) -> ::windows_core::Result<EmailAttachmentDownloadState> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailAttachmentDownloadState>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailAttachmentDownloadState>(result__)
        }
    }
    pub fn SetDownloadState(&self, value: EmailAttachmentDownloadState) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDownloadState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EstimatedDownloadSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedDownloadSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetEstimatedDownloadSizeInBytes(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEstimatedDownloadSizeInBytes)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFromBaseMessage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBaseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInline(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInline(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsInline)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MimeType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MimeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMimeType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMimeType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(filename: Param0, data: Param1) -> ::windows_core::Result<EmailAttachment> {
        Self::IEmailAttachmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), filename.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailAttachment>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(filename: Param0, data: Param1, mimetype: Param2) -> ::windows_core::Result<EmailAttachment> {
        Self::IEmailAttachmentFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), filename.into_param().abi(), data.into_param().abi(), mimetype.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailAttachment>(result__)
        })
    }
    pub fn IEmailAttachmentFactory<R, F: FnOnce(&IEmailAttachmentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailAttachment, IEmailAttachmentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IEmailAttachmentFactory2<R, F: FnOnce(&IEmailAttachmentFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailAttachment, IEmailAttachmentFactory2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailAttachment {}
impl ::core::fmt::Debug for EmailAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailAttachment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailAttachment;{f353caf9-57c8-4adb-b992-60fceb584f54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailAttachment {
    type Vtable = IEmailAttachment_Vtbl;
    const IID: ::windows_core::GUID = <IEmailAttachment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailAttachment";
}
impl ::core::convert::From<EmailAttachment> for ::windows_core::IUnknown {
    fn from(value: EmailAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAttachment> for ::windows_core::IUnknown {
    fn from(value: &EmailAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailAttachment> for ::windows_core::IInspectable {
    fn from(value: EmailAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAttachment> for ::windows_core::IInspectable {
    fn from(value: &EmailAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailAttachment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailAttachment {}
unsafe impl ::core::marker::Sync for EmailAttachment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailAttachmentDownloadState(pub i32);
impl EmailAttachmentDownloadState {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailAttachmentDownloadState {}
impl ::core::clone::Clone for EmailAttachmentDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailAttachmentDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailAttachmentDownloadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailAttachmentDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAttachmentDownloadState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailAttachmentDownloadState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailAttachmentDownloadState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailBatchStatus(pub i32);
impl EmailBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailBatchStatus {}
impl ::core::clone::Clone for EmailBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailBatchStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailBatchStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailBatchStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailBatchStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailBatchStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailBatchStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailCertificateValidationStatus(pub i32);
impl EmailCertificateValidationStatus {
    pub const Success: Self = Self(0i32);
    pub const NoMatch: Self = Self(1i32);
    pub const InvalidUsage: Self = Self(2i32);
    pub const InvalidCertificate: Self = Self(3i32);
    pub const Revoked: Self = Self(4i32);
    pub const ChainRevoked: Self = Self(5i32);
    pub const RevocationServerFailure: Self = Self(6i32);
    pub const Expired: Self = Self(7i32);
    pub const Untrusted: Self = Self(8i32);
    pub const ServerError: Self = Self(9i32);
    pub const UnknownFailure: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailCertificateValidationStatus {}
impl ::core::clone::Clone for EmailCertificateValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailCertificateValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailCertificateValidationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailCertificateValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailCertificateValidationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailCertificateValidationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailCertificateValidationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailConversation(::windows_core::IUnknown);
impl EmailConversation {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MailboxId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MailboxId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FlagState(&self) -> ::windows_core::Result<EmailFlagState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailFlagState>::zeroed();
            (::windows_core::Interface::vtable(this).FlagState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailFlagState>(result__)
        }
    }
    pub fn HasAttachment(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasAttachment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Importance(&self) -> ::windows_core::Result<EmailImportance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailImportance>::zeroed();
            (::windows_core::Interface::vtable(this).Importance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailImportance>(result__)
        }
    }
    pub fn LastEmailResponseKind(&self) -> ::windows_core::Result<EmailMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMessageResponseKind>::zeroed();
            (::windows_core::Interface::vtable(this).LastEmailResponseKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageResponseKind>(result__)
        }
    }
    pub fn MessageCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MessageCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MostRecentMessageId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MostRecentMessageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MostRecentMessageTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).MostRecentMessageTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Preview(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Preview)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LatestSender(&self) -> ::windows_core::Result<EmailRecipient> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LatestSender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailRecipient>(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn UnreadMessageCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UnreadMessageCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMessagesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindMessagesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMessagesWithCountAsync(&self, count: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindMessagesWithCountAsync)(::windows_core::Interface::as_raw(this), count, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailConversation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversation {}
impl ::core::fmt::Debug for EmailConversation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailConversation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversation;{da18c248-a0bc-4349-902d-90f66389f51b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailConversation {
    type Vtable = IEmailConversation_Vtbl;
    const IID: ::windows_core::GUID = <IEmailConversation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversation";
}
impl ::core::convert::From<EmailConversation> for ::windows_core::IUnknown {
    fn from(value: EmailConversation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversation> for ::windows_core::IUnknown {
    fn from(value: &EmailConversation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailConversation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailConversation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailConversation> for ::windows_core::IInspectable {
    fn from(value: EmailConversation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversation> for ::windows_core::IInspectable {
    fn from(value: &EmailConversation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailConversation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailConversation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailConversation {}
unsafe impl ::core::marker::Sync for EmailConversation {}
#[repr(transparent)]
pub struct EmailConversationBatch(::windows_core::IUnknown);
impl EmailConversationBatch {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Conversations(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Conversations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<EmailConversation>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EmailBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailBatchStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailBatchStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailConversationBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailConversationBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversationBatch {}
impl ::core::fmt::Debug for EmailConversationBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversationBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailConversationBatch {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversationBatch;{b8c1ab81-01c5-432a-9df1-fe85d98a279a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailConversationBatch {
    type Vtable = IEmailConversationBatch_Vtbl;
    const IID: ::windows_core::GUID = <IEmailConversationBatch as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailConversationBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversationBatch";
}
impl ::core::convert::From<EmailConversationBatch> for ::windows_core::IUnknown {
    fn from(value: EmailConversationBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversationBatch> for ::windows_core::IUnknown {
    fn from(value: &EmailConversationBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailConversationBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailConversationBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailConversationBatch> for ::windows_core::IInspectable {
    fn from(value: EmailConversationBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversationBatch> for ::windows_core::IInspectable {
    fn from(value: &EmailConversationBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailConversationBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailConversationBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailConversationBatch {}
unsafe impl ::core::marker::Sync for EmailConversationBatch {}
#[repr(transparent)]
pub struct EmailConversationReader(::windows_core::IUnknown);
impl EmailConversationReader {
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailConversationBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailConversationBatch>>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailConversationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversationReader {}
impl ::core::fmt::Debug for EmailConversationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailConversationReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversationReader;{b4630f82-2875-44c8-9b8c-85beb3a3c653})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailConversationReader {
    type Vtable = IEmailConversationReader_Vtbl;
    const IID: ::windows_core::GUID = <IEmailConversationReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversationReader";
}
impl ::core::convert::From<EmailConversationReader> for ::windows_core::IUnknown {
    fn from(value: EmailConversationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversationReader> for ::windows_core::IUnknown {
    fn from(value: &EmailConversationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailConversationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailConversationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailConversationReader> for ::windows_core::IInspectable {
    fn from(value: EmailConversationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailConversationReader> for ::windows_core::IInspectable {
    fn from(value: &EmailConversationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailConversationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailConversationReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailConversationReader {}
unsafe impl ::core::marker::Sync for EmailConversationReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailFlagState(pub i32);
impl EmailFlagState {
    pub const Unflagged: Self = Self(0i32);
    pub const Flagged: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Cleared: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailFlagState {}
impl ::core::clone::Clone for EmailFlagState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailFlagState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailFlagState {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailFlagState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFlagState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailFlagState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailFlagState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailFolder(::windows_core::IUnknown);
impl EmailFolder {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MailboxId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MailboxId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParentFolderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ParentFolderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsSyncEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSyncEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSyncEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<EmailSpecialFolderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailSpecialFolderKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailSpecialFolderKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindChildFoldersAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindChildFoldersAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>>(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    pub fn GetConversationReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMessage>>(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    pub fn GetMessageReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetMessageCountsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailItemCounts>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageCountsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailItemCounts>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailFolder>>(&self, newparentfolder: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveAsync)(::windows_core::Interface::as_raw(this), newparentfolder.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveWithNewNameAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, newparentfolder: Param0, newfoldername: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveWithNewNameAsync)(::windows_core::Interface::as_raw(this), newparentfolder.into_param().abi(), newfoldername.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrySaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailFolder {}
impl ::core::fmt::Debug for EmailFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFolder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailFolder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailFolder;{a24f7771-996c-4864-b1ba-ed1240e57d11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailFolder {
    type Vtable = IEmailFolder_Vtbl;
    const IID: ::windows_core::GUID = <IEmailFolder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailFolder {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailFolder";
}
impl ::core::convert::From<EmailFolder> for ::windows_core::IUnknown {
    fn from(value: EmailFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolder> for ::windows_core::IUnknown {
    fn from(value: &EmailFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailFolder> for ::windows_core::IInspectable {
    fn from(value: EmailFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolder> for ::windows_core::IInspectable {
    fn from(value: &EmailFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailFolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailFolder {}
unsafe impl ::core::marker::Sync for EmailFolder {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailImportance(pub i32);
impl EmailImportance {
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailImportance {}
impl ::core::clone::Clone for EmailImportance {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailImportance {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailImportance {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailImportance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailImportance").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailImportance {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailImportance;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailIrmInfo(::windows_core::IUnknown);
impl EmailIrmInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailIrmInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CanEdit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanEdit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanEdit(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanEdit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanExtractData(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanExtractData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanExtractData(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanExtractData)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanForward(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanForward)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanForward(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanForward)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanModifyRecipientsOnResponse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanModifyRecipientsOnResponse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanModifyRecipientsOnResponse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanModifyRecipientsOnResponse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanPrintData(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanPrintData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanPrintData(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanPrintData)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanRemoveIrmOnResponse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRemoveIrmOnResponse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanRemoveIrmOnResponse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanRemoveIrmOnResponse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanReply(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanReply)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanReply(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanReply)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanReplyAll(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanReplyAll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanReplyAll(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanReplyAll)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationDate<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsIrmOriginator(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIrmOriginator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsIrmOriginator(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsIrmOriginator)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsProgramaticAccessAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProgramaticAccessAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsProgramaticAccessAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsProgramaticAccessAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Template(&self) -> ::windows_core::Result<EmailIrmTemplate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Template)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailIrmTemplate>(result__)
        }
    }
    pub fn SetTemplate<'a, Param0: ::windows_core::IntoParam<'a, EmailIrmTemplate>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTemplate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, EmailIrmTemplate>>(expiration: Param0, irmtemplate: Param1) -> ::windows_core::Result<EmailIrmInfo> {
        Self::IEmailIrmInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), expiration.into_param().abi(), irmtemplate.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailIrmInfo>(result__)
        })
    }
    pub fn IEmailIrmInfoFactory<R, F: FnOnce(&IEmailIrmInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailIrmInfo, IEmailIrmInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailIrmInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailIrmInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailIrmInfo {}
impl ::core::fmt::Debug for EmailIrmInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailIrmInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailIrmInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailIrmInfo;{90f52193-b1a0-4ebd-a6b6-ddca55606e0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailIrmInfo {
    type Vtable = IEmailIrmInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEmailIrmInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailIrmInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailIrmInfo";
}
impl ::core::convert::From<EmailIrmInfo> for ::windows_core::IUnknown {
    fn from(value: EmailIrmInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailIrmInfo> for ::windows_core::IUnknown {
    fn from(value: &EmailIrmInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailIrmInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailIrmInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailIrmInfo> for ::windows_core::IInspectable {
    fn from(value: EmailIrmInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailIrmInfo> for ::windows_core::IInspectable {
    fn from(value: &EmailIrmInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailIrmInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailIrmInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailIrmInfo {}
unsafe impl ::core::marker::Sync for EmailIrmInfo {}
#[repr(transparent)]
pub struct EmailIrmTemplate(::windows_core::IUnknown);
impl EmailIrmTemplate {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailIrmTemplate, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0, name: Param1, description: Param2) -> ::windows_core::Result<EmailIrmTemplate> {
        Self::IEmailIrmTemplateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), id.into_param().abi(), name.into_param().abi(), description.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailIrmTemplate>(result__)
        })
    }
    pub fn IEmailIrmTemplateFactory<R, F: FnOnce(&IEmailIrmTemplateFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailIrmTemplate, IEmailIrmTemplateFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailIrmTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailIrmTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailIrmTemplate {}
impl ::core::fmt::Debug for EmailIrmTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailIrmTemplate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailIrmTemplate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailIrmTemplate;{f327758d-546d-4bea-a963-54a38b2cc016})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailIrmTemplate {
    type Vtable = IEmailIrmTemplate_Vtbl;
    const IID: ::windows_core::GUID = <IEmailIrmTemplate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailIrmTemplate {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailIrmTemplate";
}
impl ::core::convert::From<EmailIrmTemplate> for ::windows_core::IUnknown {
    fn from(value: EmailIrmTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailIrmTemplate> for ::windows_core::IUnknown {
    fn from(value: &EmailIrmTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailIrmTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailIrmTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailIrmTemplate> for ::windows_core::IInspectable {
    fn from(value: EmailIrmTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailIrmTemplate> for ::windows_core::IInspectable {
    fn from(value: &EmailIrmTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailIrmTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailIrmTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailIrmTemplate {}
unsafe impl ::core::marker::Sync for EmailIrmTemplate {}
#[repr(transparent)]
pub struct EmailItemCounts(::windows_core::IUnknown);
impl EmailItemCounts {
    pub fn Flagged(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Flagged)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Important(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Important)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Total(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Total)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Unread(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Unread)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailItemCounts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailItemCounts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailItemCounts {}
impl ::core::fmt::Debug for EmailItemCounts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailItemCounts").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailItemCounts {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailItemCounts;{5bd13321-fec8-4bab-83ba-0baf3c1f6cbd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailItemCounts {
    type Vtable = IEmailItemCounts_Vtbl;
    const IID: ::windows_core::GUID = <IEmailItemCounts as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailItemCounts {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailItemCounts";
}
impl ::core::convert::From<EmailItemCounts> for ::windows_core::IUnknown {
    fn from(value: EmailItemCounts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailItemCounts> for ::windows_core::IUnknown {
    fn from(value: &EmailItemCounts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailItemCounts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailItemCounts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailItemCounts> for ::windows_core::IInspectable {
    fn from(value: EmailItemCounts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailItemCounts> for ::windows_core::IInspectable {
    fn from(value: &EmailItemCounts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailItemCounts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailItemCounts {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailItemCounts {}
unsafe impl ::core::marker::Sync for EmailItemCounts {}
#[repr(transparent)]
pub struct EmailMailbox(::windows_core::IUnknown);
impl EmailMailbox {
    pub fn Capabilities(&self) -> ::windows_core::Result<EmailMailboxCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxCapabilities>(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows_core::Result<EmailMailboxChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxChangeTracker>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsOwnedByCurrentApp(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOwnedByCurrentApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDataEncryptedUnderLock(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDataEncryptedUnderLock)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MailAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MailAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMailAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMailAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MailAddressAliases(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MailAddressAliases)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<EmailMailboxOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxOtherAppReadAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: EmailMailboxOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows_core::Result<EmailMailboxOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxOtherAppWriteAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppWriteAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxOtherAppWriteAccess>(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: EmailMailboxOtherAppWriteAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Policies(&self) -> ::windows_core::Result<EmailMailboxPolicies> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Policies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxPolicies>(result__)
        }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows_core::Result<EmailMailboxSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxSyncManager>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    pub fn GetConversationReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    pub fn GetMessageReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailConversation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSpecialFolderAsync(&self, foldertype: EmailSpecialFolderKind) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSpecialFolderAsync)(::windows_core::Interface::as_raw(this), foldertype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageAsSeenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MarkMessageAsSeenAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkFolderAsSeenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MarkFolderAsSeenAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0, isread: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MarkMessageReadAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), isread, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ChangeMessageFlagStateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0, flagstate: EmailFlagState) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeMessageFlagStateAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), flagstate, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0, newparentfolderid: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveMessageAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), newparentfolderid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0, newparentfolderid: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFolderAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), newparentfolderid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFolderWithNewNameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0, newparentfolderid: Param1, newfoldername: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFolderWithNewNameAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), newparentfolderid.into_param().abi(), newfoldername.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteMessageAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MarkFolderSyncEnabledAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0, issyncenabled: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MarkFolderSyncEnabledAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), issyncenabled, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveDraftAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveDraftAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadMessageAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DownloadAttachmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, attachmentid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadAttachmentAsync)(::windows_core::Interface::as_raw(this), attachmentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateResponseMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, messageid: Param0, responsetype: EmailMessageResponseKind, subject: Param2, responseheadertype: EmailMessageBodyKind, responseheader: Param4) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateResponseMessageAsync)(::windows_core::Interface::as_raw(this), messageid.into_param().abi(), responsetype, subject.into_param().abi(), responseheadertype, responseheader.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateMeetingResponseAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, response: EmailMeetingResponseType, subject: Param2, comment: Param3, sendupdate: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateMeetingResponseAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), response, subject.into_param().abi(), comment.into_param().abi(), sendupdate, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryForwardMeetingAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<EmailRecipient>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param5: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, recipients: Param1, subject: Param2, forwardheadertype: EmailMessageBodyKind, forwardheader: Param4, comment: Param5) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryForwardMeetingAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), recipients.into_param().abi(), subject.into_param().abi(), forwardheadertype, forwardheader.into_param().abi(), comment.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryProposeNewTimeForMeetingAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, meeting: Param0, newstarttime: Param1, newduration: Param2, subject: Param3, comment: Param4) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryProposeNewTimeForMeetingAsync)(::windows_core::Interface::as_raw(this), meeting.into_param().abi(), newstarttime.into_param().abi(), newduration.into_param().abi(), subject.into_param().abi(), comment.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MailboxChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs>>>(&self, phandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MailboxChanged)(::windows_core::Interface::as_raw(this), phandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMailboxChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMailboxChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SmartSendMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(&self, message: Param0, smartsend: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmartSendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), smartsend, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrySetAutoReplySettingsAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMailboxAutoReplySettings>>(&self, autoreplysettings: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetAutoReplySettingsAsync)(::windows_core::Interface::as_raw(this), autoreplysettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryGetAutoReplySettingsAsync(&self, requestedformat: EmailMailboxAutoReplyMessageResponseKind) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAutoReplySettingsAsync)(::windows_core::Interface::as_raw(this), requestedformat, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>>(result__)
        }
    }
    pub fn LinkedMailboxId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LinkedMailboxId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NetworkId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveRecipientsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, recipients: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResolveRecipientsAsync)(::windows_core::Interface::as_raw(this), recipients.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ValidateCertificatesAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>>(&self, certificates: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValidateCertificatesAsync)(::windows_core::Interface::as_raw(this), certificates.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryEmptyFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryEmptyFolderAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryCreateFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, parentfolderid: Param0, name: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFolderAsync)(::windows_core::Interface::as_raw(this), parentfolderid.into_param().abi(), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, folderid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteFolderAsync)(::windows_core::Interface::as_raw(this), folderid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetChangeTracker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, identity: Param0) -> ::windows_core::Result<EmailMailboxChangeTracker> {
        let this = &::windows_core::Interface::cast::<IEmailMailbox5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeTracker)(::windows_core::Interface::as_raw(this), identity.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailMailboxChangeTracker>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailbox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailbox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailbox {}
impl ::core::fmt::Debug for EmailMailbox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailbox").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailbox {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailbox;{a8790649-cf5b-411b-80b1-4a6a1484ce25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailbox {
    type Vtable = IEmailMailbox_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailbox as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailbox {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailbox";
}
impl ::core::convert::From<EmailMailbox> for ::windows_core::IUnknown {
    fn from(value: EmailMailbox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailbox> for ::windows_core::IUnknown {
    fn from(value: &EmailMailbox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailbox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailbox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailbox> for ::windows_core::IInspectable {
    fn from(value: EmailMailbox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailbox> for ::windows_core::IInspectable {
    fn from(value: &EmailMailbox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailbox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailbox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailbox {}
unsafe impl ::core::marker::Sync for EmailMailbox {}
#[repr(transparent)]
pub struct EmailMailboxAction(::windows_core::IUnknown);
impl EmailMailboxAction {
    pub fn Kind(&self) -> ::windows_core::Result<EmailMailboxActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxActionKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxActionKind>(result__)
        }
    }
    pub fn ChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAction {}
impl ::core::fmt::Debug for EmailMailboxAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxAction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAction;{ac9889fa-21fa-4927-9210-d410582fdf3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxAction {
    type Vtable = IEmailMailboxAction_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxAction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxAction {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAction";
}
impl ::core::convert::From<EmailMailboxAction> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAction> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxAction> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAction> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxAction {}
unsafe impl ::core::marker::Sync for EmailMailboxAction {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxActionKind(pub i32);
impl EmailMailboxActionKind {
    pub const MarkMessageAsSeen: Self = Self(0i32);
    pub const MarkMessageRead: Self = Self(1i32);
    pub const ChangeMessageFlagState: Self = Self(2i32);
    pub const MoveMessage: Self = Self(3i32);
    pub const SaveDraft: Self = Self(4i32);
    pub const SendMessage: Self = Self(5i32);
    pub const CreateResponseReplyMessage: Self = Self(6i32);
    pub const CreateResponseReplyAllMessage: Self = Self(7i32);
    pub const CreateResponseForwardMessage: Self = Self(8i32);
    pub const MoveFolder: Self = Self(9i32);
    pub const MarkFolderForSyncEnabled: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailMailboxActionKind {}
impl ::core::clone::Clone for EmailMailboxActionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxActionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxActionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxActionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxActionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxActionKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxActionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(pub i32);
impl EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    pub const None: Self = Self(0i32);
    pub const StrongAlgorithm: Self = Self(1i32);
    pub const AnyAlgorithm: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {}
impl ::core::clone::Clone for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMailboxAutoReply(::windows_core::IUnknown);
impl EmailMailboxAutoReply {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Response(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetResponse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponse)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxAutoReply {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAutoReply {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAutoReply {}
impl ::core::fmt::Debug for EmailMailboxAutoReply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReply").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxAutoReply {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAutoReply;{e223254c-8ab4-485b-b31f-04d15476bd59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxAutoReply {
    type Vtable = IEmailMailboxAutoReply_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxAutoReply as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxAutoReply {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAutoReply";
}
impl ::core::convert::From<EmailMailboxAutoReply> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxAutoReply) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAutoReply> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxAutoReply) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxAutoReply {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxAutoReply {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxAutoReply> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxAutoReply) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAutoReply> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxAutoReply) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxAutoReply {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxAutoReply {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxAutoReply {}
unsafe impl ::core::marker::Sync for EmailMailboxAutoReply {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxAutoReplyMessageResponseKind(pub i32);
impl EmailMailboxAutoReplyMessageResponseKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxAutoReplyMessageResponseKind {}
impl ::core::clone::Clone for EmailMailboxAutoReplyMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxAutoReplyMessageResponseKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxAutoReplyMessageResponseKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxAutoReplyMessageResponseKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReplyMessageResponseKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxAutoReplyMessageResponseKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxAutoReplyMessageResponseKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMailboxAutoReplySettings(::windows_core::IUnknown);
impl EmailMailboxAutoReplySettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailMailboxAutoReplySettings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResponseKind(&self) -> ::windows_core::Result<EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxAutoReplyMessageResponseKind>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxAutoReplyMessageResponseKind>(result__)
        }
    }
    pub fn SetResponseKind(&self, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponseKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn InternalReply(&self) -> ::windows_core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InternalReply)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxAutoReply>(result__)
        }
    }
    pub fn KnownExternalReply(&self) -> ::windows_core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KnownExternalReply)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxAutoReply>(result__)
        }
    }
    pub fn UnknownExternalReply(&self) -> ::windows_core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnknownExternalReply)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxAutoReply>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxAutoReplySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAutoReplySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAutoReplySettings {}
impl ::core::fmt::Debug for EmailMailboxAutoReplySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReplySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxAutoReplySettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAutoReplySettings;{a87a9fa8-0ac6-4b77-ba77-a6b99e9a27b8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxAutoReplySettings {
    type Vtable = IEmailMailboxAutoReplySettings_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxAutoReplySettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxAutoReplySettings {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAutoReplySettings";
}
impl ::core::convert::From<EmailMailboxAutoReplySettings> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxAutoReplySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAutoReplySettings> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxAutoReplySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxAutoReplySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxAutoReplySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxAutoReplySettings> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxAutoReplySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxAutoReplySettings> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxAutoReplySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxAutoReplySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxAutoReplySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxAutoReplySettings {}
unsafe impl ::core::marker::Sync for EmailMailboxAutoReplySettings {}
#[repr(transparent)]
pub struct EmailMailboxCapabilities(::windows_core::IUnknown);
impl EmailMailboxCapabilities {
    pub fn CanForwardMeetings(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanForwardMeetings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanGetAndSetExternalAutoReplies(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGetAndSetExternalAutoReplies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanGetAndSetInternalAutoReplies(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGetAndSetInternalAutoReplies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanUpdateMeetingResponses(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanUpdateMeetingResponses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanServerSearchFolders(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanServerSearchFolders)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanServerSearchMailbox(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanServerSearchMailbox)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanProposeNewTimeForMeetings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanSmartSend(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSmartSend)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanResolveRecipients(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanResolveRecipients)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanValidateCertificates(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanValidateCertificates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanEmptyFolder(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanEmptyFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanCreateFolder(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCreateFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanDeleteFolder(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanDeleteFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanMoveFolder(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanMoveFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanForwardMeetings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanGetAndSetExternalAutoReplies(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanGetAndSetExternalAutoReplies)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanGetAndSetInternalAutoReplies(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanGetAndSetInternalAutoReplies)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanUpdateMeetingResponses)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanServerSearchFolders(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanServerSearchFolders)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanServerSearchMailbox(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanServerSearchMailbox)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanProposeNewTimeForMeetings)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanSmartSend(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSmartSend)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanResolveRecipients(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanResolveRecipients)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanValidateCertificates(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanValidateCertificates)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanEmptyFolder(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanEmptyFolder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanCreateFolder(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanCreateFolder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanDeleteFolder(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanDeleteFolder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanMoveFolder(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanMoveFolder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCapabilities {}
impl ::core::fmt::Debug for EmailMailboxCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxCapabilities;{eedec3a6-89db-4305-82c4-439e0a33da11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxCapabilities {
    type Vtable = IEmailMailboxCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxCapabilities";
}
impl ::core::convert::From<EmailMailboxCapabilities> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxCapabilities> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxCapabilities> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxCapabilities> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxCapabilities {}
unsafe impl ::core::marker::Sync for EmailMailboxCapabilities {}
#[repr(transparent)]
pub struct EmailMailboxChange(::windows_core::IUnknown);
impl EmailMailboxChange {
    pub fn ChangeType(&self) -> ::windows_core::Result<EmailMailboxChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxChangeType>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxChangeType>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MailboxActions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailMailboxAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MailboxActions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailMailboxAction>>(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessage>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<EmailFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailFolder>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChange {}
impl ::core::fmt::Debug for EmailMailboxChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChange;{61edf54b-11ef-400c-adde-8cde65c85e66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxChange {
    type Vtable = IEmailMailboxChange_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxChange as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxChange {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChange";
}
impl ::core::convert::From<EmailMailboxChange> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChange> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxChange> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChange> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxChange {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxChange {}
unsafe impl ::core::marker::Sync for EmailMailboxChange {}
#[repr(transparent)]
pub struct EmailMailboxChangeReader(::windows_core::IUnknown);
impl EmailMailboxChangeReader {
    pub fn AcceptChanges(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChanges)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough<'a, Param0: ::windows_core::IntoParam<'a, EmailMailboxChange>>(&self, lastchangetoacknowledge: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptChangesThrough)(::windows_core::Interface::as_raw(this), lastchangetoacknowledge.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangeReader {}
impl ::core::fmt::Debug for EmailMailboxChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChangeReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangeReader;{bdbd0ebb-c53d-4331-97be-be75a2146a75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxChangeReader {
    type Vtable = IEmailMailboxChangeReader_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxChangeReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangeReader";
}
impl ::core::convert::From<EmailMailboxChangeReader> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangeReader> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxChangeReader> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangeReader> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxChangeReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxChangeReader {}
unsafe impl ::core::marker::Sync for EmailMailboxChangeReader {}
#[repr(transparent)]
pub struct EmailMailboxChangeTracker(::windows_core::IUnknown);
impl EmailMailboxChangeTracker {
    pub fn IsTracking(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTracking)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows_core::Result<EmailMailboxChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetChangeReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxChangeReader>(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangeTracker {}
impl ::core::fmt::Debug for EmailMailboxChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChangeTracker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangeTracker;{7ae48638-5166-42b7-8882-fd21c92bdd4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxChangeTracker {
    type Vtable = IEmailMailboxChangeTracker_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxChangeTracker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangeTracker";
}
impl ::core::convert::From<EmailMailboxChangeTracker> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangeTracker> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxChangeTracker> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangeTracker> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxChangeTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxChangeTracker {}
unsafe impl ::core::marker::Sync for EmailMailboxChangeTracker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxChangeType(pub i32);
impl EmailMailboxChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const FolderCreated: Self = Self(3i32);
    pub const FolderModified: Self = Self(4i32);
    pub const FolderDeleted: Self = Self(5i32);
    pub const ChangeTrackingLost: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxChangeType {}
impl ::core::clone::Clone for EmailMailboxChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChangeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMailboxChangedDeferral(::windows_core::IUnknown);
impl EmailMailboxChangedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangedDeferral {}
impl ::core::fmt::Debug for EmailMailboxChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChangedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangedDeferral;{779a74c1-97c5-4b54-b30d-306232623e6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxChangedDeferral {
    type Vtable = IEmailMailboxChangedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxChangedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangedDeferral";
}
impl ::core::convert::From<EmailMailboxChangedDeferral> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangedDeferral> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxChangedDeferral> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangedDeferral> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxChangedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxChangedDeferral {}
unsafe impl ::core::marker::Sync for EmailMailboxChangedDeferral {}
#[repr(transparent)]
pub struct EmailMailboxChangedEventArgs(::windows_core::IUnknown);
impl EmailMailboxChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<EmailMailboxChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxChangedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangedEventArgs {}
impl ::core::fmt::Debug for EmailMailboxChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangedEventArgs;{3cfd5f6e-01d4-4e4a-a44c-b22dd42ec207})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxChangedEventArgs {
    type Vtable = IEmailMailboxChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangedEventArgs";
}
impl ::core::convert::From<EmailMailboxChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxChangedEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxChangedEventArgs {}
#[repr(transparent)]
pub struct EmailMailboxCreateFolderResult(::windows_core::IUnknown);
impl EmailMailboxCreateFolderResult {
    pub fn Status(&self) -> ::windows_core::Result<EmailMailboxCreateFolderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxCreateFolderStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxCreateFolderStatus>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<EmailFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailFolder>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxCreateFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCreateFolderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCreateFolderResult {}
impl ::core::fmt::Debug for EmailMailboxCreateFolderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxCreateFolderResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxCreateFolderResult;{b228557f-2885-4998-b595-8a2d374ce950})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxCreateFolderResult {
    type Vtable = IEmailMailboxCreateFolderResult_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxCreateFolderResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxCreateFolderResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxCreateFolderResult";
}
impl ::core::convert::From<EmailMailboxCreateFolderResult> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxCreateFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderResult> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxCreateFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxCreateFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxCreateFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxCreateFolderResult> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxCreateFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderResult> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxCreateFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxCreateFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxCreateFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderResult {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxCreateFolderStatus(pub i32);
impl EmailMailboxCreateFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const NameCollision: Self = Self(5i32);
    pub const ServerRejected: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxCreateFolderStatus {}
impl ::core::clone::Clone for EmailMailboxCreateFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxCreateFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxCreateFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxCreateFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxCreateFolderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxCreateFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxDeleteFolderStatus(pub i32);
impl EmailMailboxDeleteFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxDeleteFolderStatus {}
impl ::core::clone::Clone for EmailMailboxDeleteFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxDeleteFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxDeleteFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxDeleteFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDeleteFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxDeleteFolderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxDeleteFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxEmptyFolderStatus(pub i32);
impl EmailMailboxEmptyFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxEmptyFolderStatus {}
impl ::core::clone::Clone for EmailMailboxEmptyFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxEmptyFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxEmptyFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxEmptyFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxEmptyFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxEmptyFolderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxEmptyFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxOtherAppReadAccess(pub i32);
impl EmailMailboxOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppReadAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxOtherAppReadAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxOtherAppWriteAccess(pub i32);
impl EmailMailboxOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppWriteAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxOtherAppWriteAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMailboxPolicies(::windows_core::IUnknown);
impl EmailMailboxPolicies {
    pub fn AllowedSmimeEncryptionAlgorithmNegotiation(&self) -> ::windows_core::Result<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedSmimeEncryptionAlgorithmNegotiation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation>(result__)
        }
    }
    pub fn AllowSmimeSoftCertificates(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowSmimeSoftCertificates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequiredSmimeEncryptionAlgorithm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredSmimeEncryptionAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequiredSmimeSigningAlgorithm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredSmimeSigningAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>(result__)
        }
    }
    pub fn MustEncryptSmimeMessages(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MustEncryptSmimeMessages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MustSignSmimeMessages(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MustSignSmimeMessages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowedSmimeEncryptionAlgorithmNegotiation(&self, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowedSmimeEncryptionAlgorithmNegotiation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAllowSmimeSoftCertificates(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowSmimeSoftCertificates)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetRequiredSmimeEncryptionAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiredSmimeEncryptionAlgorithm)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetRequiredSmimeSigningAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiredSmimeSigningAlgorithm)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetMustEncryptSmimeMessages(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMustEncryptSmimeMessages)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMustSignSmimeMessages(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMustSignSmimeMessages)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxPolicies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxPolicies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxPolicies {}
impl ::core::fmt::Debug for EmailMailboxPolicies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxPolicies").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxPolicies {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxPolicies;{1f3345c5-1c3b-4dc7-b410-6373783e545d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxPolicies {
    type Vtable = IEmailMailboxPolicies_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxPolicies as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxPolicies {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxPolicies";
}
impl ::core::convert::From<EmailMailboxPolicies> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxPolicies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxPolicies> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxPolicies) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxPolicies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxPolicies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxPolicies> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxPolicies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxPolicies> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxPolicies) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxPolicies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxPolicies {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxPolicies {}
unsafe impl ::core::marker::Sync for EmailMailboxPolicies {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(pub i32);
impl EmailMailboxSmimeEncryptionAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const TripleDes: Self = Self(1i32);
    pub const Des: Self = Self(2i32);
    pub const RC2128Bit: Self = Self(3i32);
    pub const RC264Bit: Self = Self(4i32);
    pub const RC240Bit: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeEncryptionAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSmimeEncryptionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxSmimeEncryptionAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxSmimeEncryptionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSmimeEncryptionAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxSmimeEncryptionAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSmimeEncryptionAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxSmimeSigningAlgorithm(pub i32);
impl EmailMailboxSmimeSigningAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const Sha1: Self = Self(1i32);
    pub const MD5: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeSigningAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeSigningAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSmimeSigningAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxSmimeSigningAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxSmimeSigningAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSmimeSigningAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxSmimeSigningAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSmimeSigningAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMailboxSyncManager(::windows_core::IUnknown);
impl EmailMailboxSyncManager {
    pub fn Status(&self) -> ::windows_core::Result<EmailMailboxSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMailboxSyncStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMailboxSyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SyncAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SyncStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetStatus(&self, value: EmailMailboxSyncStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for EmailMailboxSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSyncManager {}
impl ::core::fmt::Debug for EmailMailboxSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxSyncManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxSyncManager;{517ac55a-3591-4b5d-85bc-c71dde862263})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMailboxSyncManager {
    type Vtable = IEmailMailboxSyncManager_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMailboxSyncManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMailboxSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxSyncManager";
}
impl ::core::convert::From<EmailMailboxSyncManager> for ::windows_core::IUnknown {
    fn from(value: EmailMailboxSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxSyncManager> for ::windows_core::IUnknown {
    fn from(value: &EmailMailboxSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMailboxSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMailboxSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMailboxSyncManager> for ::windows_core::IInspectable {
    fn from(value: EmailMailboxSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMailboxSyncManager> for ::windows_core::IInspectable {
    fn from(value: &EmailMailboxSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMailboxSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMailboxSyncManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMailboxSyncManager {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMailboxSyncStatus(pub i32);
impl EmailMailboxSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxSyncStatus {}
impl ::core::clone::Clone for EmailMailboxSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMailboxSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMailboxSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMailboxSyncStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct EmailManager;
impl EmailManager {
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeNewEmailAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IEmailManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowComposeNewEmailAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(accesstype: EmailStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailStore>> {
        Self::IEmailManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailStore>>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows_core::Result<EmailManagerForUser> {
        Self::IEmailManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailManagerForUser>(result__)
        })
    }
    pub fn IEmailManagerStatics<R, F: FnOnce(&IEmailManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailManager, IEmailManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IEmailManagerStatics2<R, F: FnOnce(&IEmailManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailManager, IEmailManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IEmailManagerStatics3<R, F: FnOnce(&IEmailManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailManager, IEmailManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for EmailManager {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailManager";
}
#[repr(transparent)]
pub struct EmailManagerForUser(::windows_core::IUnknown);
impl EmailManagerForUser {
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeNewEmailAsync<'a, Param0: ::windows_core::IntoParam<'a, EmailMessage>>(&self, message: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowComposeNewEmailAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), accesstype, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailManagerForUser {}
impl ::core::fmt::Debug for EmailManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailManagerForUser;{f773de9f-3ca5-4b0f-90c1-156e40174ce5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailManagerForUser {
    type Vtable = IEmailManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IEmailManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailManagerForUser";
}
impl ::core::convert::From<EmailManagerForUser> for ::windows_core::IUnknown {
    fn from(value: EmailManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &EmailManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailManagerForUser> for ::windows_core::IInspectable {
    fn from(value: EmailManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &EmailManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailManagerForUser {}
unsafe impl ::core::marker::Sync for EmailManagerForUser {}
#[repr(transparent)]
pub struct EmailMeetingInfo(::windows_core::IUnknown);
impl EmailMeetingInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailMeetingInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowNewTimeProposal(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowNewTimeProposal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowNewTimeProposal)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppointmentRoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentRoamingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAppointmentRoamingId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppointmentRoamingId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentOriginalStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetAppointmentOriginalStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppointmentOriginalStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsAllDay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAllDay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAllDay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAllDay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsResponseRequested(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsResponseRequested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsResponseRequested)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Location(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLocation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProposedStartTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProposedStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetProposedStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, proposedstarttime: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProposedStartTime)(::windows_core::Interface::as_raw(this), proposedstarttime.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProposedDuration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProposedDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetProposedDuration<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, duration: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProposedDuration)(::windows_core::Interface::as_raw(this), duration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecurrenceStartTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecurrenceStartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetRecurrenceStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecurrenceStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Recurrence(&self) -> ::windows_core::Result<super::Appointments::AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Recurrence)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentRecurrence>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn SetRecurrence<'a, Param0: ::windows_core::IntoParam<'a, super::Appointments::AppointmentRecurrence>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecurrence)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoteChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteChangeNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsReportedOutOfDateByServer(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMeetingInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReportedOutOfDateByServer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMeetingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMeetingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMeetingInfo {}
impl ::core::fmt::Debug for EmailMeetingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMeetingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMeetingInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMeetingInfo;{31c03fa9-7933-415f-a275-d165ba07026b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMeetingInfo {
    type Vtable = IEmailMeetingInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMeetingInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMeetingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMeetingInfo";
}
impl ::core::convert::From<EmailMeetingInfo> for ::windows_core::IUnknown {
    fn from(value: EmailMeetingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMeetingInfo> for ::windows_core::IUnknown {
    fn from(value: &EmailMeetingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMeetingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMeetingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMeetingInfo> for ::windows_core::IInspectable {
    fn from(value: EmailMeetingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMeetingInfo> for ::windows_core::IInspectable {
    fn from(value: &EmailMeetingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMeetingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMeetingInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMeetingInfo {}
unsafe impl ::core::marker::Sync for EmailMeetingInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMeetingResponseType(pub i32);
impl EmailMeetingResponseType {
    pub const Accept: Self = Self(0i32);
    pub const Decline: Self = Self(1i32);
    pub const Tentative: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMeetingResponseType {}
impl ::core::clone::Clone for EmailMeetingResponseType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMeetingResponseType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMeetingResponseType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMeetingResponseType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMeetingResponseType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMeetingResponseType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMeetingResponseType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMessage(::windows_core::IUnknown);
impl EmailMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSubject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubject)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetBody<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBody)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn To(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailRecipient>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CC(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CC)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailRecipient>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bcc(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bcc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailRecipient>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailAttachment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Attachments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailAttachment>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MailboxId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MailboxId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ConversationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConversationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FolderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FolderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AllowInternetImages(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowInternetImages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowInternetImages(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowInternetImages)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn DownloadState(&self) -> ::windows_core::Result<EmailMessageDownloadState> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMessageDownloadState>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageDownloadState>(result__)
        }
    }
    pub fn SetDownloadState(&self, value: EmailMessageDownloadState) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDownloadState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EstimatedDownloadSizeInBytes(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedDownloadSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetEstimatedDownloadSizeInBytes(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEstimatedDownloadSizeInBytes)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FlagState(&self) -> ::windows_core::Result<EmailFlagState> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailFlagState>::zeroed();
            (::windows_core::Interface::vtable(this).FlagState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailFlagState>(result__)
        }
    }
    pub fn SetFlagState(&self, value: EmailFlagState) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFlagState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HasPartialBodies(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasPartialBodies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Importance(&self) -> ::windows_core::Result<EmailImportance> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailImportance>::zeroed();
            (::windows_core::Interface::vtable(this).Importance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailImportance>(result__)
        }
    }
    pub fn SetImportance(&self, value: EmailImportance) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImportance)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InResponseToMessageId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InResponseToMessageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IrmInfo(&self) -> ::windows_core::Result<EmailIrmInfo> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IrmInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailIrmInfo>(result__)
        }
    }
    pub fn SetIrmInfo<'a, Param0: ::windows_core::IntoParam<'a, EmailIrmInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIrmInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDraftMessage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDraftMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRead(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRead)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSeen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSeen)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsServerSearchMessage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsServerSearchMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSmartSendable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSmartSendable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MessageClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMessageClass<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMessageClass)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NormalizedSubject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedSubject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn OriginalCodePage(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalCodePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOriginalCodePage(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOriginalCodePage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Preview(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Preview)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPreview<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreview)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LastResponseKind(&self) -> ::windows_core::Result<EmailMessageResponseKind> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMessageResponseKind>::zeroed();
            (::windows_core::Interface::vtable(this).LastResponseKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageResponseKind>(result__)
        }
    }
    pub fn SetLastResponseKind(&self, value: EmailMessageResponseKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLastResponseKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Sender(&self) -> ::windows_core::Result<EmailRecipient> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailRecipient>(result__)
        }
    }
    pub fn SetSender<'a, Param0: ::windows_core::IntoParam<'a, EmailRecipient>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSender)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SentTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SentTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSentTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSentTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MeetingInfo(&self) -> ::windows_core::Result<EmailMeetingInfo> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MeetingInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMeetingInfo>(result__)
        }
    }
    pub fn SetMeetingInfo<'a, Param0: ::windows_core::IntoParam<'a, EmailMeetingInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMeetingInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBodyStream(&self, r#type: EmailMessageBodyKind) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBodyStream)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBodyStream<'a, Param1: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, r#type: EmailMessageBodyKind, stream: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBodyStream)(::windows_core::Interface::as_raw(this), r#type, stream.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SmimeData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<IEmailMessage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmimeData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSmimeData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSmimeData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SmimeKind(&self) -> ::windows_core::Result<EmailMessageSmimeKind> {
        let this = &::windows_core::Interface::cast::<IEmailMessage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailMessageSmimeKind>::zeroed();
            (::windows_core::Interface::vtable(this).SmimeKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageSmimeKind>(result__)
        }
    }
    pub fn SetSmimeKind(&self, value: EmailMessageSmimeKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSmimeKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplyTo(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = &::windows_core::Interface::cast::<IEmailMessage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplyTo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<EmailRecipient>>(result__)
        }
    }
    pub fn SentRepresenting(&self) -> ::windows_core::Result<EmailRecipient> {
        let this = &::windows_core::Interface::cast::<IEmailMessage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SentRepresenting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailRecipient>(result__)
        }
    }
    pub fn SetSentRepresenting<'a, Param0: ::windows_core::IntoParam<'a, EmailRecipient>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailMessage4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSentRepresenting)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for EmailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessage {}
impl ::core::fmt::Debug for EmailMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessage;{6c6d948d-80b5-48f8-b0b1-e04e430f44e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMessage {
    type Vtable = IEmailMessage_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessage";
}
impl ::core::convert::From<EmailMessage> for ::windows_core::IUnknown {
    fn from(value: EmailMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessage> for ::windows_core::IUnknown {
    fn from(value: &EmailMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMessage> for ::windows_core::IInspectable {
    fn from(value: EmailMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessage> for ::windows_core::IInspectable {
    fn from(value: &EmailMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMessage {}
unsafe impl ::core::marker::Sync for EmailMessage {}
#[repr(transparent)]
pub struct EmailMessageBatch(::windows_core::IUnknown);
impl EmailMessageBatch {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Messages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Messages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<EmailMessage>>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<EmailBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailBatchStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailBatchStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMessageBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMessageBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessageBatch {}
impl ::core::fmt::Debug for EmailMessageBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageBatch {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessageBatch;{605cd08f-25d9-4f1b-9e51-0514c0149653})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMessageBatch {
    type Vtable = IEmailMessageBatch_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMessageBatch as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMessageBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessageBatch";
}
impl ::core::convert::From<EmailMessageBatch> for ::windows_core::IUnknown {
    fn from(value: EmailMessageBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessageBatch> for ::windows_core::IUnknown {
    fn from(value: &EmailMessageBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMessageBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMessageBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMessageBatch> for ::windows_core::IInspectable {
    fn from(value: EmailMessageBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessageBatch> for ::windows_core::IInspectable {
    fn from(value: &EmailMessageBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMessageBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMessageBatch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMessageBatch {}
unsafe impl ::core::marker::Sync for EmailMessageBatch {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMessageBodyKind(pub i32);
impl EmailMessageBodyKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMessageBodyKind {}
impl ::core::clone::Clone for EmailMessageBodyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageBodyKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMessageBodyKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMessageBodyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageBodyKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageBodyKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageBodyKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMessageDownloadState(pub i32);
impl EmailMessageDownloadState {
    pub const PartiallyDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageDownloadState {}
impl ::core::clone::Clone for EmailMessageDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMessageDownloadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMessageDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageDownloadState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageDownloadState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageDownloadState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailMessageReader(::windows_core::IUnknown);
impl EmailMessageReader {
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMessageBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadBatchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMessageBatch>>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMessageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessageReader {}
impl ::core::fmt::Debug for EmailMessageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessageReader;{2f4abe9f-6213-4a85-a3b0-f92d1a839d19})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailMessageReader {
    type Vtable = IEmailMessageReader_Vtbl;
    const IID: ::windows_core::GUID = <IEmailMessageReader as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessageReader";
}
impl ::core::convert::From<EmailMessageReader> for ::windows_core::IUnknown {
    fn from(value: EmailMessageReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessageReader> for ::windows_core::IUnknown {
    fn from(value: &EmailMessageReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailMessageReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailMessageReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailMessageReader> for ::windows_core::IInspectable {
    fn from(value: EmailMessageReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailMessageReader> for ::windows_core::IInspectable {
    fn from(value: &EmailMessageReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailMessageReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailMessageReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailMessageReader {}
unsafe impl ::core::marker::Sync for EmailMessageReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMessageResponseKind(pub i32);
impl EmailMessageResponseKind {
    pub const None: Self = Self(0i32);
    pub const Reply: Self = Self(1i32);
    pub const ReplyAll: Self = Self(2i32);
    pub const Forward: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageResponseKind {}
impl ::core::clone::Clone for EmailMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageResponseKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMessageResponseKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMessageResponseKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageResponseKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageResponseKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageResponseKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailMessageSmimeKind(pub i32);
impl EmailMessageSmimeKind {
    pub const None: Self = Self(0i32);
    pub const ClearSigned: Self = Self(1i32);
    pub const OpaqueSigned: Self = Self(2i32);
    pub const Encrypted: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageSmimeKind {}
impl ::core::clone::Clone for EmailMessageSmimeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageSmimeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailMessageSmimeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailMessageSmimeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageSmimeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailMessageSmimeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageSmimeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailQueryKind(pub i32);
impl EmailQueryKind {
    pub const All: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Flagged: Self = Self(2i32);
    pub const Unread: Self = Self(3i32);
    pub const Read: Self = Self(4i32);
    pub const Unseen: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailQueryKind {}
impl ::core::clone::Clone for EmailQueryKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQueryKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailQueryKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailQueryKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQueryKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQueryKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailQueryOptions(::windows_core::IUnknown);
impl EmailQueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailQueryOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TextSearch(&self) -> ::windows_core::Result<EmailQueryTextSearch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextSearch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQueryTextSearch>(result__)
        }
    }
    pub fn SortDirection(&self) -> ::windows_core::Result<EmailQuerySortDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailQuerySortDirection>::zeroed();
            (::windows_core::Interface::vtable(this).SortDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQuerySortDirection>(result__)
        }
    }
    pub fn SetSortDirection(&self, value: EmailQuerySortDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSortDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SortProperty(&self) -> ::windows_core::Result<EmailQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailQuerySortProperty>::zeroed();
            (::windows_core::Interface::vtable(this).SortProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQuerySortProperty>(result__)
        }
    }
    pub fn SetSortProperty(&self, value: EmailQuerySortProperty) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSortProperty)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<EmailQueryKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailQueryKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQueryKind>(result__)
        }
    }
    pub fn SetKind(&self, value: EmailQueryKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FolderIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FolderIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CreateWithText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0) -> ::windows_core::Result<EmailQueryOptions> {
        Self::IEmailQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithText)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailQueryOptions>(result__)
        })
    }
    pub fn CreateWithTextAndFields<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0, fields: EmailQuerySearchFields) -> ::windows_core::Result<EmailQueryOptions> {
        Self::IEmailQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTextAndFields)(::windows_core::Interface::as_raw(this), text.into_param().abi(), fields, result__.as_mut_ptr()).from_abi::<EmailQueryOptions>(result__)
        })
    }
    pub fn IEmailQueryOptionsFactory<R, F: FnOnce(&IEmailQueryOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailQueryOptions, IEmailQueryOptionsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailQueryOptions {}
impl ::core::fmt::Debug for EmailQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQueryOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailQueryOptions;{45504b9b-3e7f-4d52-b6dd-d6fd4e1fbd9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailQueryOptions {
    type Vtable = IEmailQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = <IEmailQueryOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailQueryOptions";
}
impl ::core::convert::From<EmailQueryOptions> for ::windows_core::IUnknown {
    fn from(value: EmailQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailQueryOptions> for ::windows_core::IUnknown {
    fn from(value: &EmailQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailQueryOptions> for ::windows_core::IInspectable {
    fn from(value: EmailQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailQueryOptions> for ::windows_core::IInspectable {
    fn from(value: &EmailQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailQueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailQueryOptions {}
unsafe impl ::core::marker::Sync for EmailQueryOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailQuerySearchFields(pub u32);
impl EmailQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Subject: Self = Self(1u32);
    pub const Sender: Self = Self(2u32);
    pub const Preview: Self = Self(4u32);
    pub const Recipients: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for EmailQuerySearchFields {}
impl ::core::clone::Clone for EmailQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySearchFields {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailQuerySearchFields {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailQuerySearchFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySearchFields").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EmailQuerySearchFields {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EmailQuerySearchFields {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EmailQuerySearchFields {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EmailQuerySearchFields {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EmailQuerySearchFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQuerySearchFields {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySearchFields;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailQuerySearchScope(pub i32);
impl EmailQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySearchScope {}
impl ::core::clone::Clone for EmailQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySearchScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailQuerySearchScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailQuerySearchScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySearchScope").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQuerySearchScope {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySearchScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailQuerySortDirection(pub i32);
impl EmailQuerySortDirection {
    pub const Descending: Self = Self(0i32);
    pub const Ascending: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySortDirection {}
impl ::core::clone::Clone for EmailQuerySortDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySortDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailQuerySortDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailQuerySortDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySortDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQuerySortDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySortDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailQuerySortProperty(pub i32);
impl EmailQuerySortProperty {
    pub const Date: Self = Self(0i32);
}
impl ::core::marker::Copy for EmailQuerySortProperty {}
impl ::core::clone::Clone for EmailQuerySortProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySortProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailQuerySortProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailQuerySortProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySortProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQuerySortProperty {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySortProperty;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailQueryTextSearch(::windows_core::IUnknown);
impl EmailQueryTextSearch {
    pub fn Fields(&self) -> ::windows_core::Result<EmailQuerySearchFields> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailQuerySearchFields>::zeroed();
            (::windows_core::Interface::vtable(this).Fields)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQuerySearchFields>(result__)
        }
    }
    pub fn SetFields(&self, value: EmailQuerySearchFields) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFields)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SearchScope(&self) -> ::windows_core::Result<EmailQuerySearchScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailQuerySearchScope>::zeroed();
            (::windows_core::Interface::vtable(this).SearchScope)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailQuerySearchScope>(result__)
        }
    }
    pub fn SetSearchScope(&self, value: EmailQuerySearchScope) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchScope)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for EmailQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailQueryTextSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailQueryTextSearch {}
impl ::core::fmt::Debug for EmailQueryTextSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryTextSearch").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailQueryTextSearch {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailQueryTextSearch;{9fa0a288-3c5d-46a5-a6e2-31d6fd17e540})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailQueryTextSearch {
    type Vtable = IEmailQueryTextSearch_Vtbl;
    const IID: ::windows_core::GUID = <IEmailQueryTextSearch as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailQueryTextSearch";
}
impl ::core::convert::From<EmailQueryTextSearch> for ::windows_core::IUnknown {
    fn from(value: EmailQueryTextSearch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailQueryTextSearch> for ::windows_core::IUnknown {
    fn from(value: &EmailQueryTextSearch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailQueryTextSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailQueryTextSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailQueryTextSearch> for ::windows_core::IInspectable {
    fn from(value: EmailQueryTextSearch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailQueryTextSearch> for ::windows_core::IInspectable {
    fn from(value: &EmailQueryTextSearch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailQueryTextSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailQueryTextSearch {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailQueryTextSearch {}
unsafe impl ::core::marker::Sync for EmailQueryTextSearch {}
#[repr(transparent)]
pub struct EmailRecipient(::windows_core::IUnknown);
impl EmailRecipient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailRecipient, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
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
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAddress<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(address: Param0) -> ::windows_core::Result<EmailRecipient> {
        Self::IEmailRecipientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), address.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailRecipient>(result__)
        })
    }
    pub fn CreateWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(address: Param0, name: Param1) -> ::windows_core::Result<EmailRecipient> {
        Self::IEmailRecipientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithName)(::windows_core::Interface::as_raw(this), address.into_param().abi(), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailRecipient>(result__)
        })
    }
    pub fn IEmailRecipientFactory<R, F: FnOnce(&IEmailRecipientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailRecipient, IEmailRecipientFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EmailRecipient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailRecipient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailRecipient {}
impl ::core::fmt::Debug for EmailRecipient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailRecipient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailRecipient;{cae825b3-4478-4814-b900-c902b5e19b53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailRecipient {
    type Vtable = IEmailRecipient_Vtbl;
    const IID: ::windows_core::GUID = <IEmailRecipient as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailRecipient {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailRecipient";
}
impl ::core::convert::From<EmailRecipient> for ::windows_core::IUnknown {
    fn from(value: EmailRecipient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailRecipient> for ::windows_core::IUnknown {
    fn from(value: &EmailRecipient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailRecipient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailRecipient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailRecipient> for ::windows_core::IInspectable {
    fn from(value: EmailRecipient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailRecipient> for ::windows_core::IInspectable {
    fn from(value: &EmailRecipient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailRecipient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailRecipient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailRecipient {}
unsafe impl ::core::marker::Sync for EmailRecipient {}
#[repr(transparent)]
pub struct EmailRecipientResolutionResult(::windows_core::IUnknown);
impl EmailRecipientResolutionResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EmailRecipientResolutionResult, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Status(&self) -> ::windows_core::Result<EmailRecipientResolutionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EmailRecipientResolutionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailRecipientResolutionStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn PublicKeys(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PublicKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    pub fn SetStatus(&self, value: EmailRecipientResolutionStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailRecipientResolutionResult2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn SetPublicKeys<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IEmailRecipientResolutionResult2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPublicKeys)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for EmailRecipientResolutionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailRecipientResolutionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailRecipientResolutionResult {}
impl ::core::fmt::Debug for EmailRecipientResolutionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipientResolutionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailRecipientResolutionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailRecipientResolutionResult;{918338fa-8d8d-4573-80d1-07172a34b98d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailRecipientResolutionResult {
    type Vtable = IEmailRecipientResolutionResult_Vtbl;
    const IID: ::windows_core::GUID = <IEmailRecipientResolutionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailRecipientResolutionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailRecipientResolutionResult";
}
impl ::core::convert::From<EmailRecipientResolutionResult> for ::windows_core::IUnknown {
    fn from(value: EmailRecipientResolutionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailRecipientResolutionResult> for ::windows_core::IUnknown {
    fn from(value: &EmailRecipientResolutionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailRecipientResolutionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailRecipientResolutionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailRecipientResolutionResult> for ::windows_core::IInspectable {
    fn from(value: EmailRecipientResolutionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailRecipientResolutionResult> for ::windows_core::IInspectable {
    fn from(value: &EmailRecipientResolutionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailRecipientResolutionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailRecipientResolutionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailRecipientResolutionResult {}
unsafe impl ::core::marker::Sync for EmailRecipientResolutionResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailRecipientResolutionStatus(pub i32);
impl EmailRecipientResolutionStatus {
    pub const Success: Self = Self(0i32);
    pub const RecipientNotFound: Self = Self(1i32);
    pub const AmbiguousRecipient: Self = Self(2i32);
    pub const NoCertificate: Self = Self(3i32);
    pub const CertificateRequestLimitReached: Self = Self(4i32);
    pub const CannotResolveDistributionList: Self = Self(5i32);
    pub const ServerError: Self = Self(6i32);
    pub const UnknownFailure: Self = Self(7i32);
}
impl ::core::marker::Copy for EmailRecipientResolutionStatus {}
impl ::core::clone::Clone for EmailRecipientResolutionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailRecipientResolutionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailRecipientResolutionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailRecipientResolutionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipientResolutionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailRecipientResolutionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailRecipientResolutionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailSpecialFolderKind(pub i32);
impl EmailSpecialFolderKind {
    pub const None: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const Inbox: Self = Self(2i32);
    pub const Outbox: Self = Self(3i32);
    pub const Drafts: Self = Self(4i32);
    pub const DeletedItems: Self = Self(5i32);
    pub const Sent: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailSpecialFolderKind {}
impl ::core::clone::Clone for EmailSpecialFolderKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailSpecialFolderKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailSpecialFolderKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailSpecialFolderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailSpecialFolderKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailSpecialFolderKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailSpecialFolderKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailStore(::windows_core::IUnknown);
impl EmailStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMailboxesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindMailboxesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>>(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    pub fn GetConversationReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailConversationReader>(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    pub fn GetMessageReaderWithOptions<'a, Param0: ::windows_core::IntoParam<'a, EmailQueryOptions>>(&self, options: Param0) -> ::windows_core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<EmailMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetMailboxAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMailboxAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailbox>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConversationAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailConversation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateMailboxAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, accountname: Param0, accountaddress: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMailboxAsync)(::windows_core::Interface::as_raw(this), accountname.into_param().abi(), accountaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailbox>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateMailboxInAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, accountname: Param0, accountaddress: Param1, userdataaccountid: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateMailboxInAccountAsync)(::windows_core::Interface::as_raw(this), accountname.into_param().abi(), accountaddress.into_param().abi(), userdataaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EmailMailbox>>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStore {}
impl ::core::fmt::Debug for EmailStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailStore;{f803226e-9137-4f8b-a470-279ac3058eb6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailStore {
    type Vtable = IEmailStore_Vtbl;
    const IID: ::windows_core::GUID = <IEmailStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailStore {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailStore";
}
impl ::core::convert::From<EmailStore> for ::windows_core::IUnknown {
    fn from(value: EmailStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStore> for ::windows_core::IUnknown {
    fn from(value: &EmailStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailStore> for ::windows_core::IInspectable {
    fn from(value: EmailStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStore> for ::windows_core::IInspectable {
    fn from(value: &EmailStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailStore {}
unsafe impl ::core::marker::Sync for EmailStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EmailStoreAccessType(pub i32);
impl EmailStoreAccessType {
    pub const AppMailboxesReadWrite: Self = Self(0i32);
    pub const AllMailboxesLimitedReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailStoreAccessType {}
impl ::core::clone::Clone for EmailStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EmailStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmailStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailStoreAccessType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EmailStoreNotificationTriggerDetails(::windows_core::IUnknown);
impl EmailStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for EmailStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EmailStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailStoreNotificationTriggerDetails;{ce17563c-46e6-43c9-96f7-facf7dd710cb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EmailStoreNotificationTriggerDetails {
    type Vtable = IEmailStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IEmailStoreNotificationTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EmailStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailStoreNotificationTriggerDetails";
}
impl ::core::convert::From<EmailStoreNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: EmailStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStoreNotificationTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &EmailStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EmailStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EmailStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailStoreNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: EmailStoreNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailStoreNotificationTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &EmailStoreNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EmailStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EmailStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EmailStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for EmailStoreNotificationTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAttachment {
    type Vtable = IEmailAttachment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf353caf9_57c8_4adb_b992_60fceb584f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAttachment2 {
    type Vtable = IEmailAttachment2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x225f1070_b0ff_4571_9d54_a706c48d55c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachment2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailAttachmentDownloadState) -> ::windows_core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailAttachmentDownloadState) -> ::windows_core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub IsFromBaseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsInline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAttachmentFactory {
    type Vtable = IEmailAttachmentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x796eac46_ed56_4979_8708_abb8bc854b7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachmentFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAttachmentFactory2 {
    type Vtable = IEmailAttachmentFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23259435_51f9_427d_adcd_241023c8cfb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachmentFactory2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr, mimetype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailConversation {
    type Vtable = IEmailConversation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda18c248_a0bc_4349_902d_90f66389f51b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows_core::HRESULT,
    pub HasAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows_core::HRESULT,
    pub LastEmailResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MostRecentMessageTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MostRecentMessageTime: usize,
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LatestSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UnreadMessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversationBatch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailConversationBatch {
    type Vtable = IEmailConversationBatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8c1ab81_01c5_432a_9df1_fe85d98a279a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversationBatch_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Conversations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Conversations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversationReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailConversationReader {
    type Vtable = IEmailConversationReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4630f82_2875_44c8_9b8c_85beb3a3c653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversationReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailFolder {
    type Vtable = IEmailFolder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa24f7771_996c_4864_b1ba_ed1240e57d11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailSpecialFolderKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindChildFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindChildFoldersAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageCountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageCountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentfolder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveWithNewNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentfolder: ::windows_core::RawPtr, newfoldername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailIrmInfo {
    type Vtable = IEmailIrmInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90f52193_b1a0_4ebd_a6b6_ddca55606e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanExtractData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanExtractData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanPrintData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanPrintData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanReplyAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanReplyAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationDate: usize,
    pub IsIrmOriginator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsIrmOriginator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailIrmInfoFactory {
    type Vtable = IEmailIrmInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x314bb18c_e3e6_4d7b_be8d_91a96311b01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expiration: super::super::Foundation::DateTime, irmtemplate: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmTemplate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailIrmTemplate {
    type Vtable = IEmailIrmTemplate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf327758d_546d_4bea_a963_54a38b2cc016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmTemplate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmTemplateFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailIrmTemplateFactory {
    type Vtable = IEmailIrmTemplateFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3da31876_8738_4418_b9cb_471b936fe71e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmTemplateFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailItemCounts(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailItemCounts {
    type Vtable = IEmailItemCounts_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bd13321_fec8_4bab_83ba_0baf3c1f6cbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailItemCounts_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Flagged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Important: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Unread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailbox {
    type Vtable = IEmailMailbox_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8790649_cf5b_411b_80b1_4a6a1484ce25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOwnedByCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDataEncryptedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailAddressAliases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailAddressAliases: usize,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub Policies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSpecialFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldertype: EmailSpecialFolderKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSpecialFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, isread: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeMessageFlagStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, flagstate: EmailFlagState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeMessageFlagStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderWithNewNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newparentfolderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, newfoldername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderSyncEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, issyncenabled: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderSyncEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveDraftAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveDraftAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadAttachmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadAttachmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateResponseMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, responsetype: EmailMessageResponseKind, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, responseheadertype: EmailMessageBodyKind, responseheader: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateResponseMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, response: EmailMeetingResponseType, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sendupdate: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, recipients: ::windows_core::RawPtr, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, forwardheadertype: EmailMessageBodyKind, forwardheader: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: ::windows_core::RawPtr, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, comment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MailboxChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMailboxChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SmartSendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, smartsend: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SmartSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoreplysettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetAutoReplySettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedformat: EmailMailboxAutoReplyMessageResponseKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAutoReplySettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailbox2 {
    type Vtable = IEmailMailbox2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14f8e404_6ca2_4ab2_9241_79cd7bf46346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LinkedMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailbox3 {
    type Vtable = IEmailMailbox3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3da5897b_458b_408a_8e37_ac8b05d8af56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveRecipientsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recipients: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveRecipientsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ValidateCertificatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificates: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ValidateCertificatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEmptyFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEmptyFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailbox4 {
    type Vtable = IEmailMailbox4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d1f301b_f222_48a7_b7b6_716356cd26a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailbox5 {
    type Vtable = IEmailMailbox5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39967087_0092_49be_bd0e_5d4dc9d96d90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxAction {
    type Vtable = IEmailMailboxAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac9889fa_21fa_4927_9210_d410582fdf3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxActionKind) -> ::windows_core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAutoReply(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxAutoReply {
    type Vtable = IEmailMailboxAutoReply_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe223254c_8ab4_485b_b31f_04d15476bd59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAutoReply_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAutoReplySettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxAutoReplySettings {
    type Vtable = IEmailMailboxAutoReplySettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa87a9fa8_0ac6_4b77_ba77_a6b99e9a27b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAutoReplySettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAutoReplyMessageResponseKind) -> ::windows_core::HRESULT,
    pub SetResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    pub InternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KnownExternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnknownExternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxCapabilities {
    type Vtable = IEmailMailboxCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeedec3a6_89db_4305_82c4_439e0a33da11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanServerSearchFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanServerSearchMailbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanSmartSend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxCapabilities2 {
    type Vtable = IEmailMailboxCapabilities2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69723ee4_2f21_4cbc_88ab_2e7602a4806b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanResolveRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanValidateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanEmptyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanCreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanDeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanMoveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxCapabilities3 {
    type Vtable = IEmailMailboxCapabilities3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf690e944_56f2_45aa_872c_0ce9f3db0b5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanServerSearchFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanServerSearchMailbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanSmartSend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanResolveRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanValidateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanEmptyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanCreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanDeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetCanMoveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxChange {
    type Vtable = IEmailMailboxChange_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61edf54b_11ef_400c_adde_8cde65c85e66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChange_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxChangeType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailboxActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailboxActions: usize,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxChangeReader {
    type Vtable = IEmailMailboxChangeReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdbd0ebb_c53d_4331_97be_be75a2146a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangeReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangeTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxChangeTracker {
    type Vtable = IEmailMailboxChangeTracker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ae48638_5166_42b7_8882_fd21c92bdd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangeTracker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxChangedDeferral {
    type Vtable = IEmailMailboxChangedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x779a74c1_97c5_4b54_b30d_306232623e6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxChangedEventArgs {
    type Vtable = IEmailMailboxChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cfd5f6e_01d4_4e4a_a44c_b22dd42ec207);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxCreateFolderResult {
    type Vtable = IEmailMailboxCreateFolderResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb228557f_2885_4998_b595_8a2d374ce950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxCreateFolderStatus) -> ::windows_core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxPolicies {
    type Vtable = IEmailMailboxPolicies_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f3345c5_1c3b_4dc7_b410_6373783e545d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows_core::HRESULT,
    pub AllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeSigningAlgorithm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxPolicies2 {
    type Vtable = IEmailMailboxPolicies2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbab58afb_a14b_497c_a8e2_55eac29cc4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MustSignSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxPolicies3 {
    type Vtable = IEmailMailboxPolicies3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdd4a01f_4867_414a_81a2_803919c44191);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows_core::HRESULT,
    pub SetAllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeSigningAlgorithm: usize,
    pub SetMustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetMustSignSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxSyncManager {
    type Vtable = IEmailMailboxSyncManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x517ac55a_3591_4b5d_85bc_c71dde862263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMailboxSyncManager2 {
    type Vtable = IEmailMailboxSyncManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd8dc97e_95c1_4f89_81b7_e6aecb6695fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailManagerForUser {
    type Vtable = IEmailManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf773de9f_3ca5_4b0f_90c1_156e40174ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailManagerStatics {
    type Vtable = IEmailManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5128654_55c5_4890_a824_216c2618ce7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailManagerStatics2 {
    type Vtable = IEmailManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac052da3_b194_425d_b6d9_d0f04135eda2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailManagerStatics3 {
    type Vtable = IEmailManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a722395_843e_4945_b3aa_349e07a362c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMeetingInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMeetingInfo {
    type Vtable = IEmailMeetingInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31c03fa9_7933_415f_a275_d165ba07026b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMeetingInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AppointmentRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppointmentRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProposedStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proposedstarttime: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ProposedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub RecurrenceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecurrenceStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecurrenceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecurrenceStartTime: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Recurrence: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub SetRecurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    SetRecurrence: usize,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMeetingInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMeetingInfo2 {
    type Vtable = IEmailMeetingInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e59386d_b0d9_4fe5_867c_e31ed2b588b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMeetingInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReportedOutOfDateByServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessage {
    type Vtable = IEmailMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c6d948d_80b5_48f8_b0b1_e04e430f44e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    To: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CC: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Bcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bcc: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessage2 {
    type Vtable = IEmailMessage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdc8248b_9f1a_44db_bd3c_65c384770f86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowInternetImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowInternetImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageDownloadState) -> ::windows_core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageDownloadState) -> ::windows_core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows_core::HRESULT,
    pub SetFlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailFlagState) -> ::windows_core::HRESULT,
    pub HasPartialBodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows_core::HRESULT,
    pub SetImportance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailImportance) -> ::windows_core::HRESULT,
    pub InResponseToMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IrmInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIrmInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsDraftMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsServerSearchMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSmartSendable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NormalizedSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OriginalCodePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOriginalCodePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows_core::HRESULT,
    pub SetLastResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageResponseKind) -> ::windows_core::HRESULT,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SentTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSentTime: usize,
    pub MeetingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMeetingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBodyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBodyStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBodyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, stream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBodyStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessage3 {
    type Vtable = IEmailMessage3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1ea675c_e598_4d29_a018_fc7b7eece0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SmimeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmimeData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSmimeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSmimeData: usize,
    pub SmimeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageSmimeKind) -> ::windows_core::HRESULT,
    pub SetSmimeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageSmimeKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessage4 {
    type Vtable = IEmailMessage4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x317cf181_3e7f_4a05_8394_3e10336dd435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReplyTo: usize,
    pub SentRepresenting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSentRepresenting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessageBatch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessageBatch {
    type Vtable = IEmailMessageBatch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x605cd08f_25d9_4f1b_9e51_0514c0149653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessageBatch_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Messages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Messages: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessageReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailMessageReader {
    type Vtable = IEmailMessageReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f4abe9f_6213_4a85_a3b0_f92d1a839d19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessageReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailQueryOptions {
    type Vtable = IEmailQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45504b9b_3e7f_4d52_b6dd_d6fd4e1fbd9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TextSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SortDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortDirection) -> ::windows_core::HRESULT,
    pub SetSortDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySortDirection) -> ::windows_core::HRESULT,
    pub SortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortProperty) -> ::windows_core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySortProperty) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQueryKind) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQueryKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FolderIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FolderIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailQueryOptionsFactory {
    type Vtable = IEmailQueryOptionsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88f1a1b8_78ab_4ee8_b4e3_046d6e2fe5e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, fields: EmailQuerySearchFields, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryTextSearch(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailQueryTextSearch {
    type Vtable = IEmailQueryTextSearch_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fa0a288_3c5d_46a5_a6e2_31d6fd17e540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryTextSearch_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchFields) -> ::windows_core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySearchFields) -> ::windows_core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchScope) -> ::windows_core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySearchScope) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipient(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailRecipient {
    type Vtable = IEmailRecipient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcae825b3_4478_4814_b900_c902b5e19b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipient_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailRecipientFactory {
    type Vtable = IEmailRecipientFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5500b84d_c79a_4ef8_b909_722e18e3935d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailRecipientResolutionResult {
    type Vtable = IEmailRecipientResolutionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x918338fa_8d8d_4573_80d1_07172a34b98d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientResolutionResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailRecipientResolutionStatus) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub PublicKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    PublicKeys: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailRecipientResolutionResult2 {
    type Vtable = IEmailRecipientResolutionResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e420bb6_ce5b_4bde_b9d4_e16da0b09fca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientResolutionResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailRecipientResolutionStatus) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub SetPublicKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    SetPublicKeys: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailStore {
    type Vtable = IEmailStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf803226e_9137_4f8b_a470_279ac3058eb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMailboxesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMailboxesAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMailboxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, accountaddress: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, accountaddress: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStoreNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailStoreNotificationTriggerDetails {
    type Vtable = IEmailStoreNotificationTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce17563c_46e6_43c9_96f7_facf7dd710cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}