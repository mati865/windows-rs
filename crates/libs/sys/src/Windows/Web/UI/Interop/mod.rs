#[repr(C)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Core::CoreAcceleratorKeyEventType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    EventType: usize,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::UI::Core::CorePhysicalKeyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub RoutingStage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlAcceleratorKeyRoutingStage) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlAcceleratorKeyPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2007147838, data2: 31860, data3: 17277, data4: [162, 144, 58, 192, 216, 205, 86, 85] };
}
#[repr(C)]
pub struct IWebViewControlMoveFocusRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlMoveFocusReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlMoveFocusRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1797927949, data2: 19408, data3: 16478, data4: [183, 193, 30, 114, 164, 146, 244, 70] };
}
#[repr(C)]
pub struct IWebViewControlProcess {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsPrivateNetworkClientServerCapabilityEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWebViewControlAsync: unsafe extern "system" fn(this: *mut *mut Self, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebViewControlAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWebViewControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWebViewControls: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessExited: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlProcess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 46605292, data2: 39126, data3: 16970, data4: [182, 62, 198, 19, 108, 54, 160, 242] };
}
#[repr(C)]
pub struct IWebViewControlProcessFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut *mut Self, processoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlProcessFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1203133689, data2: 41682, data3: 17724, data4: [176, 151, 246, 119, 157, 75, 142, 2] };
}
#[repr(C)]
pub struct IWebViewControlProcessOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut *mut Self, value: WebViewControlProcessCapabilityState) -> ::windows_sys::core::HRESULT,
    pub PrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlProcessCapabilityState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlProcessOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 483029671, data2: 15318, data3: 18470, data4: [130, 97, 108, 129, 137, 80, 93, 137] };
}
#[repr(C)]
pub struct IWebViewControlSite {
    pub base__: ::windows_sys::core::IInspectable,
    pub Process: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MoveFocus: unsafe extern "system" fn(this: *mut *mut Self, reason: WebViewControlMoveFocusReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoveFocusRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AcceleratorKeyPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceleratorKeyPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAcceleratorKeyPressed: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlSite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322914246, data2: 4828, data3: 18584, data4: [189, 71, 4, 150, 125, 230, 72, 186] };
}
#[repr(C)]
pub struct IWebViewControlSite2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlSite2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3510316607, data2: 18670, data3: 18224, data4: [130, 67, 210, 237, 12, 5, 96, 106] };
}
pub type WebViewControl = *mut ::core::ffi::c_void;
pub type WebViewControlAcceleratorKeyPressedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlAcceleratorKeyRoutingStage(pub i32);
impl WebViewControlAcceleratorKeyRoutingStage {
    pub const Tunneling: Self = Self(0i32);
    pub const Bubbling: Self = Self(1i32);
}
impl ::core::marker::Copy for WebViewControlAcceleratorKeyRoutingStage {}
impl ::core::clone::Clone for WebViewControlAcceleratorKeyRoutingStage {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlMoveFocusReason {}
impl ::core::clone::Clone for WebViewControlMoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebViewControlMoveFocusRequestedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlProcess = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlProcessCapabilityState {}
impl ::core::clone::Clone for WebViewControlProcessCapabilityState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebViewControlProcessOptions = *mut ::core::ffi::c_void;
