#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReader<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReader(riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CreateXmlReader(::core::mem::transmute(riid), ::core::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pinputstream: Param0, pmalloc: Param1, nencodingcodepage: u32, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingCodePage(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr, nencodingcodepage: u32, fencodinghint: ::win32_foundation::BOOL, pwszbaseuri: ::windows_core::PCWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pinputstream: Param0, pmalloc: Param1, pwszencodingname: Param2, fencodinghint: Param3, pwszbaseuri: Param4) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlReaderInputWithEncodingName(pinputstream: *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr, pwszencodingname: ::windows_core::PCWSTR, fencodinghint: ::win32_foundation::BOOL, pwszbaseuri: ::windows_core::PCWSTR, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), fencodinghint.into_param().abi(), pwszbaseuri.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriter<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriter(riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        CreateXmlWriter(::core::mem::transmute(riid), ::core::mem::transmute(ppvobject), pmalloc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>>(poutputstream: Param0, pmalloc: Param1, nencodingcodepage: u32) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream: *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr, nencodingcodepage: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateXmlWriterOutputWithEncodingCodePage(poutputstream.into_param().abi(), pmalloc.into_param().abi(), ::core::mem::transmute(nencodingcodepage), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IMalloc>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(poutputstream: Param0, pmalloc: Param1, pwszencodingname: Param2) -> ::windows_core::Result<::windows_core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateXmlWriterOutputWithEncodingName(poutputstream: *mut ::core::ffi::c_void, pmalloc: ::windows_core::RawPtr, pwszencodingname: ::windows_core::PCWSTR, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.into_param().abi(), pmalloc.into_param().abi(), pwszencodingname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DtdProcessing(pub i32);
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::core::marker::Copy for DtdProcessing {}
impl ::core::clone::Clone for DtdProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtdProcessing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DtdProcessing {
    type Abi = Self;
}
impl ::core::fmt::Debug for DtdProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdProcessing").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IXmlReader(::windows_core::IUnknown);
impl IXmlReader {
    pub unsafe fn SetInput<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pinput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInput)(::windows_core::Interface::as_raw(self), pinput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn Read(&self, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnodetype)))
    }
    pub unsafe fn GetNodeType(&self) -> ::windows_core::Result<XmlNodeType> {
        let mut result__ = ::core::mem::MaybeUninit::<XmlNodeType>::zeroed();
        (::windows_core::Interface::vtable(self).GetNodeType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XmlNodeType>(result__)
    }
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).MoveToFirstAttribute)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).MoveToNextAttribute)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn MoveToAttributeByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).MoveToAttributeByName)(::windows_core::Interface::as_raw(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()))
    }
    pub unsafe fn MoveToElement(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveToElement)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut ::windows_core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetQualifiedName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwszqualifiedname), ::core::mem::transmute(pcwchqualifiedname)).ok()
    }
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut ::windows_core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamespaceUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwsznamespaceuri), ::core::mem::transmute(pcwchnamespaceuri)).ok()
    }
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut ::windows_core::PWSTR, pcwchlocalname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLocalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwszlocalname), ::core::mem::transmute(pcwchlocalname)).ok()
    }
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut ::windows_core::PWSTR, pcwchprefix: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrefix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwszprefix), ::core::mem::transmute(pcwchprefix)).ok()
    }
    pub unsafe fn GetValue(&self, ppwszvalue: *mut ::windows_core::PWSTR, pcwchvalue: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwszvalue), ::core::mem::transmute(pcwchvalue)).ok()
    }
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: &mut [u16], pcwchread: *mut u32) -> ::windows_core::HRESULT {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).ReadValueChunk)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwchbuffer)), pwchbuffer.len() as _, ::core::mem::transmute(pcwchread)))
    }
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut ::windows_core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBaseUri)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppwszbaseuri), ::core::mem::transmute(pcwchbaseuri)).ok()
    }
    pub unsafe fn IsDefault(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsDefault)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn IsEmptyElement(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsEmptyElement)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetLineNumber(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLineNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLinePosition(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLinePosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDepth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn IsEOF(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsEOF)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IXmlReader> for ::windows_core::IUnknown {
    fn from(value: IXmlReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlReader> for ::windows_core::IUnknown {
    fn from(value: &IXmlReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlReader {}
impl ::core::fmt::Debug for IXmlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXmlReader {
    type Vtable = IXmlReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT,
    pub MoveToFirstAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveToNextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveToAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub MoveToElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut ::windows_core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows_core::HRESULT,
    pub GetNamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut ::windows_core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows_core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut ::windows_core::PWSTR, pcwchlocalname: *mut u32) -> ::windows_core::HRESULT,
    pub GetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszprefix: *mut ::windows_core::PWSTR, pcwchprefix: *mut u32) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszvalue: *mut ::windows_core::PWSTR, pcwchvalue: *mut u32) -> ::windows_core::HRESULT,
    pub ReadValueChunk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchbuffer: ::windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows_core::HRESULT,
    pub GetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut ::windows_core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub IsEmptyElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub GetLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows_core::HRESULT,
    pub GetLinePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows_core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows_core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows_core::HRESULT,
    pub IsEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
}
#[repr(transparent)]
pub struct IXmlResolver(::windows_core::IUnknown);
impl IXmlResolver {
    pub unsafe fn ResolveUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszbaseuri: Param0, pwszpublicidentifier: Param1, pwszsystemidentifier: Param2) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ResolveUri)(::windows_core::Interface::as_raw(self), pwszbaseuri.into_param().abi(), pwszpublicidentifier.into_param().abi(), pwszsystemidentifier.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IXmlResolver> for ::windows_core::IUnknown {
    fn from(value: IXmlResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlResolver> for ::windows_core::IUnknown {
    fn from(value: &IXmlResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlResolver {}
impl ::core::fmt::Debug for IXmlResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXmlResolver {
    type Vtable = IXmlResolver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ResolveUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszbaseuri: ::windows_core::PCWSTR, pwszpublicidentifier: ::windows_core::PCWSTR, pwszsystemidentifier: ::windows_core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlWriter(::windows_core::IUnknown);
impl IXmlWriter {
    pub unsafe fn SetOutput<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, poutput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutput)(::windows_core::Interface::as_raw(self), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn WriteAttributes<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteAttributes)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteAttributeString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteAttributeString)(::windows_core::Interface::as_raw(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn WriteCData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteCData)(::windows_core::Interface::as_raw(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteCharEntity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wch)).ok()
    }
    pub unsafe fn WriteChars(&self, pwch: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteChars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    pub unsafe fn WriteComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteComment)(::windows_core::Interface::as_raw(self), pwszcomment.into_param().abi()).ok()
    }
    pub unsafe fn WriteDocType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteDocType)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    pub unsafe fn WriteElementString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2, pwszvalue: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteElementString)(::windows_core::Interface::as_raw(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEndDocument)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WriteEndElement(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEndElement)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEntityRef)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn WriteFullEndElement(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteFullEndElement)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WriteName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn WriteNmToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsznmtoken: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNmToken)(::windows_core::Interface::as_raw(self), pwsznmtoken.into_param().abi()).ok()
    }
    pub unsafe fn WriteNode<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNode)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNodeShallow)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteProcessingInstruction)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteQualifiedName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszlocalname: Param0, pwsznamespaceuri: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteQualifiedName)(::windows_core::Interface::as_raw(self), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn WriteRaw<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteRaw)(::windows_core::Interface::as_raw(self), pwszdata.into_param().abi()).ok()
    }
    pub unsafe fn WriteRawChars(&self, pwch: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteRawChars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStartDocument)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(standalone)).ok()
    }
    pub unsafe fn WriteStartElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprefix: Param0, pwszlocalname: Param1, pwsznamespaceuri: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStartElement)(::windows_core::Interface::as_raw(self), pwszprefix.into_param().abi(), pwszlocalname.into_param().abi(), pwsznamespaceuri.into_param().abi()).ok()
    }
    pub unsafe fn WriteString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteString)(::windows_core::Interface::as_raw(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteSurrogateCharEntity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszwhitespace: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteWhitespace)(::windows_core::Interface::as_raw(self), pwszwhitespace.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IXmlWriter> for ::windows_core::IUnknown {
    fn from(value: IXmlWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlWriter> for ::windows_core::IUnknown {
    fn from(value: &IXmlWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriter {}
impl ::core::fmt::Debug for IXmlWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXmlWriter {
    type Vtable = IXmlWriter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT,
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows_core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszpublicid: ::windows_core::PCWSTR, pwszsystemid: ::windows_core::PCWSTR, pwszsubset: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows_core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows_core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXmlWriterLite(::windows_core::IUnknown);
impl IXmlWriterLite {
    pub unsafe fn SetOutput<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, poutput: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutput)(::windows_core::Interface::as_raw(self), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows_core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::<isize>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nproperty), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn WriteAttributes<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteAttributes)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteAttributeString(&self, pwszqname: &[u16], pwszvalue: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteAttributeString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszqname)), pwszqname.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszvalue)), pwszvalue.len() as _).ok()
    }
    pub unsafe fn WriteCData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteCData)(::windows_core::Interface::as_raw(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteCharEntity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wch)).ok()
    }
    pub unsafe fn WriteChars(&self, pwch: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteChars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    pub unsafe fn WriteComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszcomment: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteComment)(::windows_core::Interface::as_raw(self), pwszcomment.into_param().abi()).ok()
    }
    pub unsafe fn WriteDocType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, pwszpublicid: Param1, pwszsystemid: Param2, pwszsubset: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteDocType)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pwszpublicid.into_param().abi(), pwszsystemid.into_param().abi(), pwszsubset.into_param().abi()).ok()
    }
    pub unsafe fn WriteElementString<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszqname: &[u16], pwszvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteElementString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszqname)), pwszqname.len() as _, pwszvalue.into_param().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEndDocument)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WriteEndElement(&self, pwszqname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEndElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteEntityRef<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteEntityRef)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn WriteFullEndElement(&self, pwszqname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteFullEndElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn WriteNmToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsznmtoken: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNmToken)(::windows_core::Interface::as_raw(self), pwsznmtoken.into_param().abi()).ok()
    }
    pub unsafe fn WriteNode<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNode)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteNodeShallow<'a, Param0: ::windows_core::IntoParam<'a, IXmlReader>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, preader: Param0, fwritedefaultattributes: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteNodeShallow)(::windows_core::Interface::as_raw(self), preader.into_param().abi(), fwritedefaultattributes.into_param().abi()).ok()
    }
    pub unsafe fn WriteProcessingInstruction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, pwsztext: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteProcessingInstruction)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteRaw<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteRaw)(::windows_core::Interface::as_raw(self), pwszdata.into_param().abi()).ok()
    }
    pub unsafe fn WriteRawChars(&self, pwch: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteRawChars)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwch)), pwch.len() as _).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStartDocument)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(standalone)).ok()
    }
    pub unsafe fn WriteStartElement(&self, pwszqname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStartElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pwszqname)), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteString)(::windows_core::Interface::as_raw(self), pwsztext.into_param().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteSurrogateCharEntity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wchlow), ::core::mem::transmute(wchhigh)).ok()
    }
    pub unsafe fn WriteWhitespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszwhitespace: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteWhitespace)(::windows_core::Interface::as_raw(self), pwszwhitespace.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IXmlWriterLite> for ::windows_core::IUnknown {
    fn from(value: IXmlWriterLite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXmlWriterLite> for ::windows_core::IUnknown {
    fn from(value: &IXmlWriterLite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXmlWriterLite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXmlWriterLite {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXmlWriterLite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriterLite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriterLite {}
impl ::core::fmt::Debug for IXmlWriterLite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriterLite").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXmlWriterLite {
    type Vtable = IXmlWriterLite_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT,
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_core::PCWSTR, cwszvalue: u32) -> ::windows_core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows_core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszpublicid: ::windows_core::PCWSTR, pwszsystemid: ::windows_core::PCWSTR, pwszsubset: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: ::windows_core::RawPtr, fwritedefaultattributes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows_core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows_core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlConformanceLevel(pub i32);
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::core::marker::Copy for XmlConformanceLevel {}
impl ::core::clone::Clone for XmlConformanceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlConformanceLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlConformanceLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlConformanceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlConformanceLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlError(pub i32);
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
impl ::core::marker::Copy for XmlError {}
impl ::core::clone::Clone for XmlError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlError {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlError").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlNodeType(pub i32);
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
impl ::core::marker::Copy for XmlNodeType {}
impl ::core::clone::Clone for XmlNodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlNodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlNodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlNodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNodeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlReadState(pub i32);
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::core::marker::Copy for XmlReadState {}
impl ::core::clone::Clone for XmlReadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlReadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReadState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlReaderProperty(pub i32);
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
impl ::core::marker::Copy for XmlReaderProperty {}
impl ::core::clone::Clone for XmlReaderProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReaderProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlReaderProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReaderProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReaderProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlStandalone(pub i32);
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::core::marker::Copy for XmlStandalone {}
impl ::core::clone::Clone for XmlStandalone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlStandalone {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlStandalone {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlStandalone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlStandalone").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XmlWriterProperty(pub i32);
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::core::marker::Copy for XmlWriterProperty {}
impl ::core::clone::Clone for XmlWriterProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlWriterProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XmlWriterProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlWriterProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlWriterProperty").field(&self.0).finish()
    }
}
pub const _IID_IXmlReader: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlResolver: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlWriter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
