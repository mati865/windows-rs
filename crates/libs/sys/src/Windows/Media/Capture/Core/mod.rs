#[repr(C)]
pub struct IVariablePhotoCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub UsedFrameControllerIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UsedFrameControllerIndex: usize,
    pub CapturedFrameControlValues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVariablePhotoCapturedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3521858652, data2: 6995, data3: 20042, data4: [139, 92, 219, 120, 135, 172, 148, 155] };
}
#[repr(C)]
pub struct IVariablePhotoSequenceCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
impl ::windows_sys::core::Interface for IVariablePhotoSequenceCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3490786589, data2: 798, data3: 16449, data4: [166, 214, 189, 116, 36, 118, 168, 238] };
}
#[repr(C)]
pub struct IVariablePhotoSequenceCapture2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UpdateSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSettingsAsync: usize,
}
impl ::windows_sys::core::Interface for IVariablePhotoSequenceCapture2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4264321724, data2: 20656, data3: 17379, data4: [145, 124, 227, 185, 39, 152, 148, 47] };
}
pub type VariablePhotoCapturedEventArgs = *mut ::core::ffi::c_void;
pub type VariablePhotoSequenceCapture = *mut ::core::ffi::c_void;
