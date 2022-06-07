pub const CLSID_SoftwareBitmapNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2229687953, data2: 34306, data3: 19076, data4: [190, 70, 112, 139, 233, 205, 75, 116] };
#[repr(C)]
pub struct ISoftwareBitmapNative {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISoftwareBitmapNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2495382549, data2: 1258, data3: 19246, data4: [175, 19, 77, 233, 90, 168, 152, 235] };
}
#[repr(C)]
pub struct ISoftwareBitmapNativeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub CreateFromWICBitmap: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    CreateFromWICBitmap: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, subtype: *const ::windows_sys::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMF2DBuffer2: usize,
}
impl ::windows_sys::core::Interface for ISoftwareBitmapNativeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3284238828, data2: 10516, data3: 18321, data4: [175, 2, 2, 210, 36, 161, 11, 67] };
}
