pub const APOERR_ALREADY_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073919i32);
pub const APOERR_ALREADY_UNLOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073914i32);
pub const APOERR_APO_LOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073910i32);
pub const APOERR_BUFFERS_OVERLAP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073915i32);
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073917i32);
pub const APOERR_INVALID_APO_CLSID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073916i32);
pub const APOERR_INVALID_COEFFCOUNT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073909i32);
pub const APOERR_INVALID_COEFFICIENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073908i32);
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073911i32);
pub const APOERR_INVALID_CURVE_PARAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073907i32);
pub const APOERR_INVALID_INPUTID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073906i32);
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073912i32);
pub const APOERR_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073918i32);
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2005073913i32);
#[repr(C)]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows_core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APOInitBaseStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitBaseStruct").field("cbSize", &self.cbSize).field("clsid", &self.clsid).finish()
    }
}
unsafe impl ::windows_core::Abi for APOInitBaseStruct {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APOInitBaseStruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APOInitBaseStruct>()) == 0 }
    }
}
impl ::core::cmp::Eq for APOInitBaseStruct {}
impl ::core::default::Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pAPOSystemEffectsProperties: self.pAPOSystemEffectsProperties.clone(),
            pReserved: self.pReserved,
            pDeviceCollection: self.pDeviceCollection.clone(),
        }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for APOInitSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects").field("APOInit", &self.APOInit).field("pAPOEndpointProperties", &self.pAPOEndpointProperties).field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties).field("pReserved", &self.pReserved).field("pDeviceCollection", &self.pDeviceCollection).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for APOInitSystemEffects {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APOInitSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_core::GUID,
    pub InitializeForDiscoveryOnly: ::win32_foundation::BOOL,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pAPOSystemEffectsProperties: self.pAPOSystemEffectsProperties.clone(),
            pReserved: self.pReserved,
            pDeviceCollection: self.pDeviceCollection.clone(),
            nSoftwareIoDeviceInCollection: self.nSoftwareIoDeviceInCollection,
            nSoftwareIoConnectorIndex: self.nSoftwareIoConnectorIndex,
            AudioProcessingMode: self.AudioProcessingMode,
            InitializeForDiscoveryOnly: self.InitializeForDiscoveryOnly,
        }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for APOInitSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects2")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties)
            .field("pReserved", &self.pReserved)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for APOInitSystemEffects2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APOInitSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APOInitSystemEffects2 {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub pServiceProvider: ::core::option::Option<::win32_system::Com::IServiceProvider>,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_core::GUID,
    pub InitializeForDiscoveryOnly: ::win32_foundation::BOOL,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pServiceProvider: self.pServiceProvider.clone(),
            pDeviceCollection: self.pDeviceCollection.clone(),
            nSoftwareIoDeviceInCollection: self.nSoftwareIoDeviceInCollection,
            nSoftwareIoConnectorIndex: self.nSoftwareIoConnectorIndex,
            AudioProcessingMode: self.AudioProcessingMode,
            InitializeForDiscoveryOnly: self.InitializeForDiscoveryOnly,
        }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects3")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pServiceProvider", &self.pServiceProvider)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows_core::Abi for APOInitSystemEffects3 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pServiceProvider == other.pServiceProvider && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_BUFFER_FLAGS(pub i32);
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
impl ::core::marker::Copy for APO_BUFFER_FLAGS {}
impl ::core::clone::Clone for APO_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APO_BUFFER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::core::marker::Copy for APO_CONNECTION_BUFFER_TYPE {}
impl ::core::clone::Clone for APO_CONNECTION_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_CONNECTION_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APO_CONNECTION_BUFFER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_CONNECTION_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_CONNECTION_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: ::core::option::Option<IAudioMediaType>,
    pub u32Signature: u32,
}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { Type: self.Type, pBuffer: self.pBuffer, u32MaxFrameCount: self.u32MaxFrameCount, pFormat: self.pFormat.clone(), u32Signature: self.u32Signature }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_DESCRIPTOR").field("Type", &self.Type).field("pBuffer", &self.pBuffer).field("u32MaxFrameCount", &self.u32MaxFrameCount).field("pFormat", &self.pFormat).field("u32Signature", &self.u32Signature).finish()
    }
}
unsafe impl ::windows_core::Abi for APO_CONNECTION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pBuffer == other.pBuffer && self.u32MaxFrameCount == other.u32MaxFrameCount && self.pFormat == other.pFormat && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_DESCRIPTOR {}
impl ::core::default::Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY").field("pBuffer", &self.pBuffer).field("u32ValidFrameCount", &self.u32ValidFrameCount).field("u32BufferFlags", &self.u32BufferFlags).field("u32Signature", &self.u32Signature).finish()
    }
}
unsafe impl ::windows_core::Abi for APO_CONNECTION_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_CONNECTION_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY_V2").field("property", &self.property).field("u64QPCTime", &self.u64QPCTime).finish()
    }
}
unsafe impl ::windows_core::Abi for APO_CONNECTION_PROPERTY_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_CONNECTION_PROPERTY_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_FLAG(pub i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::core::marker::Copy for APO_FLAG {}
impl ::core::clone::Clone for APO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APO_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_LOG_LEVEL(pub i32);
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
impl ::core::marker::Copy for APO_LOG_LEVEL {}
impl ::core::clone::Clone for APO_LOG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_LOG_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APO_LOG_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_LOG_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_LOG_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { r#type: self.r#type, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for APO_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APO_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APO_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APO_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for APO_NOTIFICATION_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APO_NOTIFICATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_NOTIFICATION_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APO_NOTIFICATION_0 {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APO_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { r#type: self.r#type, Anonymous: self.Anonymous.clone() }
    }
}
unsafe impl ::windows_core::Abi for APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows_core::Abi for APO_NOTIFICATION_DESCRIPTOR_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_NOTIFICATION_DESCRIPTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for APO_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for APO_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APO_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows_core::GUID,
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
    pub iidAPOInterfaceList: [::windows_core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APO_REG_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_REG_PROPERTIES")
            .field("clsid", &self.clsid)
            .field("Flags", &self.Flags)
            .field("szFriendlyName", &self.szFriendlyName)
            .field("szCopyrightInfo", &self.szCopyrightInfo)
            .field("u32MajorVersion", &self.u32MajorVersion)
            .field("u32MinorVersion", &self.u32MinorVersion)
            .field("u32MinInputConnections", &self.u32MinInputConnections)
            .field("u32MaxInputConnections", &self.u32MaxInputConnections)
            .field("u32MinOutputConnections", &self.u32MinOutputConnections)
            .field("u32MaxOutputConnections", &self.u32MaxOutputConnections)
            .field("u32MaxInstances", &self.u32MaxInstances)
            .field("u32NumAPOInterfaces", &self.u32NumAPOInterfaces)
            .field("iidAPOInterfaceList", &self.iidAPOInterfaceList)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for APO_REG_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_REG_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_REG_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_REG_PROPERTIES {}
impl ::core::default::Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStore: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { endpoint: self.endpoint.clone(), propertyStore: self.propertyStore.clone(), propertyKey: self.propertyKey }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { endpoint: self.endpoint.clone(), volume: self.volume }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("volume", &self.volume).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.volume == other.volume
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_FLOW_TYPE(pub i32);
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::core::marker::Copy for AUDIO_FLOW_TYPE {}
impl ::core::clone::Clone for AUDIO_FLOW_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_FLOW_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUDIO_FLOW_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_FLOW_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_FLOW_TYPE").field(&self.0).finish()
    }
}
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows_core::GUID,
    pub canSetState: ::win32_foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIO_SYSTEMEFFECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_SYSTEMEFFECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECT {}
impl ::core::default::Default for AUDIO_SYSTEMEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows_core::GUID,
}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone(), propertyStoreContext: self.propertyStoreContext }
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).field("propertyStoreContext", &self.propertyStoreContext).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.propertyStoreContext == other.propertyStoreContext
    }
}
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows_core::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self {
            endpoint: self.endpoint.clone(),
            propertyStoreContext: self.propertyStoreContext,
            propertyStoreType: self.propertyStoreType,
            propertyStore: self.propertyStore.clone(),
            propertyKey: self.propertyKey,
        }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStoreContext", &self.propertyStoreContext).field("propertyStoreType", &self.propertyStoreType).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStoreContext == other.propertyStoreContext && self.propertyStoreType == other.propertyStoreType && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_SYSTEMEFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUDIO_SYSTEMEFFECT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECT_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AudioFXExtensionParams {
    pub AddPageParam: ::win32_foundation::LPARAM,
    pub pwstrEndpointID: ::windows_core::PWSTR,
    pub pFxProperties: ::core::option::Option<::win32_ui::Shell::PropertiesSystem::IPropertyStore>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        Self { AddPageParam: self.AddPageParam, pwstrEndpointID: self.pwstrEndpointID, pFxProperties: self.pFxProperties.clone() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AudioFXExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioFXExtensionParams").field("AddPageParam", &self.AddPageParam).field("pwstrEndpointID", &self.pwstrEndpointID).field("pFxProperties", &self.pFxProperties).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows_core::Abi for AudioFXExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AudioFXExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pwstrEndpointID == other.pwstrEndpointID && self.pFxProperties == other.pFxProperties
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AudioFXExtensionParams {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EAudioConstriction(pub i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::core::marker::Copy for EAudioConstriction {}
impl ::core::clone::Clone for EAudioConstriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAudioConstriction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EAudioConstriction {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAudioConstriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAudioConstriction").field(&self.0).finish()
    }
}
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
#[repr(transparent)]
pub struct IApoAcousticEchoCancellation(::windows_core::IUnknown);
impl IApoAcousticEchoCancellation {}
impl ::core::convert::From<IApoAcousticEchoCancellation> for ::windows_core::IUnknown {
    fn from(value: IApoAcousticEchoCancellation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAcousticEchoCancellation> for ::windows_core::IUnknown {
    fn from(value: &IApoAcousticEchoCancellation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAcousticEchoCancellation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAcousticEchoCancellation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAcousticEchoCancellation {}
impl ::core::fmt::Debug for IApoAcousticEchoCancellation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAcousticEchoCancellation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IApoAcousticEchoCancellation {
    type Vtable = IApoAcousticEchoCancellation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25385759_3236_4101_a943_25693dfb5d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAcousticEchoCancellation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IApoAuxiliaryInputConfiguration(::windows_core::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, pbydata: &[u8], pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAuxiliaryInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputid), pbydata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbydata)), ::core::mem::transmute(pinputconnection)).ok()
    }
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAuxiliaryInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputid)).ok()
    }
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows_core::IntoParam<'a, IAudioMediaType>>(&self, prequestedinputformat: Param0) -> ::windows_core::Result<IAudioMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IsInputFormatSupported)(::windows_core::Interface::as_raw(self), prequestedinputformat.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAudioMediaType>(result__)
    }
}
impl ::core::convert::From<IApoAuxiliaryInputConfiguration> for ::windows_core::IUnknown {
    fn from(value: IApoAuxiliaryInputConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputConfiguration> for ::windows_core::IUnknown {
    fn from(value: &IApoAuxiliaryInputConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAuxiliaryInputConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAuxiliaryInputConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputConfiguration {}
impl ::core::fmt::Debug for IApoAuxiliaryInputConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IApoAuxiliaryInputConfiguration {
    type Vtable = IApoAuxiliaryInputConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ceb0aab_fa19_48ed_a857_87771ae1b768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputConfiguration_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::HRESULT,
    pub RemoveAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows_core::RawPtr, ppsupportedinputformat: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IApoAuxiliaryInputRT(::windows_core::IUnknown);
impl IApoAuxiliaryInputRT {
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
        (::windows_core::Interface::vtable(self).AcceptInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputid), ::core::mem::transmute(pinputconnection))
    }
}
impl ::core::convert::From<IApoAuxiliaryInputRT> for ::windows_core::IUnknown {
    fn from(value: IApoAuxiliaryInputRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputRT> for ::windows_core::IUnknown {
    fn from(value: &IApoAuxiliaryInputRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAuxiliaryInputRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAuxiliaryInputRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputRT {}
impl ::core::fmt::Debug for IApoAuxiliaryInputRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputRT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IApoAuxiliaryInputRT {
    type Vtable = IApoAuxiliaryInputRT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf851809c_c177_49a0_b1b2_b66f017943ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputRT_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AcceptInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY),
}
#[repr(transparent)]
pub struct IAudioDeviceModulesClient(::windows_core::IUnknown);
impl IAudioDeviceModulesClient {
    pub unsafe fn SetAudioDeviceModulesManager<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, paudiodevicemodulesmanager: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAudioDeviceModulesManager)(::windows_core::Interface::as_raw(self), paudiodevicemodulesmanager.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioDeviceModulesClient> for ::windows_core::IUnknown {
    fn from(value: IAudioDeviceModulesClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioDeviceModulesClient> for ::windows_core::IUnknown {
    fn from(value: &IAudioDeviceModulesClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioDeviceModulesClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioDeviceModulesClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioDeviceModulesClient {}
impl ::core::fmt::Debug for IAudioDeviceModulesClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioDeviceModulesClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioDeviceModulesClient {
    type Vtable = IAudioDeviceModulesClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98f37dac_d0b6_49f5_896a_aa4d169a4c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesClient_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetAudioDeviceModulesManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioMediaType(::windows_core::IUnknown);
impl IAudioMediaType {
    pub unsafe fn IsCompressedFormat(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsCompressedFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, IAudioMediaType>>(&self, piaudiotype: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), piaudiotype.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).GetAudioFormat)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetUncompressedAudioFormat(&self) -> ::windows_core::Result<UNCOMPRESSEDAUDIOFORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::<UNCOMPRESSEDAUDIOFORMAT>::zeroed();
        (::windows_core::Interface::vtable(self).GetUncompressedAudioFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UNCOMPRESSEDAUDIOFORMAT>(result__)
    }
}
impl ::core::convert::From<IAudioMediaType> for ::windows_core::IUnknown {
    fn from(value: IAudioMediaType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMediaType> for ::windows_core::IUnknown {
    fn from(value: &IAudioMediaType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioMediaType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioMediaType {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioMediaType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMediaType {}
impl ::core::fmt::Debug for IAudioMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioMediaType {
    type Vtable = IAudioMediaType_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e997f73_b71f_4798_873b_ed7dfcf15b4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaType_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsCompressedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompressed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piaudiotype: ::windows_core::RawPtr, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX,
    pub GetUncompressedAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioProcessingObject(::windows_core::IUnknown);
impl IAudioProcessingObject {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLatency(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        (::windows_core::Interface::vtable(self).GetLatency)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows_core::Result<*mut APO_REG_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut APO_REG_PROPERTIES>::zeroed();
        (::windows_core::Interface::vtable(self).GetRegistrationProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut APO_REG_PROPERTIES>(result__)
    }
    pub unsafe fn Initialize(&self, pbydata: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pbydata.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbydata))).ok()
    }
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows_core::IntoParam<'a, IAudioMediaType>, Param1: ::windows_core::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedinputformat: Param1) -> ::windows_core::Result<IAudioMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IsInputFormatSupported)(::windows_core::Interface::as_raw(self), poppositeformat.into_param().abi(), prequestedinputformat.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAudioMediaType>(result__)
    }
    pub unsafe fn IsOutputFormatSupported<'a, Param0: ::windows_core::IntoParam<'a, IAudioMediaType>, Param1: ::windows_core::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedoutputformat: Param1) -> ::windows_core::Result<IAudioMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).IsOutputFormatSupported)(::windows_core::Interface::as_raw(self), poppositeformat.into_param().abi(), prequestedoutputformat.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAudioMediaType>(result__)
    }
    pub unsafe fn GetInputChannelCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetInputChannelCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObject> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObject> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObject {}
impl ::core::fmt::Debug for IAudioProcessingObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObject {
    type Vtable = IAudioProcessingObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd7f2b29_24d0_4b5c_b177_592c39f9ca10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows_core::HRESULT,
    pub GetRegistrationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: ::windows_core::RawPtr, prequestedinputformat: ::windows_core::RawPtr, ppsupportedinputformat: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: ::windows_core::RawPtr, prequestedoutputformat: ::windows_core::RawPtr, ppsupportedoutputformat: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetInputChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioProcessingObjectConfiguration(::windows_core::IUnknown);
impl IAudioProcessingObjectConfiguration {
    pub unsafe fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockForProcess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections)).ok()
    }
    pub unsafe fn UnlockForProcess(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockForProcess)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IAudioProcessingObjectConfiguration> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectConfiguration> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectConfiguration {}
impl ::core::fmt::Debug for IAudioProcessingObjectConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectConfiguration {
    type Vtable = IAudioProcessingObjectConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e5ed805_aba6_49c3_8f9a_2b8c889c4fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectConfiguration_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioProcessingObjectLoggingService(::windows_core::IUnknown);
impl IAudioProcessingObjectLoggingService {
    pub unsafe fn ApoLog<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, level: APO_LOG_LEVEL, format: Param1) {
        (::windows_core::Interface::vtable(self).ApoLog)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(level), format.into_param().abi())
    }
}
impl ::core::convert::From<IAudioProcessingObjectLoggingService> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectLoggingService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectLoggingService> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectLoggingService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectLoggingService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectLoggingService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectLoggingService {}
impl ::core::fmt::Debug for IAudioProcessingObjectLoggingService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectLoggingService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectLoggingService {
    type Vtable = IAudioProcessingObjectLoggingService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x698f0107_1745_4708_95a5_d84478a62a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectLoggingService_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ApoLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: ::windows_core::PCWSTR),
}
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications(::windows_core::IUnknown);
impl IAudioProcessingObjectNotifications {
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApoNotificationRegistrationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aponotifications), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        (::windows_core::Interface::vtable(self).HandleNotification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aponotification))
    }
}
impl ::core::convert::From<IAudioProcessingObjectNotifications> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectNotifications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectNotifications> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectNotifications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectNotifications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectNotifications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectNotifications {}
impl ::core::fmt::Debug for IAudioProcessingObjectNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectNotifications").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectNotifications {
    type Vtable = IAudioProcessingObjectNotifications_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56b0c76f_02fd_4b21_a52e_9f8219fc86e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectNotifications_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetApoNotificationRegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub HandleNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION),
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    HandleNotification: usize,
}
#[repr(transparent)]
pub struct IAudioProcessingObjectRT(::windows_core::IUnknown);
impl IAudioProcessingObjectRT {
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
        (::windows_core::Interface::vtable(self).APOProcess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections))
    }
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).CalcInputFrames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32outputframecount)))
    }
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).CalcOutputFrames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32inputframecount)))
    }
}
impl ::core::convert::From<IAudioProcessingObjectRT> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRT> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRT {}
impl ::core::fmt::Debug for IAudioProcessingObjectRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRT").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectRT {
    type Vtable = IAudioProcessingObjectRT_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e1d6a6d_ddbc_4e95_a4c7_ad64ba37846c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRT_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub APOProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY),
    pub CalcInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32,
}
#[repr(transparent)]
pub struct IAudioProcessingObjectRTQueueService(::windows_core::IUnknown);
impl IAudioProcessingObjectRTQueueService {
    pub unsafe fn GetRealTimeWorkQueue(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetRealTimeWorkQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObjectRTQueueService> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectRTQueueService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRTQueueService> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectRTQueueService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectRTQueueService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectRTQueueService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRTQueueService {}
impl ::core::fmt::Debug for IAudioProcessingObjectRTQueueService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRTQueueService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectRTQueueService {
    type Vtable = IAudioProcessingObjectRTQueueService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacd65e2f_955b_4b57_b9bf_ac297bb752c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRTQueueService_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRealTimeWorkQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioProcessingObjectVBR(::windows_core::IUnknown);
impl IAudioProcessingObjectVBR {
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).CalcMaxInputFrames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32maxoutputframecount), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).CalcMaxOutputFrames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(u32maxinputframecount), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObjectVBR> for ::windows_core::IUnknown {
    fn from(value: IAudioProcessingObjectVBR) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectVBR> for ::windows_core::IUnknown {
    fn from(value: &IAudioProcessingObjectVBR) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectVBR {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectVBR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectVBR {}
impl ::core::fmt::Debug for IAudioProcessingObjectVBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectVBR").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioProcessingObjectVBR {
    type Vtable = IAudioProcessingObjectVBR_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ba1db8f_78ad_49cd_9591_f79d80a17c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectVBR_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CalcMaxInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows_core::HRESULT,
    pub CalcMaxOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioSystemEffects(::windows_core::IUnknown);
impl IAudioSystemEffects {}
impl ::core::convert::From<IAudioSystemEffects> for ::windows_core::IUnknown {
    fn from(value: IAudioSystemEffects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects> for ::windows_core::IUnknown {
    fn from(value: &IAudioSystemEffects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioSystemEffects {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioSystemEffects {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects {}
impl ::core::fmt::Debug for IAudioSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioSystemEffects {
    type Vtable = IAudioSystemEffects_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fa00f27_add6_499a_8a9d_6b98521fa75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IAudioSystemEffects2(::windows_core::IUnknown);
impl IAudioSystemEffects2 {
    pub unsafe fn GetEffectsList<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows_core::GUID, pceffects: *mut u32, event: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectsList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffects2> for ::windows_core::IUnknown {
    fn from(value: IAudioSystemEffects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects2> for ::windows_core::IUnknown {
    fn from(value: &IAudioSystemEffects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioSystemEffects2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: IAudioSystemEffects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: &IAudioSystemEffects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects> for &'a IAudioSystemEffects2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects2 {}
impl ::core::fmt::Debug for IAudioSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioSystemEffects2 {
    type Vtable = IAudioSystemEffects2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbafe99d2_7436_44ce_9e0e_4d89afbfff56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects2_Vtbl {
    pub base__: IAudioSystemEffects_Vtbl,
    pub GetEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows_core::GUID, pceffects: *mut u32, event: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioSystemEffects3(::windows_core::IUnknown);
impl IAudioSystemEffects3 {
    pub unsafe fn GetEffectsList<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows_core::GUID, pceffects: *mut u32, event: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEffectsList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
    pub unsafe fn GetControllableSystemEffectsList<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetControllableSystemEffectsList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects), event.into_param().abi()).ok()
    }
    pub unsafe fn SetAudioSystemEffectState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, effectid: Param0, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAudioSystemEffectState)(::windows_core::Interface::as_raw(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for ::windows_core::IUnknown {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for ::windows_core::IUnknown {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for IAudioSystemEffects {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for IAudioSystemEffects {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for IAudioSystemEffects2 {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for IAudioSystemEffects2 {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects2> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAudioSystemEffects2> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAudioSystemEffects2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects3 {}
impl ::core::fmt::Debug for IAudioSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioSystemEffects3 {
    type Vtable = IAudioSystemEffects3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc58b31cd_fc6a_4255_bc1f_ad29bb0a4a17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects3_Vtbl {
    pub base__: IAudioSystemEffects2_Vtbl,
    pub GetControllableSystemEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub SetAudioSystemEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: ::windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAudioSystemEffectsCustomFormats(::windows_core::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    pub unsafe fn GetFormatCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, nformat: u32) -> ::windows_core::Result<IAudioMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nformat), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAudioMediaType>(result__)
    }
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatRepresentation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nformat), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IAudioSystemEffectsCustomFormats> for ::windows_core::IUnknown {
    fn from(value: IAudioSystemEffectsCustomFormats) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffectsCustomFormats> for ::windows_core::IUnknown {
    fn from(value: &IAudioSystemEffectsCustomFormats) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffectsCustomFormats {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsCustomFormats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsCustomFormats {}
impl ::core::fmt::Debug for IAudioSystemEffectsCustomFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsCustomFormats").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAudioSystemEffectsCustomFormats {
    type Vtable = IAudioSystemEffectsCustomFormats_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1176e34_bb7f_4f05_bebd_1b18a534e097);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsCustomFormats_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFormatRepresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY = ::win32_ui::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 5u32 };
pub const SID_AudioProcessingObjectLoggingService: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b8008af_09f9_456e_a173_bdb58499bce7);
pub const SID_AudioProcessingObjectRTQueue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x458c1a1f_6899_4c12_99ac_e2e6ac253104);
#[repr(C)]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows_core::GUID,
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
impl ::core::fmt::Debug for UNCOMPRESSEDAUDIOFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNCOMPRESSEDAUDIOFORMAT").field("guidFormatType", &self.guidFormatType).field("dwSamplesPerFrame", &self.dwSamplesPerFrame).field("dwBytesPerSampleContainer", &self.dwBytesPerSampleContainer).field("dwValidBitsPerSample", &self.dwValidBitsPerSample).field("fFramesPerSecond", &self.fFramesPerSecond).field("dwChannelMask", &self.dwChannelMask).finish()
    }
}
unsafe impl ::windows_core::Abi for UNCOMPRESSEDAUDIOFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNCOMPRESSEDAUDIOFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNCOMPRESSEDAUDIOFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::default::Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
