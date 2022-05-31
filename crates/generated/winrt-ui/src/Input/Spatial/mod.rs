#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGestureRecognizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71605bcc_0c35_4673_adbd_cc04caa6ef45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RecognitionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RecognitionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecognitionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecognitionEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecognitionEnded: usize,
    #[cfg(feature = "Foundation")]
    pub Tapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tapped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTapped: usize,
    #[cfg(feature = "Foundation")]
    pub HoldStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldStarted: usize,
    #[cfg(feature = "Foundation")]
    pub HoldCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub HoldCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ManipulationCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ManipulationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveManipulationCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveManipulationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarted: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCanceled: usize,
    pub CaptureInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelPendingGestures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrySetGestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialGestureSettings) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGestureRecognizerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialGestureRecognizerFactory {
    type Vtable = ISpatialGestureRecognizerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77214186_57b9_3150_8382_698b24e264d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGestureRecognizerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: SpatialGestureSettings, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialHoldCanceledEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5dfcb667_4caa_4093_8c35_b601a839f31b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldCanceledEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialHoldCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f64470b_4cfd_43da_8dc4_e64552173971);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialHoldStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e343d79_acb6_4144_8615_2cfba8a3cb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialHoldStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteraction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteraction {
    type Vtable = ISpatialInteraction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc967639_88e6_4646_9112_4344aaec9dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteraction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionController {
    type Vtable = ISpatialInteractionController_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f0e5ba3_0954_4e97_86c5_e7f30b114dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasTouchpad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasThumbstick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionController2 {
    type Vtable = ISpatialInteractionController2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35b6d924_c7a2_49b7_b72e_5436b2fb8f9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryGetRenderableModelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryGetRenderableModelAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionController3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionController3 {
    type Vtable = ISpatialInteractionController3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x628466a0_9d91_4a0b_888d_165e670a8cd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionController3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionControllerProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61056fb1_7ba9_4e35_b93f_9272cba9b28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionControllerProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTouchpadTouched: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTouchpadPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsThumbstickPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ThumbstickX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ThumbstickY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub TouchpadX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub TouchpadY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x075878e4_5961_3b41_9dfb_cea5d89cc38a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub Interaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionDetectedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionDetectedEventArgs2 {
    type Vtable = ISpatialInteractionDetectedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b263e93_5f13_419c_97d5_834678266aa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionDetectedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32a64ea8_a15a_3995_b8bd_80513cb5adef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SourceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub SourceLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceLost: usize,
    #[cfg(feature = "Foundation")]
    pub SourceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub SourcePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourcePressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourcePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourcePressed: usize,
    #[cfg(feature = "Foundation")]
    pub SourceReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceReleased: usize,
    #[cfg(feature = "Foundation")]
    pub InteractionDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InteractionDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInteractionDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInteractionDetected: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    pub GetDetectedSourcesAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Perception")))]
    GetDetectedSourcesAtTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionManagerStatics {
    type Vtable = ISpatialInteractionManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00e31fa6_8ca2_30bf_91fe_d9cb4a008990);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionManagerStatics2 {
    type Vtable = ISpatialInteractionManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93f16c52_b88a_5929_8d7c_48cb948b081c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSourceKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SpatialInteractionSourceKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb5433ba_b0b3_3148_9f3b_e9f5de568f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSource2 {
    type Vtable = ISpatialInteractionSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4c5b70c_0470_4028_88c0_a0eb44d34efe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPointingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMenuSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGraspSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception")]
    pub TryGetStateAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    TryGetStateAtTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSource3 {
    type Vtable = ISpatialInteractionSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0406d9f9_9afd_44f9_85dc_700023a962e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handedness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceHandedness) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSource4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSource4 {
    type Vtable = ISpatialInteractionSource4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0073bc4d_df66_5a91_a2ba_cea3e5c58a19);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSource4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Perception_People")]
    pub TryCreateHandMeshObserver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryCreateHandMeshObserver: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))]
    pub TryCreateHandMeshObserverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_People")))]
    TryCreateHandMeshObserverAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23b786cf_ec23_3979_b27c_eb0e12feb7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceEventArgs2 {
    type Vtable = ISpatialInteractionSourceEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b4b467_e648_4d52_ab49_e0d227199f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PressKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionPressKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea4696c4_7e8b_30ca_bcc5_c77189cea30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Velocity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceLocation2 {
    type Vtable = ISpatialInteractionSourceLocation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c671045_3917_40fc_a9ac_31c9cf5ff91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceLocation3 {
    type Vtable = ISpatialInteractionSourceLocation3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6702e65e_e915_4cfb_9c1b_0538efc86687);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceLocation3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub AngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AngularVelocity: usize,
    pub SourcePointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05604542_3ef7_3222_9f53_63c9cb7e3bc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetSourceLossMitigationDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetSourceLossMitigationDirection: usize,
    pub SourceLossRisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetLocation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5c475ef_4b63_37ec_98b9_9fc652b9d2f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceState2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceState2 {
    type Vtable = ISpatialInteractionSourceState2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45f6d0bd_1773_492e_9ba3_8ac1cbe77c08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelectPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMenuPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGrasped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SelectPressedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ControllerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialInteractionSourceState3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialInteractionSourceState3 {
    type Vtable = ISpatialInteractionSourceState3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2f00bc2_bd2b_4a01_a8fb_323e0158527c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionSourceState3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Perception_People")]
    pub TryGetHandPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryGetHandPose: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialManipulationCanceledEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d40d1cb_e7da_4220_b0bf_819301674780);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationCanceledEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialManipulationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05086802_f301_4343_9250_2fbaa5f87a37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialManipulationDelta(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7ec967a_d123_3a81_a15b_992923dcbe91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationDelta_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialManipulationStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1d6bbce_42a5_377b_ada6_d28e3d384737);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialManipulationUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f230b9b_60c6_4dc6_bdc9_9f4a6f15fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialManipulationUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialNavigationCanceledEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce503edc_e8a5_46f0_92d4_3c122b35112a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationCanceledEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialNavigationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x012e80b7_af3b_42c2_9e41_baaa0e721f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialNavigationStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x754a348a_fb64_4656_8ebd_9deecaafe475);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsNavigatingX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsNavigatingY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsNavigatingZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialNavigationUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b713fd7_839d_4a74_8732_45466fc044b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialNavigationUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerInteractionSourcePose(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7104307_2c2b_4d3a_92a7_80ced7c4a0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ForwardDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ForwardDirection: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpDirection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerInteractionSourcePose2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerInteractionSourcePose2 {
    type Vtable = ISpatialPointerInteractionSourcePose2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeccd86b8_52db_469f_9e3f_80c47f74bce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerInteractionSourcePose2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    pub PositionAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourcePositionAccuracy) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerPose(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerPose {
    type Vtable = ISpatialPointerPose_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6953a42e_c17e_357d_97a1_7269d0ed2d10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_People")]
    pub Head: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Head: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerPose2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerPose2 {
    type Vtable = ISpatialPointerPose2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d202b17_954e_4e0c_96d1_b6790b6fc2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetInteractionSourcePose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerPose3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerPose3 {
    type Vtable = ISpatialPointerPose3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6342f3f0_ec49_5b4b_b8d1_d16cbb16be84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPose3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Perception_People")]
    pub Eyes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Eyes: usize,
    pub IsHeadCapturedBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialPointerPoseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialPointerPoseStatics {
    type Vtable = ISpatialPointerPoseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa25591a9_aca1_3ee0_9816_785cfb2e3fb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialPointerPoseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, timestamp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetAtTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialRecognitionEndedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e35f5cb_3f75_43f3_ac81_d1dc2df9b1fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialRecognitionEndedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialRecognitionStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24da128f_0008_4a6d_aa50_2a76f9cfb264);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialRecognitionStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsGesturePossible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gesture: SpatialGestureSettings, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialTappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x296d83de_f444_4aa1_b2bf_9dc88d567da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialTappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialInteractionSourceKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub TapCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SpatialGestureRecognizer(::windows_core::IUnknown);
impl SpatialGestureRecognizer {
    #[cfg(feature = "Foundation")]
    pub fn RecognitionStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RecognitionStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecognitionStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRecognitionStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecognitionEnded<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RecognitionEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecognitionEnded<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRecognitionEnded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Tapped<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Tapped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTapped<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTapped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HoldStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HoldStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHoldStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HoldCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HoldCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHoldCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HoldCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HoldCanceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHoldCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHoldCanceled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ManipulationStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ManipulationUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ManipulationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ManipulationCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationCanceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveManipulationCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveManipulationCanceled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NavigationUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NavigationCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCanceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCanceled<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationCanceled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CaptureInteraction<'a, Param0: ::windows_core::IntoParam<'a, SpatialInteraction>>(&self, interaction: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CaptureInteraction)(::windows_core::Interface::as_raw(this), interaction.into_param().abi()).ok() }
    }
    pub fn CancelPendingGestures(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CancelPendingGestures)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetGestureSettings)(::windows_core::Interface::as_raw(this), settings, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GestureSettings(&self) -> ::windows_core::Result<SpatialGestureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialGestureSettings>::zeroed();
            (::windows_core::Interface::vtable(this).GestureSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialGestureSettings>(result__)
        }
    }
    pub fn Create(settings: SpatialGestureSettings) -> ::windows_core::Result<SpatialGestureRecognizer> {
        Self::ISpatialGestureRecognizerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), settings, result__.as_mut_ptr()).from_abi::<SpatialGestureRecognizer>(result__)
        })
    }
    pub fn ISpatialGestureRecognizerFactory<R, F: FnOnce(&ISpatialGestureRecognizerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialGestureRecognizer, ISpatialGestureRecognizerFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialGestureRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialGestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialGestureRecognizer {}
impl ::core::fmt::Debug for SpatialGestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialGestureRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialGestureRecognizer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialGestureRecognizer;{71605bcc-0c35-4673-adbd-cc04caa6ef45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialGestureRecognizer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialGestureRecognizer";
}
impl ::core::convert::From<SpatialGestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: SpatialGestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGestureRecognizer> for ::windows_core::IUnknown {
    fn from(value: &SpatialGestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialGestureRecognizer> for ::windows_core::IInspectable {
    fn from(value: SpatialGestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGestureRecognizer> for ::windows_core::IInspectable {
    fn from(value: &SpatialGestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialGestureRecognizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialGestureRecognizer {}
unsafe impl ::core::marker::Sync for SpatialGestureRecognizer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialGestureSettings(pub u32);
impl SpatialGestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const ManipulationTranslate: Self = Self(8u32);
    pub const NavigationX: Self = Self(16u32);
    pub const NavigationY: Self = Self(32u32);
    pub const NavigationZ: Self = Self(64u32);
    pub const NavigationRailsX: Self = Self(128u32);
    pub const NavigationRailsY: Self = Self(256u32);
    pub const NavigationRailsZ: Self = Self(512u32);
}
impl ::core::marker::Copy for SpatialGestureSettings {}
impl ::core::clone::Clone for SpatialGestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialGestureSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialGestureSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialGestureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialGestureSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SpatialGestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SpatialGestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SpatialGestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SpatialGestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SpatialGestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialGestureSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialGestureSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialHoldCanceledEventArgs(::windows_core::IUnknown);
impl SpatialHoldCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialHoldCanceledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialHoldCanceledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialHoldCanceledEventArgs {}
impl ::core::fmt::Debug for SpatialHoldCanceledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialHoldCanceledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialHoldCanceledEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs;{5dfcb667-4caa-4093-8c35-b601a839f31b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialHoldCanceledEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
}
impl ::core::convert::From<SpatialHoldCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialHoldCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialHoldCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialHoldCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialHoldCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialHoldCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialHoldCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldCanceledEventArgs {}
#[repr(transparent)]
pub struct SpatialHoldCompletedEventArgs(::windows_core::IUnknown);
impl SpatialHoldCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialHoldCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialHoldCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialHoldCompletedEventArgs {}
impl ::core::fmt::Debug for SpatialHoldCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialHoldCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialHoldCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs;{3f64470b-4cfd-43da-8dc4-e64552173971})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialHoldCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
}
impl ::core::convert::From<SpatialHoldCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialHoldCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialHoldCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialHoldCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialHoldCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialHoldCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialHoldCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldCompletedEventArgs {}
#[repr(transparent)]
pub struct SpatialHoldStartedEventArgs(::windows_core::IUnknown);
impl SpatialHoldStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialHoldStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialHoldStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialHoldStartedEventArgs {}
impl ::core::fmt::Debug for SpatialHoldStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialHoldStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialHoldStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs;{8e343d79-acb6-4144-8615-2cfba8a3cb3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialHoldStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
}
impl ::core::convert::From<SpatialHoldStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialHoldStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialHoldStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialHoldStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialHoldStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialHoldStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialHoldStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialHoldStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialHoldStartedEventArgs {}
#[repr(transparent)]
pub struct SpatialInteraction(::windows_core::IUnknown);
impl SpatialInteraction {
    pub fn SourceState(&self) -> ::windows_core::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteraction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteraction {}
impl ::core::fmt::Debug for SpatialInteraction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteraction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteraction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteraction;{fc967639-88e6-4646-9112-4344aaec9dfa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteraction {
    type Vtable = ISpatialInteraction_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteraction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteraction";
}
impl ::core::convert::From<SpatialInteraction> for ::windows_core::IUnknown {
    fn from(value: SpatialInteraction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteraction> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteraction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteraction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteraction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteraction> for ::windows_core::IInspectable {
    fn from(value: SpatialInteraction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteraction> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteraction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteraction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteraction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteraction {}
unsafe impl ::core::marker::Sync for SpatialInteraction {}
#[repr(transparent)]
pub struct SpatialInteractionController(::windows_core::IUnknown);
impl SpatialInteractionController {
    pub fn HasTouchpad(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasTouchpad)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasThumbstick(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasThumbstick)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<super::super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Devices::Haptics::SimpleHapticsController>(result__)
        }
    }
    pub fn VendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).VendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn ProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn Version(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryGetRenderableModelAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetRenderableModelAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> ::windows_core::Result<super::super::super::Devices::Power::BatteryReport> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionController3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBatteryReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Devices::Power::BatteryReport>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionController {}
impl ::core::fmt::Debug for SpatialInteractionController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionController").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionController {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionController;{5f0e5ba3-0954-4e97-86c5-e7f30b114dfd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionController {
    type Vtable = ISpatialInteractionController_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionController";
}
impl ::core::convert::From<SpatialInteractionController> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionController> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionController> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionController> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionController {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionController {}
unsafe impl ::core::marker::Sync for SpatialInteractionController {}
#[repr(transparent)]
pub struct SpatialInteractionControllerProperties(::windows_core::IUnknown);
impl SpatialInteractionControllerProperties {
    pub fn IsTouchpadTouched(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTouchpadTouched)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTouchpadPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTouchpadPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsThumbstickPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsThumbstickPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ThumbstickX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ThumbstickX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ThumbstickY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ThumbstickY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn TouchpadX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TouchpadX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn TouchpadY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).TouchpadY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionControllerProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionControllerProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionControllerProperties {}
impl ::core::fmt::Debug for SpatialInteractionControllerProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionControllerProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionControllerProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionControllerProperties;{61056fb1-7ba9-4e35-b93f-9272cba9b28b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionControllerProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
}
impl ::core::convert::From<SpatialInteractionControllerProperties> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionControllerProperties> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionControllerProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionControllerProperties> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionControllerProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionControllerProperties> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionControllerProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionControllerProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionControllerProperties {}
unsafe impl ::core::marker::Sync for SpatialInteractionControllerProperties {}
#[repr(transparent)]
pub struct SpatialInteractionDetectedEventArgs(::windows_core::IUnknown);
impl SpatialInteractionDetectedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
    pub fn Interaction(&self) -> ::windows_core::Result<SpatialInteraction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Interaction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteraction>(result__)
        }
    }
    pub fn InteractionSource(&self) -> ::windows_core::Result<SpatialInteractionSource> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSource>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionDetectedEventArgs {}
impl ::core::fmt::Debug for SpatialInteractionDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs;{075878e4-5961-3b41-9dfb-cea5d89cc38a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
}
impl ::core::convert::From<SpatialInteractionDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionDetectedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialInteractionDetectedEventArgs {}
#[repr(transparent)]
pub struct SpatialInteractionManager(::windows_core::IUnknown);
impl SpatialInteractionManager {
    #[cfg(feature = "Foundation")]
    pub fn SourceDetected<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceDetected<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourceLost<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceLost<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourceUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourcePressed<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourcePressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourcePressed<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourcePressed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourceReleased<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceReleased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceReleased<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceReleased)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InteractionDetected<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInteractionDetected<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInteractionDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    pub fn GetDetectedSourcesAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDetectedSourcesAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<SpatialInteractionManager> {
        Self::ISpatialInteractionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionManager>(result__)
        })
    }
    pub fn IsSourceKindSupported(kind: SpatialInteractionSourceKind) -> ::windows_core::Result<bool> {
        Self::ISpatialInteractionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSourceKindSupported)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISpatialInteractionManagerStatics<R, F: FnOnce(&ISpatialInteractionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialInteractionManagerStatics2<R, F: FnOnce(&ISpatialInteractionManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialInteractionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionManager {}
impl ::core::fmt::Debug for SpatialInteractionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionManager;{32a64ea8-a15a-3995-b8bd-80513cb5adef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionManager";
}
impl ::core::convert::From<SpatialInteractionManager> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionManager> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionManager> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionManager> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionManager {}
unsafe impl ::core::marker::Sync for SpatialInteractionManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialInteractionPressKind(pub i32);
impl SpatialInteractionPressKind {
    pub const None: Self = Self(0i32);
    pub const Select: Self = Self(1i32);
    pub const Menu: Self = Self(2i32);
    pub const Grasp: Self = Self(3i32);
    pub const Touchpad: Self = Self(4i32);
    pub const Thumbstick: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialInteractionPressKind {}
impl ::core::clone::Clone for SpatialInteractionPressKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialInteractionPressKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialInteractionPressKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialInteractionPressKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionPressKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionPressKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionPressKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSource(::windows_core::IUnknown);
impl SpatialInteractionSource {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    pub fn IsPointingSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPointingSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMenuSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMenuSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGraspSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGraspSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Controller(&self) -> ::windows_core::Result<SpatialInteractionController> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Controller)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionController>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    pub fn TryGetStateAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows_core::Result<SpatialInteractionSourceState> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetStateAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    pub fn Handedness(&self) -> ::windows_core::Result<SpatialInteractionSourceHandedness> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceHandedness>::zeroed();
            (::windows_core::Interface::vtable(this).Handedness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceHandedness>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn TryCreateHandMeshObserver(&self) -> ::windows_core::Result<super::super::super::Perception::People::HandMeshObserver> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateHandMeshObserver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::People::HandMeshObserver>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Perception_People"))]
    pub fn TryCreateHandMeshObserverAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateHandMeshObserverAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionSource {}
impl ::core::fmt::Debug for SpatialInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSource;{fb5433ba-b0b3-3148-9f3b-e9f5de568f5d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSource";
}
impl ::core::convert::From<SpatialInteractionSource> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSource> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionSource> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSource> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSource {}
unsafe impl ::core::marker::Sync for SpatialInteractionSource {}
#[repr(transparent)]
pub struct SpatialInteractionSourceEventArgs(::windows_core::IUnknown);
impl SpatialInteractionSourceEventArgs {
    pub fn State(&self) -> ::windows_core::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceState>(result__)
        }
    }
    pub fn PressKind(&self) -> ::windows_core::Result<SpatialInteractionPressKind> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionPressKind>::zeroed();
            (::windows_core::Interface::vtable(this).PressKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionPressKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionSourceEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionSourceEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionSourceEventArgs {}
impl ::core::fmt::Debug for SpatialInteractionSourceEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs;{23b786cf-ec23-3979-b27c-eb0e12feb7c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionSourceEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
}
impl ::core::convert::From<SpatialInteractionSourceEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionSourceEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionSourceEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceEventArgs {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceHandedness {}
impl ::core::clone::Clone for SpatialInteractionSourceHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialInteractionSourceHandedness {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialInteractionSourceHandedness {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialInteractionSourceHandedness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceHandedness").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceHandedness {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceHandedness;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: Self = Self(0i32);
    pub const Hand: Self = Self(1i32);
    pub const Voice: Self = Self(2i32);
    pub const Controller: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceKind {}
impl ::core::clone::Clone for SpatialInteractionSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialInteractionSourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialInteractionSourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialInteractionSourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSourceLocation(::windows_core::IUnknown);
impl SpatialInteractionSourceLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Velocity(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Velocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>(result__)
        }
    }
    pub fn PositionAccuracy(&self) -> ::windows_core::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourcePositionAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).PositionAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AngularVelocity(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AngularVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    pub fn SourcePointerPose(&self) -> ::windows_core::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourcePointerPose)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionSourceLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionSourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionSourceLocation {}
impl ::core::fmt::Debug for SpatialInteractionSourceLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceLocation;{ea4696c4-7e8b-30ca-bcc5-c77189cea30a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionSourceLocation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
}
impl ::core::convert::From<SpatialInteractionSourceLocation> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceLocation> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionSourceLocation> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionSourceLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceLocation> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionSourceLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionSourceLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceLocation {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceLocation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialInteractionSourcePositionAccuracy {}
impl ::core::clone::Clone for SpatialInteractionSourcePositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialInteractionSourcePositionAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialInteractionSourcePositionAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialInteractionSourcePositionAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourcePositionAccuracy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourcePositionAccuracy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourcePositionAccuracy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSourceProperties(::windows_core::IUnknown);
impl SpatialInteractionSourceProperties {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetSourceLossMitigationDirection<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetSourceLossMitigationDirection)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    pub fn SourceLossRisk(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SourceLossRisk)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetLocation<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialInteractionSourceLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetLocation)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceLocation>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionSourceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionSourceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionSourceProperties {}
impl ::core::fmt::Debug for SpatialInteractionSourceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceProperties;{05604542-3ef7-3222-9f53-63c9cb7e3bc7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionSourceProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
}
impl ::core::convert::From<SpatialInteractionSourceProperties> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceProperties> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionSourceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionSourceProperties> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionSourceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceProperties> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionSourceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionSourceProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceProperties {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceProperties {}
#[repr(transparent)]
pub struct SpatialInteractionSourceState(::windows_core::IUnknown);
impl SpatialInteractionSourceState {
    pub fn Source(&self) -> ::windows_core::Result<SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSource>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<SpatialInteractionSourceProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceProperties>(result__)
        }
    }
    pub fn IsPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
    pub fn IsSelectPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsMenuPressed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMenuPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsGrasped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrasped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SelectPressedValue(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectPressedValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ControllerProperties(&self) -> ::windows_core::Result<SpatialInteractionControllerProperties> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControllerProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionControllerProperties>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn TryGetHandPose(&self) -> ::windows_core::Result<super::super::super::Perception::People::HandPose> {
        let this = &::windows_core::Interface::cast::<ISpatialInteractionSourceState3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetHandPose)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::People::HandPose>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialInteractionSourceState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialInteractionSourceState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialInteractionSourceState {}
impl ::core::fmt::Debug for SpatialInteractionSourceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialInteractionSourceState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialInteractionSourceState;{d5c475ef-4b63-37ec-98b9-9fc652b9d2f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialInteractionSourceState as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceState";
}
impl ::core::convert::From<SpatialInteractionSourceState> for ::windows_core::IUnknown {
    fn from(value: SpatialInteractionSourceState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceState> for ::windows_core::IUnknown {
    fn from(value: &SpatialInteractionSourceState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialInteractionSourceState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialInteractionSourceState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialInteractionSourceState> for ::windows_core::IInspectable {
    fn from(value: SpatialInteractionSourceState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialInteractionSourceState> for ::windows_core::IInspectable {
    fn from(value: &SpatialInteractionSourceState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialInteractionSourceState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialInteractionSourceState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialInteractionSourceState {}
unsafe impl ::core::marker::Sync for SpatialInteractionSourceState {}
#[repr(transparent)]
pub struct SpatialManipulationCanceledEventArgs(::windows_core::IUnknown);
impl SpatialManipulationCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialManipulationCanceledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialManipulationCanceledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialManipulationCanceledEventArgs {}
impl ::core::fmt::Debug for SpatialManipulationCanceledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialManipulationCanceledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialManipulationCanceledEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs;{2d40d1cb-e7da-4220-b0bf-819301674780})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialManipulationCanceledEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
}
impl ::core::convert::From<SpatialManipulationCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialManipulationCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialManipulationCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialManipulationCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialManipulationCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialManipulationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationCanceledEventArgs {}
#[repr(transparent)]
pub struct SpatialManipulationCompletedEventArgs(::windows_core::IUnknown);
impl SpatialManipulationCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetCumulativeDelta)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialManipulationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialManipulationCompletedEventArgs {}
impl ::core::fmt::Debug for SpatialManipulationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialManipulationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialManipulationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs;{05086802-f301-4343-9250-2fbaa5f87a37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialManipulationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
}
impl ::core::convert::From<SpatialManipulationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialManipulationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialManipulationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationCompletedEventArgs {}
#[repr(transparent)]
pub struct SpatialManipulationDelta(::windows_core::IUnknown);
impl SpatialManipulationDelta {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Translation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Translation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialManipulationDelta {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialManipulationDelta {}
impl ::core::fmt::Debug for SpatialManipulationDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialManipulationDelta").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialManipulationDelta {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationDelta;{a7ec967a-d123-3a81-a15b-992923dcbe91})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialManipulationDelta as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationDelta";
}
impl ::core::convert::From<SpatialManipulationDelta> for ::windows_core::IUnknown {
    fn from(value: SpatialManipulationDelta) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationDelta> for ::windows_core::IUnknown {
    fn from(value: &SpatialManipulationDelta) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialManipulationDelta {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialManipulationDelta {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialManipulationDelta> for ::windows_core::IInspectable {
    fn from(value: SpatialManipulationDelta) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationDelta> for ::windows_core::IInspectable {
    fn from(value: &SpatialManipulationDelta) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialManipulationDelta {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialManipulationDelta {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationDelta {}
unsafe impl ::core::marker::Sync for SpatialManipulationDelta {}
#[repr(transparent)]
pub struct SpatialManipulationStartedEventArgs(::windows_core::IUnknown);
impl SpatialManipulationStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialManipulationStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialManipulationStartedEventArgs {}
impl ::core::fmt::Debug for SpatialManipulationStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialManipulationStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialManipulationStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs;{a1d6bbce-42a5-377b-ada6-d28e3d384737})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialManipulationStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
}
impl ::core::convert::From<SpatialManipulationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialManipulationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialManipulationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationStartedEventArgs {}
#[repr(transparent)]
pub struct SpatialManipulationUpdatedEventArgs(::windows_core::IUnknown);
impl SpatialManipulationUpdatedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetCumulativeDelta<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetCumulativeDelta)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialManipulationDelta>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialManipulationUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialManipulationUpdatedEventArgs {}
impl ::core::fmt::Debug for SpatialManipulationUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialManipulationUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs;{5f230b9b-60c6-4dc6-bdc9-9f4a6f15fe49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialManipulationUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
}
impl ::core::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialManipulationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialManipulationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialManipulationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialManipulationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialManipulationUpdatedEventArgs {}
#[repr(transparent)]
pub struct SpatialNavigationCanceledEventArgs(::windows_core::IUnknown);
impl SpatialNavigationCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialNavigationCanceledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialNavigationCanceledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialNavigationCanceledEventArgs {}
impl ::core::fmt::Debug for SpatialNavigationCanceledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialNavigationCanceledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialNavigationCanceledEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs;{ce503edc-e8a5-46f0-92d4-3c122b35112a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialNavigationCanceledEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
}
impl ::core::convert::From<SpatialNavigationCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialNavigationCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialNavigationCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialNavigationCanceledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationCanceledEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialNavigationCanceledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialNavigationCanceledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationCanceledEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationCanceledEventArgs {}
#[repr(transparent)]
pub struct SpatialNavigationCompletedEventArgs(::windows_core::IUnknown);
impl SpatialNavigationCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NormalizedOffset(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialNavigationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialNavigationCompletedEventArgs {}
impl ::core::fmt::Debug for SpatialNavigationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialNavigationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialNavigationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs;{012e80b7-af3b-42c2-9e41-baaa0e721f3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialNavigationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
}
impl ::core::convert::From<SpatialNavigationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialNavigationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationCompletedEventArgs {}
#[repr(transparent)]
pub struct SpatialNavigationStartedEventArgs(::windows_core::IUnknown);
impl SpatialNavigationStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
    pub fn IsNavigatingX(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNavigatingX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsNavigatingY(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNavigatingY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsNavigatingZ(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsNavigatingZ)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialNavigationStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialNavigationStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialNavigationStartedEventArgs {}
impl ::core::fmt::Debug for SpatialNavigationStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialNavigationStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialNavigationStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs;{754a348a-fb64-4656-8ebd-9deecaafe475})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialNavigationStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
}
impl ::core::convert::From<SpatialNavigationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialNavigationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialNavigationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialNavigationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialNavigationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialNavigationStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationStartedEventArgs {}
#[repr(transparent)]
pub struct SpatialNavigationUpdatedEventArgs(::windows_core::IUnknown);
impl SpatialNavigationUpdatedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NormalizedOffset(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialNavigationUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialNavigationUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialNavigationUpdatedEventArgs {}
impl ::core::fmt::Debug for SpatialNavigationUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialNavigationUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialNavigationUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs;{9b713fd7-839d-4a74-8732-45466fc044b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialNavigationUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
}
impl ::core::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialNavigationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialNavigationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialNavigationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialNavigationUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialNavigationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialNavigationUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialNavigationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialNavigationUpdatedEventArgs {}
#[repr(transparent)]
pub struct SpatialPointerInteractionSourcePose(::windows_core::IUnknown);
impl SpatialPointerInteractionSourcePose {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ForwardDirection(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).ForwardDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpDirection(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).UpDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &::windows_core::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    pub fn PositionAccuracy(&self) -> ::windows_core::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &::windows_core::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourcePositionAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).PositionAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourcePositionAccuracy>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialPointerInteractionSourcePose {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialPointerInteractionSourcePose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialPointerInteractionSourcePose {}
impl ::core::fmt::Debug for SpatialPointerInteractionSourcePose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPointerInteractionSourcePose").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialPointerInteractionSourcePose {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose;{a7104307-2c2b-4d3a-92a7-80ced7c4a0d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialPointerInteractionSourcePose as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
}
impl ::core::convert::From<SpatialPointerInteractionSourcePose> for ::windows_core::IUnknown {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialPointerInteractionSourcePose> for ::windows_core::IUnknown {
    fn from(value: &SpatialPointerInteractionSourcePose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialPointerInteractionSourcePose> for ::windows_core::IInspectable {
    fn from(value: SpatialPointerInteractionSourcePose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialPointerInteractionSourcePose> for ::windows_core::IInspectable {
    fn from(value: &SpatialPointerInteractionSourcePose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialPointerInteractionSourcePose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialPointerInteractionSourcePose {}
unsafe impl ::core::marker::Sync for SpatialPointerInteractionSourcePose {}
#[repr(transparent)]
pub struct SpatialPointerPose(::windows_core::IUnknown);
impl SpatialPointerPose {
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn Head(&self) -> ::windows_core::Result<super::super::super::Perception::People::HeadPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Head)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::People::HeadPose>(result__)
        }
    }
    pub fn TryGetInteractionSourcePose<'a, Param0: ::windows_core::IntoParam<'a, SpatialInteractionSource>>(&self, source: Param0) -> ::windows_core::Result<SpatialPointerInteractionSourcePose> {
        let this = &::windows_core::Interface::cast::<ISpatialPointerPose2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetInteractionSourcePose)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerInteractionSourcePose>(result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn Eyes(&self) -> ::windows_core::Result<super::super::super::Perception::People::EyesPose> {
        let this = &::windows_core::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Eyes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Perception::People::EyesPose>(result__)
        }
    }
    pub fn IsHeadCapturedBySystem(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHeadCapturedBySystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::super::Perception::PerceptionTimestamp>>(coordinatesystem: Param0, timestamp: Param1) -> ::windows_core::Result<SpatialPointerPose> {
        Self::ISpatialPointerPoseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAtTimestamp)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        })
    }
    pub fn ISpatialPointerPoseStatics<R, F: FnOnce(&ISpatialPointerPoseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialPointerPose, ISpatialPointerPoseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialPointerPose {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialPointerPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialPointerPose {}
impl ::core::fmt::Debug for SpatialPointerPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPointerPose").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialPointerPose {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialPointerPose;{6953a42e-c17e-357d-97a1-7269d0ed2d10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialPointerPose {
    type Vtable = ISpatialPointerPose_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialPointerPose as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerPose";
}
impl ::core::convert::From<SpatialPointerPose> for ::windows_core::IUnknown {
    fn from(value: SpatialPointerPose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialPointerPose> for ::windows_core::IUnknown {
    fn from(value: &SpatialPointerPose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialPointerPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialPointerPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialPointerPose> for ::windows_core::IInspectable {
    fn from(value: SpatialPointerPose) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialPointerPose> for ::windows_core::IInspectable {
    fn from(value: &SpatialPointerPose) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialPointerPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialPointerPose {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialPointerPose {}
unsafe impl ::core::marker::Sync for SpatialPointerPose {}
#[repr(transparent)]
pub struct SpatialRecognitionEndedEventArgs(::windows_core::IUnknown);
impl SpatialRecognitionEndedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialRecognitionEndedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialRecognitionEndedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialRecognitionEndedEventArgs {}
impl ::core::fmt::Debug for SpatialRecognitionEndedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialRecognitionEndedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialRecognitionEndedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs;{0e35f5cb-3f75-43f3-ac81-d1dc2df9b1fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialRecognitionEndedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
}
impl ::core::convert::From<SpatialRecognitionEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialRecognitionEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialRecognitionEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialRecognitionEndedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialRecognitionEndedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialRecognitionEndedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialRecognitionEndedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialRecognitionEndedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialRecognitionEndedEventArgs {}
#[repr(transparent)]
pub struct SpatialRecognitionStartedEventArgs(::windows_core::IUnknown);
impl SpatialRecognitionStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
    pub fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGesturePossible)(::windows_core::Interface::as_raw(this), gesture, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialRecognitionStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialRecognitionStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialRecognitionStartedEventArgs {}
impl ::core::fmt::Debug for SpatialRecognitionStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialRecognitionStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialRecognitionStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs;{24da128f-0008-4a6d-aa50-2a76f9cfb264})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialRecognitionStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
}
impl ::core::convert::From<SpatialRecognitionStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialRecognitionStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialRecognitionStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialRecognitionStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialRecognitionStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialRecognitionStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialRecognitionStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialRecognitionStartedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialRecognitionStartedEventArgs {}
#[repr(transparent)]
pub struct SpatialTappedEventArgs(::windows_core::IUnknown);
impl SpatialTappedEventArgs {
    pub fn InteractionSourceKind(&self) -> ::windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialInteractionSourceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSourceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialInteractionSourceKind>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<SpatialPointerPose> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPointerPose)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialPointerPose>(result__)
        }
    }
    pub fn TapCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TapCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialTappedEventArgs {}
impl ::core::fmt::Debug for SpatialTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialTappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialTappedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Spatial.SpatialTappedEventArgs;{296d83de-f444-4aa1-b2bf-9dc88d567da6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialTappedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialTappedEventArgs";
}
impl ::core::convert::From<SpatialTappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialTappedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialTappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialTappedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialTappedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialTappedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialTappedEventArgs {}