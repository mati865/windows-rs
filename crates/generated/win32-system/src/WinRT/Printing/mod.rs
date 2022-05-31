#[repr(transparent)]
pub struct IPrintManagerInterop(::windows_core::IUnknown);
impl IPrintManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPrintManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPrintManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintManagerInterop {}
impl ::core::fmt::Debug for IPrintManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5435a42_8d43_4e7b_a68a_ef311e392087);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[repr(transparent)]
pub struct IPrintWorkflowConfigurationNative(::windows_core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PrinterQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterQueue>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DriverProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UserProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowConfigurationNative> for ::windows_core::IUnknown {
    fn from(value: IPrintWorkflowConfigurationNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowConfigurationNative> for ::windows_core::IUnknown {
    fn from(value: &IPrintWorkflowConfigurationNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintWorkflowConfigurationNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowConfigurationNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowConfigurationNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowConfigurationNative {}
impl ::core::fmt::Debug for IPrintWorkflowConfigurationNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowConfigurationNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc056be0a_9ee2_450a_9823_964f0006f2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(::windows_core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    pub unsafe fn StartXpsOMGeneration<'a, Param0: ::windows_core::IntoParam<'a, IPrintWorkflowXpsReceiver>>(&self, receiver: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartXpsOMGeneration)(::windows_core::Interface::as_raw(self), receiver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ObjectFactory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::IXpsOMObjectFactory1>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowObjectModelSourceFileContentNative> for ::windows_core::IUnknown {
    fn from(value: IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowObjectModelSourceFileContentNative> for ::windows_core::IUnknown {
    fn from(value: &IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintWorkflowObjectModelSourceFileContentNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowObjectModelSourceFileContentNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowObjectModelSourceFileContentNative {}
impl ::core::fmt::Debug for IPrintWorkflowObjectModelSourceFileContentNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowObjectModelSourceFileContentNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68c9e477_993e_4052_8ac6_454eff58db9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiver: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(::windows_core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DocumentPackageTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows_core::IUnknown {
    fn from(value: IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows_core::IUnknown {
    fn from(value: &IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsObjectModelTargetPackageNative {}
impl ::core::fmt::Debug for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsObjectModelTargetPackageNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver(::windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows_core::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDocumentSequencePrintTicket)(::windows_core::Interface::as_raw(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentsequenceuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDocumentSequenceUri)(::windows_core::Interface::as_raw(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows_core::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDocumentData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver> for ::windows_core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver> for ::windows_core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintWorkflowXpsReceiver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04097374_77b8_47f6_8167_aae29d4cf84b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceprintticket: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: ::windows_core::RawPtr, documenturi: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: ::windows_core::RawPtr, pageuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver2(::windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows_core::IntoParam<'a, super::super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDocumentSequencePrintTicket)(::windows_core::Interface::as_raw(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentsequenceuri: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDocumentSequenceUri)(::windows_core::Interface::as_raw(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows_core::IntoParam<'a, super::super::Com::IStream>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDocumentData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(documentid), ::core::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Failed(&self, xpserror: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Failed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(xpserror)).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for ::windows_core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for ::windows_core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrintWorkflowXpsReceiver> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrintWorkflowXpsReceiver> for &'a IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows_core::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver2 {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x023bcc0c_dfab_4a61_b074_490c6995580d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base__: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpserror: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrinting3DManagerInterop(::windows_core::IUnknown);
impl IPrinting3DManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows_core::IUnknown {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows_core::IUnknown {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows_core::IInspectable {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows_core::IInspectable {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPrinting3DManagerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrinting3DManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrinting3DManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinting3DManagerInterop {}
impl ::core::fmt::Debug for IPrinting3DManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinting3DManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ca31010_1484_4587_b26b_dddf9f9caecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}