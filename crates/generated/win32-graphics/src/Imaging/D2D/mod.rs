#[repr(transparent)]
pub struct IWICImageEncoder(::windows_core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<'a, Param0: ::windows_core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows_core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteFrame)(::windows_core::Interface::as_raw(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<'a, Param0: ::windows_core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows_core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteFrameThumbnail)(::windows_core::Interface::as_raw(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<'a, Param0: ::windows_core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows_core::IntoParam<'a, super::IWICBitmapEncoder>>(&self, pimage: Param0, pencoder: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteThumbnail)(::windows_core::Interface::as_raw(self), pimage.into_param().abi(), pencoder.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
}
impl ::core::convert::From<IWICImageEncoder> for ::windows_core::IUnknown {
    fn from(value: IWICImageEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImageEncoder> for ::windows_core::IUnknown {
    fn from(value: &IWICImageEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWICImageEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWICImageEncoder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICImageEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICImageEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImageEncoder {}
impl ::core::fmt::Debug for IWICImageEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICImageEncoder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWICImageEncoder {
    type Vtable = IWICImageEncoder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImageEncoder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows_core::RawPtr, pframeencode: ::windows_core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows_core::RawPtr, pframeencode: ::windows_core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows_core::RawPtr, pencoder: ::windows_core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
#[repr(transparent)]
pub struct IWICImagingFactory2(::windows_core::IUnknown);
impl IWICImagingFactory2 {
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows_core::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions) -> ::windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDecoderFromFilename)(::windows_core::Interface::as_raw(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows_core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDecoderFromStream)(::windows_core::Interface::as_raw(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows_core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDecoderFromFileHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows_core::GUID) -> ::windows_core::Result<super::IWICComponentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateComponentInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clsidcomponent), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICComponentInfo>(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDecoder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<super::IWICBitmapEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateEncoder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapEncoder>(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows_core::Result<super::IWICPalette> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePalette)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICPalette>(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows_core::Result<super::IWICFormatConverter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateFormatConverter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFormatConverter>(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows_core::Result<super::IWICBitmapScaler> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapScaler)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapScaler>(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows_core::Result<super::IWICBitmapClipper> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapClipper)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapClipper>(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows_core::Result<super::IWICBitmapFlipRotator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFlipRotator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapFlipRotator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows_core::Result<super::IWICStream> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICStream>(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows_core::Result<super::IWICColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateColorContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICColorContext>(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows_core::Result<super::IWICColorTransform> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateColorTransformer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICColorTransform>(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, option: super::WICBitmapCreateCacheOption) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows_core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, option: super::WICBitmapCreateCacheOption) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFromSource)(::windows_core::Interface::as_raw(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows_core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFromSourceRect)(::windows_core::Interface::as_raw(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFromMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), pbbuffer.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbbuffer)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows_core::IntoParam<'a, super::super::Gdi::HBITMAP>, Param1: ::windows_core::IntoParam<'a, super::super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: super::WICBitmapAlphaChannelOption) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFromHBITMAP)(::windows_core::Interface::as_raw(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows_core::IntoParam<'a, ::win32_ui::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows_core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateBitmapFromHICON)(::windows_core::Interface::as_raw(self), hicon.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows_core::Result<::win32_system::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateComponentEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::IEnumUnknown>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows_core::IntoParam<'a, super::IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows_core::Result<super::IWICFastMetadataEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromDecoder)(::windows_core::Interface::as_raw(self), pidecoder.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows_core::IntoParam<'a, super::IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows_core::Result<super::IWICFastMetadataEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromFrameDecode)(::windows_core::Interface::as_raw(self), piframedecoder.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<super::IWICMetadataQueryWriter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateQueryWriter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows_core::IntoParam<'a, super::IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<super::IWICMetadataQueryWriter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateQueryWriterFromReader)(::windows_core::Interface::as_raw(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn CreateImageEncoder<'a, Param0: ::windows_core::IntoParam<'a, super::super::Direct2D::ID2D1Device>>(&self, pd2ddevice: Param0) -> ::windows_core::Result<IWICImageEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateImageEncoder)(::windows_core::Interface::as_raw(self), pd2ddevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWICImageEncoder>(result__)
    }
}
impl ::core::convert::From<IWICImagingFactory2> for ::windows_core::IUnknown {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for ::windows_core::IUnknown {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IWICImagingFactory> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::IWICImagingFactory> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IWICImagingFactory> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::IWICImagingFactory> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWICImagingFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICImagingFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImagingFactory2 {}
impl ::core::fmt::Debug for IWICImagingFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICImagingFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWICImagingFactory2 {
    type Vtable = IWICImagingFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b816b45_1996_4476_b132_de9e247c8af0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2ddevice: ::windows_core::RawPtr, ppwicimageencoder: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
