#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2005073919i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows_sys::core::HRESULT = -2005073914i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_APO_LOCKED: ::windows_sys::core::HRESULT = -2005073910i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows_sys::core::HRESULT = -2005073915i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2005073917i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows_sys::core::HRESULT = -2005073916i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows_sys::core::HRESULT = -2005073909i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows_sys::core::HRESULT = -2005073908i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows_sys::core::HRESULT = -2005073911i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows_sys::core::HRESULT = -2005073907i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_INPUTID: ::windows_sys::core::HRESULT = -2005073906i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows_sys::core::HRESULT = -2005073912i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2005073918i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows_sys::core::HRESULT = -2005073913i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: *mut *mut *mut *mut super::IMMDeviceCollection,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: *mut *mut *mut *mut super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pServiceProvider: *mut *mut *mut *mut super::super::super::System::Com::IServiceProvider,
    pub pDeviceCollection: *mut *mut *mut *mut super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_BUFFER_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_VALID: APO_BUFFER_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_CONNECTION_BUFFER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: *mut *mut *mut *mut IAudioMediaType,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_FLAG = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_NONE: APO_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_INPLACE: APO_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_MIXER: APO_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_DEFAULT: APO_FLAG = 14i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_LOG_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type APO_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows_sys::core::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: *mut *mut *mut *mut super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: *mut *mut *mut *mut super::IMMDevice,
    pub propertyStore: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: *mut *mut *mut *mut super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: *mut *mut *mut *mut super::IMMDevice,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type AUDIO_FLOW_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows_sys::core::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: *mut *mut *mut *mut super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: *mut *mut *mut *mut super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type AUDIO_SYSTEMEFFECT_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: ::windows_sys::core::PWSTR,
    pub pFxProperties: *mut *mut *mut *mut super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type EAudioConstriction = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionOff: EAudioConstriction = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction48_16: EAudioConstriction = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction44_16: EAudioConstriction = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction14_14: EAudioConstriction = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionMute: EAudioConstriction = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
pub struct IApoAcousticEchoCancellation {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IApoAcousticEchoCancellation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 624449369, data2: 12854, data3: 16641, data4: [169, 67, 37, 105, 61, 251, 93, 45] };
}
#[repr(C)]
pub struct IApoAuxiliaryInputConfiguration {
    pub base__: ::windows_sys::core::IUnknown,
    pub AddAuxiliaryInput: unsafe extern "system" fn(this: *mut *mut Self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows_sys::core::HRESULT,
    pub RemoveAuxiliaryInput: unsafe extern "system" fn(this: *mut *mut Self, dwinputid: u32) -> ::windows_sys::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IApoAuxiliaryInputConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1290472107, data2: 64025, data3: 18669, data4: [168, 87, 135, 119, 26, 225, 183, 104] };
}
#[repr(C)]
pub struct IApoAuxiliaryInputRT {
    pub base__: ::windows_sys::core::IUnknown,
    pub AcceptInput: unsafe extern "system" fn(this: *mut *mut Self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY),
}
impl ::windows_sys::core::Interface for IApoAuxiliaryInputRT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4166090908, data2: 49527, data3: 18848, data4: [177, 178, 182, 111, 1, 121, 67, 171] };
}
#[repr(C)]
pub struct IAudioDeviceModulesClient {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAudioDeviceModulesManager: unsafe extern "system" fn(this: *mut *mut Self, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioDeviceModulesClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2566094252, data2: 53430, data3: 18933, data4: [137, 106, 170, 77, 22, 154, 76, 72] };
}
#[repr(C)]
pub struct IAudioMediaType {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCompressedFormat: unsafe extern "system" fn(this: *mut *mut Self, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCompressedFormat: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, piaudiotype: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAudioFormat: unsafe extern "system" fn(this: *mut *mut Self) -> *mut super::WAVEFORMATEX,
    pub GetUncompressedAudioFormat: unsafe extern "system" fn(this: *mut *mut Self, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioMediaType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1318682483, data2: 46879, data3: 18328, data4: [135, 59, 237, 125, 252, 241, 91, 77] };
}
#[repr(C)]
pub struct IAudioProcessingObject {
    pub base__: ::windows_sys::core::IUnknown,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut *mut Self, ptime: *mut i64) -> ::windows_sys::core::HRESULT,
    pub GetRegistrationProperties: unsafe extern "system" fn(this: *mut *mut Self, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, cbdatasize: u32, pbydata: *const u8) -> ::windows_sys::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, poppositeformat: *mut ::core::ffi::c_void, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, poppositeformat: *mut ::core::ffi::c_void, prequestedoutputformat: *mut ::core::ffi::c_void, ppsupportedoutputformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetInputChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pu32channelcount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioProcessingObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4252969769, data2: 9424, data3: 19292, data4: [177, 119, 89, 44, 57, 249, 202, 16] };
}
#[repr(C)]
pub struct IAudioProcessingObjectConfiguration {
    pub base__: ::windows_sys::core::IUnknown,
    pub LockForProcess: unsafe extern "system" fn(this: *mut *mut Self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows_sys::core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 241096709, data2: 43942, data3: 18883, data4: [143, 154, 43, 140, 136, 156, 79, 168] };
}
#[repr(C)]
pub struct IAudioProcessingObjectLoggingService {
    pub base__: ::windows_sys::core::IUnknown,
    pub ApoLog: unsafe extern "system" fn(this: *mut *mut Self, level: APO_LOG_LEVEL, format: ::windows_sys::core::PCWSTR),
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectLoggingService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1770979591, data2: 5957, data3: 18184, data4: [149, 165, 216, 68, 120, 166, 42, 101] };
}
#[repr(C)]
pub struct IAudioProcessingObjectNotifications {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetApoNotificationRegistrationInfo: unsafe extern "system" fn(this: *mut *mut Self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub HandleNotification: unsafe extern "system" fn(this: *mut *mut Self, aponotification: *const APO_NOTIFICATION),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    HandleNotification: usize,
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectNotifications {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1454425967, data2: 765, data3: 19233, data4: [165, 46, 159, 130, 25, 252, 134, 228] };
}
#[repr(C)]
pub struct IAudioProcessingObjectRT {
    pub base__: ::windows_sys::core::IUnknown,
    pub APOProcess: unsafe extern "system" fn(this: *mut *mut Self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY),
    pub CalcInputFrames: unsafe extern "system" fn(this: *mut *mut Self, u32outputframecount: u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(this: *mut *mut Self, u32inputframecount: u32) -> u32,
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectRT {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2652727917, data2: 56764, data3: 20117, data4: [164, 199, 173, 100, 186, 55, 132, 108] };
}
#[repr(C)]
pub struct IAudioProcessingObjectRTQueueService {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetRealTimeWorkQueue: unsafe extern "system" fn(this: *mut *mut Self, workqueueid: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectRTQueueService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2899729967, data2: 38235, data3: 19287, data4: [185, 191, 172, 41, 123, 183, 82, 201] };
}
#[repr(C)]
pub struct IAudioProcessingObjectVBR {
    pub base__: ::windows_sys::core::IUnknown,
    pub CalcMaxInputFrames: unsafe extern "system" fn(this: *mut *mut Self, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CalcMaxOutputFrames: unsafe extern "system" fn(this: *mut *mut Self, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioProcessingObjectVBR {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2074205071, data2: 30893, data3: 18893, data4: [149, 145, 247, 157, 128, 161, 124, 129] };
}
#[repr(C)]
pub struct IAudioSystemEffects {
    pub base__: ::windows_sys::core::IUnknown,
}
impl ::windows_sys::core::Interface for IAudioSystemEffects {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1604325159, data2: 44502, data3: 18842, data4: [138, 157, 107, 152, 82, 31, 167, 91] };
}
#[repr(C)]
pub struct IAudioSystemEffects2 {
    pub base__: IAudioSystemEffects,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectsList: unsafe extern "system" fn(this: *mut *mut Self, ppeffectsids: *mut *mut ::windows_sys::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectsList: usize,
}
impl ::windows_sys::core::Interface for IAudioSystemEffects2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3137247698, data2: 29750, data3: 17614, data4: [158, 14, 77, 137, 175, 191, 255, 86] };
}
#[repr(C)]
pub struct IAudioSystemEffects3 {
    pub base__: IAudioSystemEffects2,
    #[cfg(feature = "Win32_Foundation")]
    pub GetControllableSystemEffectsList: unsafe extern "system" fn(this: *mut *mut Self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetControllableSystemEffectsList: usize,
    pub SetAudioSystemEffectState: unsafe extern "system" fn(this: *mut *mut Self, effectid: ::windows_sys::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioSystemEffects3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3314233805, data2: 64618, data3: 16981, data4: [188, 31, 173, 41, 187, 10, 74, 23] };
}
#[repr(C)]
pub struct IAudioSystemEffectsCustomFormats {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetFormatCount: unsafe extern "system" fn(this: *mut *mut Self, pcformats: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, nformat: u32, ppformat: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFormatRepresentation: unsafe extern "system" fn(this: *mut *mut Self, nformat: u32, ppwstrformatrep: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioSystemEffectsCustomFormats {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2971102772, data2: 47999, data3: 20229, data4: [190, 189, 27, 24, 165, 52, 224, 151] };
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 15u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 18u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 17u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 16u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 14u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 20u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 19u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] }, pid: 5u32 };
pub const SID_AudioProcessingObjectLoggingService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2340423855, data2: 2553, data3: 17774, data4: [161, 115, 189, 181, 132, 153, 188, 231] };
pub const SID_AudioProcessingObjectRTQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166809631, data2: 26777, data3: 19474, data4: [153, 172, 226, 230, 172, 37, 49, 4] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows_sys::core::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl ::core::marker::Copy for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::clone::Clone for UNCOMPRESSEDAUDIOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
