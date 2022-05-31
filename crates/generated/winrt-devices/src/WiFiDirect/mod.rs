#[cfg(feature = "Services")]
pub mod Services;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub InformationElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InformationElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInformationElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInformationElements: usize,
    pub ListenStateDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::HRESULT,
    pub SetListenStateDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::HRESULT,
    pub IsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub LegacySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisement2 {
    type Vtable = IWiFiDirectAdvertisement2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb759aa46_d816_491b_917a_b40d7dc403a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GroupOwnerIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub SetGroupOwnerIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParameters2 {
    type Vtable = IWiFiDirectConnectionParameters2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab3b0fbe_aa82_44b4_88c8_e3056b89801d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferenceOrderedConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferenceOrderedConfigurationMethods: usize,
    pub PreferredPairingProcedure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectPairingProcedure) -> ::windows_core::HRESULT,
    pub SetPreferredPairingProcedure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectPairingProcedure) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParametersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParametersStatics {
    type Vtable = IWiFiDirectConnectionParametersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x598af493_7642_456f_b9d8_e8a9eb1f401a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub GetDevicePairingKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    GetDevicePairingKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetConnectionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectConnectionStatus) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDeviceStatics {
    type Vtable = IWiFiDirectDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe86cb57c_3aac_4851_a792_482aaf931b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDeviceStatics2 {
    type Vtable = IWiFiDirectDeviceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a953e49_b103_437e_9226_ab67971342f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, connectionparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Oui: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Oui: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetOui: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetOui: usize,
    pub OuiType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetOuiType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectInformationElementStatics {
    type Vtable = IWiFiDirectInformationElementStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbd02f16_11a5_4e60_8caa_34772148378a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub CreateFromDeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinformation: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))]
    CreateFromDeviceInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectLegacySettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub Passphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Passphrase: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPassphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPassphrase: usize,
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisement(::windows_core::IUnknown);
impl WiFiDirectAdvertisement {
    #[cfg(feature = "Foundation_Collections")]
    pub fn InformationElements(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InformationElements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInformationElements<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInformationElements)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ListenStateDiscoverability(&self) -> ::windows_core::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectAdvertisementListenStateDiscoverability>::zeroed();
            (::windows_core::Interface::vtable(this).ListenStateDiscoverability)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectAdvertisementListenStateDiscoverability>(result__)
        }
    }
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListenStateDiscoverability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutonomousGroupOwnerEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAutonomousGroupOwnerEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LegacySettings(&self) -> ::windows_core::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LegacySettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectLegacySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows_core::Interface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedConfigurationMethods)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisement {}
impl ::core::fmt::Debug for WiFiDirectAdvertisement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisement;{ab511a2d-2a06-49a1-a584-61435c7905a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisement {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisement {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Intensive: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementListenStateDiscoverability {}
impl ::core::clone::Clone for WiFiDirectAdvertisementListenStateDiscoverability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectAdvertisementListenStateDiscoverability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectAdvertisementListenStateDiscoverability {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectAdvertisementListenStateDiscoverability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementListenStateDiscoverability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(::windows_core::IUnknown);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectAdvertisementPublisher, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Advertisement(&self) -> ::windows_core::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectAdvertisement>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectAdvertisementPublisherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementPublisher {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher;{b35a2d1a-9b1f-45d9-925a-694d66df68ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisementPublisher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementPublisherStatus {}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectAdvertisementPublisherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectAdvertisementPublisherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectAdvertisementPublisherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    pub fn Error(&self) -> ::windows_core::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectError>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs;{aafde53c-5481-46e6-90dd-32116518f192})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisementPublisherStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: Self = Self(0i32);
    pub const DisplayPin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectConfigurationMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectConfigurationMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectConfigurationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConfigurationMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiDirectConnectionListener(::windows_core::IUnknown);
impl WiFiDirectConnectionListener {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectConnectionListener, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionListener {}
impl ::core::fmt::Debug for WiFiDirectConnectionListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionListener").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionListener;{699c1b0d-8d13-4ee9-b9ec-9c72f8251f7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionListener as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionListener {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionListener {}
#[repr(transparent)]
pub struct WiFiDirectConnectionParameters(::windows_core::IUnknown);
impl WiFiDirectConnectionParameters {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectConnectionParameters, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GroupOwnerIntent(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
            (::windows_core::Interface::vtable(this).GroupOwnerIntent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn SetGroupOwnerIntent(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroupOwnerIntent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreferenceOrderedConfigurationMethods)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
    pub fn PreferredPairingProcedure(&self) -> ::windows_core::Result<WiFiDirectPairingProcedure> {
        let this = &::windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectPairingProcedure>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredPairingProcedure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectPairingProcedure>(result__)
        }
    }
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredPairingProcedure)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> ::windows_core::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Enumeration::DevicePairingKinds>::zeroed();
            (::windows_core::Interface::vtable(this).GetDevicePairingKinds)(::windows_core::Interface::as_raw(this), configurationmethod, result__.as_mut_ptr()).from_abi::<super::Enumeration::DevicePairingKinds>(result__)
        })
    }
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionParameters {}
impl ::core::fmt::Debug for WiFiDirectConnectionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters;{b2e55405-5702-4b16-a02c-bbcd21ef6098})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::core::convert::TryFrom<WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows_core::Error;
    fn try_from(value: WiFiDirectConnectionParameters) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::core::convert::TryFrom<&WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows_core::Error;
    fn try_from(value: &WiFiDirectConnectionParameters) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows_core::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows_core::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows_core::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::core::convert::TryInto::<super::Enumeration::IDevicePairingSettings>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionParameters {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionParameters {}
#[repr(transparent)]
pub struct WiFiDirectConnectionRequest(::windows_core::IUnknown);
impl WiFiDirectConnectionRequest {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionRequest {}
impl ::core::fmt::Debug for WiFiDirectConnectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest;{8eb99605-914f-49c3-a614-d18dc5b19b43})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: WiFiDirectConnectionRequest) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &WiFiDirectConnectionRequest) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequest {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequest {}
#[repr(transparent)]
pub struct WiFiDirectConnectionRequestedEventArgs(::windows_core::IUnknown);
impl WiFiDirectConnectionRequestedEventArgs {
    pub fn GetConnectionRequest(&self) -> ::windows_core::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectConnectionRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs;{f99d20be-d38d-484f-8215-e7b65abf244c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectConnectionStatus {}
impl ::core::clone::Clone for WiFiDirectConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectConnectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiDirectDevice(::windows_core::IUnknown);
impl WiFiDirectDevice {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows_core::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiDirectConnectionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiDirectConnectionStatus>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionEndpointPairs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, WiFiDirectConnectionParameters>>(deviceid: Param0, connectionparameters: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), connectionparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectDevice {}
impl ::core::fmt::Debug for WiFiDirectDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectDevice;{72deaaa8-72eb-4dae-8a28-8513355d2777})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: WiFiDirectDevice) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &WiFiDirectDevice) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectDevice {}
unsafe impl ::core::marker::Sync for WiFiDirectDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: Self = Self(0i32);
    pub const AssociationEndpoint: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectDeviceSelectorType {}
impl ::core::clone::Clone for WiFiDirectDeviceSelectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectDeviceSelectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectDeviceSelectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectDeviceSelectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectDeviceSelectorType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectError {}
impl ::core::clone::Clone for WiFiDirectError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectError {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiDirectInformationElement(::windows_core::IUnknown);
impl WiFiDirectInformationElement {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectInformationElement, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Oui(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Oui)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetOui<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOui)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OuiType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).OuiType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetOuiType(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOuiType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub fn CreateFromDeviceInformation<'a, Param0: ::windows_core::IntoParam<'a, super::Enumeration::DeviceInformation>>(deviceinformation: Param0) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceInformation)(::windows_core::Interface::as_raw(this), deviceinformation.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectInformationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectInformationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectInformationElement {}
impl ::core::fmt::Debug for WiFiDirectInformationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectInformationElement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectInformationElement;{affb72d6-76bb-497e-ac8b-dc72838bc309})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectInformationElement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectInformationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectInformationElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectInformationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectInformationElement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectInformationElement {}
unsafe impl ::core::marker::Sync for WiFiDirectInformationElement {}
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(::windows_core::IUnknown);
impl WiFiDirectLegacySettings {
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
    pub fn Ssid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Ssid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSsid<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSsid)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn Passphrase(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Passphrase)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPassphrase<'a, Param0: ::windows_core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassphrase)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectLegacySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectLegacySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectLegacySettings {}
impl ::core::fmt::Debug for WiFiDirectLegacySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectLegacySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectLegacySettings;{a64fdbba-f2fd-4567-a91b-f5c2f5321057})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiDirectLegacySettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows_core::IUnknown {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows_core::IUnknown {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows_core::IInspectable {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows_core::IInspectable {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectLegacySettings {}
unsafe impl ::core::marker::Sync for WiFiDirectLegacySettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: Self = Self(0i32);
    pub const Invitation: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectPairingProcedure {}
impl ::core::clone::Clone for WiFiDirectPairingProcedure {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectPairingProcedure {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiDirectPairingProcedure {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectPairingProcedure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectPairingProcedure").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}